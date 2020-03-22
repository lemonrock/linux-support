// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Models a PCI device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct PciDevice(PciDeviceAddress);

impl From<PciDeviceAddress> for PciDevice
{
	#[inline(always)]
	fn from(value: PciDeviceAddress) -> Self
	{
		Self(value)
	}
}

impl Into<PciDeviceAddress> for PciDevice
{
	#[inline(always)]
	fn into(self) -> PciDeviceAddress
	{
		self.0
	}
}

impl PciDevice
{
	/// Is this an ethernet device?
	#[inline(always)]
	pub fn is_class_network_ethernet(&self, sys_path: &SysPath) -> bool
	{
		// See: https://pci-ids.ucw.cz/read/PD/
		const Network: u8 = 0x02;
		const EthernetNetwork: u8 = 0x00;

		match self.class_identifier(sys_path)
		{
			(Network, EthernetNetwork, _) => true,
			_ => false,
		}
	}

	/// PCI device's associated NUMA node, if known.
	#[inline(always)]
	pub fn associated_numa_node(&self, sys_path: &SysPath) -> Option<NumaNode>
	{
		let file_path = self.device_file_or_folder_path(sys_path, "numa_node");
		if file_path.exists()
		{
			let value: u8 = file_path.read_value().expect("Could not parse numa_node");
			Some(NumaNode::from(value))
		}
		else
		{
			None
		}
	}

	/// PCI device associated hyper threads.
	///
	/// May report CPUs that don't actually exist; refine list against that known for a NUMA node.
	///
	/// Panics if file unreadable.
	#[inline(always)]
	pub fn associated_hyper_threads(&self, sys_path: &SysPath) -> BTreeSet<HyperThread>
	{
		let file_path = self.device_file_or_folder_path(sys_path, "local_cpulist");

		file_path.read_linux_core_or_numa_list(|value_u16| Ok(HyperThread::from(value_u16))).expect("Could not parse local_cpulist")
	}

	/// Take for use by userspace.
	///
	/// Returns original_linux_pci_userspace_kernel_driver_module_name.
	#[inline(always)]
	pub fn take_for_use_with_userspace(&self, sys_path: &SysPath, linux_pci_userspace_kernel_driver_module: LinuxPciUserspaceKernelDriverModule) -> Option<LinuxKernelModuleName>
	{
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device '{:?}'", self));

		let linux_pci_userspace_kernel_driver_module_name = linux_pci_userspace_kernel_driver_module.linux_kernel_module().linux_kernel_module_name();

		let original_linux_pci_userspace_kernel_driver_module_name = self.unbind_from_driver_if_necessary(sys_path);
		self.write_to_driver_override_file(sys_path, linux_pci_userspace_kernel_driver_module_name.to_str());
		self.bind_to_new_driver(sys_path);
		original_linux_pci_userspace_kernel_driver_module_name
	}

	/// Release from use by userspace.
	#[inline(always)]
	pub fn release_from_use_with_userspace(&self, sys_path: &SysPath, original_linux_pci_userspace_kernel_driver_module_name: Option<LinuxKernelModuleName>)
	{
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device `{:?}`", self));

		self.remove_override_of_pci_kernel_driver(sys_path);
		self.unbind_from_driver_if_necessary(sys_path);
		self.bind_to_original_driver_if_necessary(sys_path, original_linux_pci_userspace_kernel_driver_module_name)
	}
	
	/// PCI vendor identifier and device identifier.
	#[inline(always)]
	pub fn vendor_and_device(&self, sys_path: &SysPath) -> PciVendorAndDevice
	{
		PciVendorAndDevice
		{
			vendor: self.vendor_identifier(sys_path),
			device: self.device_identifier(sys_path),
		}
	}

	/// PCI vendor identifier.
	#[inline(always)]
	fn vendor_identifier(&self, sys_path: &SysPath) -> PciVendorIdentifier
	{
		let file_path = self.device_file_or_folder_path(sys_path, "vendor");
		PciVendorIdentifier::new(file_path.read_hexadecimal_value_with_prefix_u16().expect("Seems PCI device's vendor id does not properly exist")).expect("PCI vendor Id should not be 'Any'")
	}
	
	/// PCI device identifier.
	#[inline(always)]
	fn device_identifier(&self, sys_path: &SysPath) -> PciDeviceIdentifier
	{
		let file_path = self.device_file_or_folder_path(sys_path, "device");
		PciDeviceIdentifier::new(file_path.read_hexadecimal_value_with_prefix_u16().expect("Seems PCI device's device id does not properly exist")).expect("PCI device Id should not be 'Any'")
	}
	
	/// PCI class identifier.
	#[inline(always)]
	pub(crate) fn class_identifier(&self, sys_path: &SysPath) -> (u8, u8, u8)
	{
		let file_path = self.device_file_or_folder_path(sys_path, "class");
		let value = file_path.read_hexadecimal_value_with_prefix(6, |raw_string| u32::from_str_radix(raw_string, 16)).expect("Could not parse class");
		(((value & 0xFF0000) >> 16) as u8, ((value & 0x00FF00) >> 8) as u8, (value & 0x0000FF) as u8)
	}
	
	/// Tries to set the NUMA node of a PCI device.
	///
	/// Very brittle; only really to be used for broken system buses.
	#[allow(unused_must_use)]
	#[inline(always)]
	pub fn set_numa_node_swallowing_errors_as_this_is_brittle(&self, sys_path: &SysPath, numa_node: NumaNode)
	{
		// Strictly speaking, we should read a value of -1 first before attempting to set.
		
		let file_path = self.device_file_or_folder_path(sys_path, "numa_node");
		let x: u8 = numa_node.into();
		file_path.write_value(x);
	}
	
	#[inline(always)]
	fn unbind_from_driver_if_necessary(&self, sys_path: &SysPath) -> Option<LinuxKernelModuleName>
	{
		let unbind_file_path = self.driver_file_or_folder_path(sys_path, "unbind");
		let is_not_bound = !unbind_file_path.exists();
		if is_not_bound
		{
			return None
		}
		
		let original_linux_pci_userspace_kernel_driver_module_name = unbind_file_path.canonicalize().unwrap().parent().unwrap().file_name().unwrap().to_str().unwrap().to_owned();
		
		unbind_file_path.write_value(self.device_address_string()).unwrap();
		
		Some(LinuxKernelModuleName::from(original_linux_pci_userspace_kernel_driver_module_name))
	}
	
	#[inline(always)]
	fn bind_to_new_driver(&self, sys_path: &SysPath)
	{
		let file_path = self.driver_file_or_folder_path(sys_path, "bind");
		file_path.write_value(self.device_address_string()).unwrap()
	}

	#[inline(always)]
	fn device_address_string(&self) -> String
	{
		self.0.into()
	}
	
	#[inline(always)]
	fn remove_override_of_pci_kernel_driver(&self, sys_path: &SysPath)
	{
		self.write_to_driver_override_file(sys_path, "\0")
	}
	
	#[inline(always)]
	fn bind_to_original_driver_if_necessary(&self, sys_path: &SysPath, original_linux_pci_userspace_kernel_driver_module_name: Option<LinuxKernelModuleName>)
	{
		if let Some(original_linux_pci_userspace_kernel_driver_module_name) = original_linux_pci_userspace_kernel_driver_module_name
		{
			let bind_file_path = self.driver_file_or_folder_path(sys_path, "bind");
			bind_file_path.write_value(original_linux_pci_userspace_kernel_driver_module_name.to_str()).unwrap();
		}
	}
	
	#[inline(always)]
	fn write_to_driver_override_file(&self, sys_path: &SysPath, value: &str)
	{
		let file_path = self.device_file_or_folder_path(sys_path, "driver_override");
		file_path.write_value(value).unwrap()
	}
	
	#[inline(always)]
	fn driver_file_or_folder_path(&self, sys_path: &SysPath, file_or_folder_name: &str) -> PathBuf
	{
		let mut path = self.device_file_or_folder_path(sys_path, "driver");
		path.push(file_or_folder_name);
		path
	}
	
	#[inline(always)]
	fn device_file_or_folder_path(&self, sys_path: &SysPath, file_or_folder_name: &str) -> PathBuf
	{
		self.0.pci_device_file_path(sys_path, file_or_folder_name)
	}
}

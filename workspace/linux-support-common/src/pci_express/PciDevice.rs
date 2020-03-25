// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Models a PCI device.
///
/// Some device files are not parsed (looking at linux's `drivers/pci/pci-sysfs.c` source):-
///
/// * `of_node`: This is a file that only exists for architectures where Open Firmware (part of the Device Tree) is supported. Read-only.
/// * `label`
/// * `modalias`
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
	/// Rescans all PCI buses and devices.
	#[inline(always)]
	pub fn rescan_all_pci_buses_and_devices(sys_path: &SysPath) -> io::Result<()>
	{
		sys_path.pci_bus_file_path("rescan").write_value(1)
	}

	/// Tries to set the NUMA node of a PCI device.
	///
	/// Very brittle; only really to be used for broken system buses.
	#[allow(unused_must_use)]
	#[inline(always)]
	pub fn set_numa_node_swallowing_errors_as_this_is_brittle(&self, sys_path: &SysPath, numa_node: NumaNode)
	{
		// Strictly speaking, we should read a value of -1 first before attempting to set.
		self.numa_node_file_path(sys_path).write_value(numa_node);
	}

	/// This value does not exist if the Kernel does not support ACPI.
	///
	/// Panics if file exists but a write error occurs; does nothing if file does not exist.
	pub fn write_d3cold_allowed(&self, sys_path: &SysPath, allowed: bool)
	{
		let file_path = self.d3cold_allowed_file_path(sys_path);
		if likely!(file_path.exists())
		{
			file_path.write_value(allowed).expect("Could not write d3cold_allowed")
		}
	}

	/// Take for use by userspace.
	///
	/// Returns original_linux_pci_userspace_kernel_driver_module_name.
	#[inline(always)]
	pub fn take_for_use_with_userspace(&self, sys_path: &SysPath, linux_pci_userspace_kernel_driver_module: LinuxPciUserspaceKernelDriverModule) -> Option<LinuxKernelModuleName>
	{
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device '{:?}'", self));

		let original_linux_pci_userspace_kernel_driver_module_name = self.unbind_from_driver_if_necessary(sys_path);
		self.write_to_driver_override_file(sys_path, linux_pci_userspace_kernel_driver_module.linux_kernel_module_name());
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
	
	/// Memory map resource.
	#[inline(always)]
	pub fn memory_map_resource(&self, sys_path: &SysPath, resource_index: u8) -> Result<PciResource, io::Error>
	{
		let resource_file_path = self.device_file_or_folder_path(sys_path, &format!("resource{:?}", resource_index));

		let file = OpenOptions::new().read(true).write(true).open(&resource_file_path)?;

		let size =
		{
			let metadata = resource_file_path.metadata()?;
			if !metadata.is_file()
			{
				return Err(io::Error::from(ErrorKind::Other))
			}
			metadata.len() as usize
		};

		let result = unsafe { mmap(null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, file.as_raw_fd(), 0) };
		if unlikely!(result == MAP_FAILED)
		{
			return Err(io::Error::last_os_error())
		}

		Ok
		(
			PciResource
			{
				pointer: unsafe { NonNull::new_unchecked(result as *mut u8) },
				size,
			}
		)
	}

//	#[inline(always)]
//	pub fn enable_bus_mastering_for_direct_memory_access(&self, _sys_path: &SysPath)
//	{
//	// TODO: How does the 'config' file work?
//	https://www.mjmwired.net/kernel/Documentation/ABI/testing/sysfs-bus-pci
//	https://www.kernel.org/doc/html/v4.12/driver-api/uio-howto.html
//	https://raw.githubusercontent.com/pciutils/pciutils/master/pci.ids - Look up Intel IDs!
//	https://www.kernel.org/doc/Documentation/
//	}

	/// Details.
	#[inline(always)]
	pub fn details(&self, sys_path: &SysPath) -> PciDeviceDetails
	{
		PciDeviceDetails
		{
			vendor_and_device: self.vendor_and_device(sys_path),
			subsystem_vendor_and_subsystem_device: self.subsystem_vendor_and_subsystem_device(sys_path),
			class: self.class(sys_path),
			revision: self.revision(sys_path),
			associated_numa_node: self.associated_numa_node(sys_path),
			associated_hyper_threads: self.associated_hyper_threads(sys_path),
			permitted_hyper_threads: self.permitted_hyper_threads(sys_path),
			d3cold_allowed: self.d3cold_allowed(sys_path),
			interrupt_request_line: self.interrupt_request_line(sys_path),
			current_link_speed_and_width: self.current_link_speed_and_width(sys_path),
			maximum_link_speed_and_width: self.maximum_link_speed_and_width(sys_path),
		}
	}

	#[inline(always)]
	fn vendor_and_device(&self, sys_path: &SysPath) -> PciVendorAndDevice
	{
		PciVendorAndDevice
		{
			vendor: self.new_from_file(sys_path, "vendor", PciVendorIdentifier::new),
			device: self.new_from_file(sys_path, "device", PciDeviceIdentifier::new),
		}
	}

	#[inline(always)]
	fn subsystem_vendor_and_subsystem_device(&self, sys_path: &SysPath) -> PciVendorAndDevice
	{
		PciVendorAndDevice
		{
			vendor: self.new_from_file(sys_path, "subsystem_vendor", PciVendorIdentifier::new),
			device: self.new_from_file(sys_path, "subsystem_device", PciDeviceIdentifier::new),
		}
	}

	#[inline(always)]
	fn class(&self, sys_path: &SysPath) -> Option<PciDeviceClass>
	{
		let u24 = self.driver_file_or_folder_path(sys_path, "class").read_hexadecimal_value_with_prefix::<u32>(6).expect("Could not parse PCI class identifier");

		PciDeviceClass::parse(u24)
	}

	#[inline(always)]
	fn revision(&self, sys_path: &SysPath) -> u8
	{
		self.driver_file_or_folder_path(sys_path, "revision").read_hexadecimal_value_with_prefix::<u8>(2).expect("Could not parse PCI revision")
	}

	/// PCI device's associated NUMA node.
	///
	/// May not be present.
	#[inline(always)]
	fn associated_numa_node(&self, sys_path: &SysPath) -> Option<NumaNode>
	{
		let numa_node_file_path = self.numa_node_file_path(sys_path);

		if numa_node_file_path.exists()
		{
			Some(numa_node_file_path.read_value::<u8>().map(NumaNode::from).expect("Could not parse numa_node"))
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
	/// On a test machine, with one hyper thread, reports that hyper threads 0 through 31 were assocated.
	///
	/// Panics if file unreadable.
	#[inline(always)]
	fn associated_hyper_threads(&self, sys_path: &SysPath) -> BTreeSet<HyperThread>
	{
		self.driver_file_or_folder_path(sys_path, "local_cpulist").read_linux_core_or_numa_list(|value_u16| Ok(HyperThread::from(value_u16))).expect("Could not parse local_cpulist")
	}

	/// PCI device hyper threads that are permitted to use this device.
	///
	/// May report CPUs that don't actually exist; refine list against that known for a NUMA node.
	///
	/// ***Even more useless than `associated_hyper_threads()`; on a test machine, with one hyper thread, which  reports that hyper threads 0 through 31 were assocated in `associated_hyper_threads()`, reported hyper threads 0 through 2^31 - 1 were permitted!
	///
	/// Panics if file unreadable.
	#[inline(always)]
	fn permitted_hyper_threads(&self, sys_path: &SysPath) -> u32
	{
		self.driver_file_or_folder_path(sys_path, "local_cpus").parse_linux_core_or_numa_bitmask().expect("Could not parse local_cpulist")
	}

	/// This value does not exist if the Kernel does not support ACPI.
	#[inline(always)]
	fn d3cold_allowed(&self, sys_path: &SysPath) -> Option<bool>
	{
		let file_path = self.d3cold_allowed_file_path(sys_path);
		if likely!(file_path.exists())
		{
			Some(file_path.read_zero_or_one_bool().unwrap())
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn interrupt_request_line(&self, sys_path: &SysPath) -> u8
	{
		self.driver_file_or_folder_path(sys_path, "irq").read_value().expect("Could not parse irq")
	}

	#[inline(always)]
	fn current_link_speed_and_width(&self, sys_path: &SysPath) -> LinkSpeedAndWidth
	{
		LinkSpeedAndWidth
		{
			speed: self.driver_file_or_folder_path(sys_path, "current_link_speed").read_value().unwrap(),
			width: self.driver_file_or_folder_path(sys_path, "current_link_width").read_value().unwrap(),
		}
	}

	#[inline(always)]
	fn maximum_link_speed_and_width(&self, sys_path: &SysPath) -> LinkSpeedAndWidth
	{
		LinkSpeedAndWidth
		{
			speed: self.driver_file_or_folder_path(sys_path, "max_link_speed").read_value().unwrap(),
			width: self.driver_file_or_folder_path(sys_path, "max_link_width").read_value().unwrap(),
		}
	}

	#[inline(always)]
	fn new_from_file<P: Sized>(&self, sys_path: &SysPath, file_name: &str, constructor: impl FnOnce(u16) -> Option<P>) -> P
	{
		let file_path = self.driver_file_or_folder_path(sys_path, file_name);

		let identifier = match file_path.read_hexadecimal_value_with_prefix::<u16>(4)
		{
			Ok(value) => value,
			Err(error) => panic!("PCI {:?} identifier is missing or invalid: {:?}", file_name, error),
		};

		match constructor(identifier)
		{
			Some(identifier) => identifier,
			None => panic!("PCI {:?} identifier is Any"),
		}
	}

	#[inline(always)]
	fn unbind_from_driver_if_necessary(&self, sys_path: &SysPath) -> Option<LinuxKernelModuleName>
	{
		let unbind_file_path = self.unbind_file_path(sys_path);
		let is_not_bound = !unbind_file_path.exists();
		if is_not_bound
		{
			return None
		}
		
		let original_linux_pci_userspace_kernel_driver_module_name = unbind_file_path.canonicalize().unwrap().parent().unwrap().file_name().unwrap().to_str().unwrap().to_owned();
		
		unbind_file_path.write_value(&self.0).unwrap();
		
		Some(LinuxKernelModuleName::from(original_linux_pci_userspace_kernel_driver_module_name))
	}
	
	#[inline(always)]
	fn bind_to_new_driver(&self, sys_path: &SysPath)
	{
		self.bind_file_path(sys_path).write_value( &self.0).unwrap()
	}
	
	#[inline(always)]
	fn bind_to_original_driver_if_necessary(&self, sys_path: &SysPath, original_linux_pci_userspace_kernel_driver_module_name: Option<LinuxKernelModuleName>)
	{
		if let Some(ref original_linux_pci_userspace_kernel_driver_module_name) = original_linux_pci_userspace_kernel_driver_module_name
		{
			self.bind_file_path(sys_path).write_value(original_linux_pci_userspace_kernel_driver_module_name).unwrap();
		}
	}

	#[inline(always)]
	fn remove_override_of_pci_kernel_driver(&self, sys_path: &SysPath)
	{
		self.write_to_driver_override_file(sys_path, false)
	}
	
	#[inline(always)]
	fn write_to_driver_override_file<'a>(&self, sys_path: &SysPath, value: impl IntoLineFeedTerminatedByteString<'a>)
	{
		self.driver_override_file_path(sys_path).write_value(value).unwrap()
	}

	#[inline(always)]
	fn d3cold_allowed_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.driver_file_or_folder_path(sys_path, "d3cold_allowed")
	}

	#[inline(always)]
	fn numa_node_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.driver_file_or_folder_path(sys_path, "numa_node")
	}

	#[inline(always)]
	fn unbind_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.driver_file_or_folder_path(sys_path, "unbind")
	}

	#[inline(always)]
	fn bind_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.driver_file_or_folder_path(sys_path, "bind")
	}

	#[inline(always)]
	fn driver_override_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.driver_file_or_folder_path(sys_path, "driver_override")
	}
	
	#[inline(always)]
	fn driver_file_or_folder_path(&self, sys_path: &SysPath, file_or_folder_name: &str) -> PathBuf
	{
		self.device_file_or_folder_path(sys_path, "driver").append(file_or_folder_name)
	}
	
	#[inline(always)]
	fn device_file_or_folder_path(&self, sys_path: &SysPath, file_or_folder_name: &str) -> PathBuf
	{
		self.0.pci_device_file_path(sys_path, file_or_folder_name)
	}
}

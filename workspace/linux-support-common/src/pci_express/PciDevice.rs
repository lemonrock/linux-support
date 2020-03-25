// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// TODO: Find how many `resourceN` files there are.
// TODO: Find PciBus.
/// Models a PCI device.
///
/// The following files are not parsed or used as they are not properly documented and seem to be of very limited value:-
///
/// * `consistent_dma_mask_bits` (Used on x86-64 only, but present on other architectures
/// * `dma_mask_bits`
/// * `broken_parity_status`
/// * `modalias`
///
/// Bridges also have the files:-
///
/// * `secondary_bus_number`
/// * `subordinate_bus_number`
///
/// Bridges do not have files called `resource<N>`.
///
/// PCI Bridges have a `pci_bus` folder which contains the files `rescan`, `cpuaffinity` and `cpulistaffinity`; the same folder can accessed via `/sys/devices/pci0000:00/pci_bus`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciDevice<'a>
{
	pci_device_address: PciDeviceAddress,
	sys_path: &'a SysPath,
	cached_device_file_or_folder_path: PathBuf,
}

impl<'a> Into<PciDeviceAddress> for PciDevice<'a>
{
	#[inline(always)]
	fn into(self) -> PciDeviceAddress
	{
		self.pci_device_address
	}
}

impl<'a> PciDevice<'a>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(pci_device_address: PciDeviceAddress, sys_path: &'a SysPath) -> Self
	{
		Self
		{
			pci_device_address,
			sys_path,
			cached_device_file_or_folder_path: sys_path.pci_device_folder_path(pci_device_address),
		}
	}

	/// Rescans all PCI buses and devices.
	#[inline(always)]
	pub fn rescan_all_pci_buses_and_devices(sys_path: &SysPath) -> io::Result<()>
	{
		sys_path.pci_bus_file_path("rescan").write_value(1)
	}

	/// Is this a PCI bus?
	#[inline(always)]
	pub fn is_pci_bus(&self) -> bool
	{
		self.pci_bus_folder_path().exists()
	}

	/// Resources
	#[inline(always)]
	pub fn resources(&self) -> Resources
	{
		Resources::parse_lines(self).expect("No resources")
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
	pub fn details(&self) -> PciDeviceDetails
	{
		PciDeviceDetails
		{
			vendor_and_device: self.vendor_and_device(),
			subsystem_vendor_and_subsystem_device: self.subsystem_vendor_and_subsystem_device(),
			class: self.class(),
			revision: self.revision(),
			associated_numa_node: self.associated_numa_node(),
			associated_hyper_threads: self.associated_hyper_threads(),
			permitted_hyper_threads_bitmask: self.permitted_hyper_threads_bitmask(),
			d3cold_allowed: self.d3cold_allowed(),
			interrupt_request_line: self.interrupt_request_line(),
			current_link_speed_and_width: self.current_link_speed_and_width(),
			maximum_link_speed_and_width: self.maximum_link_speed_and_width(),
			enabled: self.enable(),
			msi_and_msi_x_enabled: self.msi_and_msi_x_enabled(),
			driver: self.current_pci_driver(),
		}
	}

	/// Tries to set the NUMA node of a PCI device.
	///
	/// Very brittle; only really to be used for broken system buses.
	#[allow(unused_must_use)]
	#[inline(always)]
	pub fn set_numa_node_swallowing_errors_as_this_is_brittle(&self, numa_node: NumaNode)
	{
		// Strictly speaking, we should read a value of -1 first before attempting to set.
		self.numa_node_file_path().write_value(numa_node);
	}

	/// This value does not exist if the Kernel does not support ACPI.
	///
	/// Panics if file exists but a write error occurs; does nothing if file does not exist.
	pub fn write_d3cold_allowed(&self, allowed: bool)
	{
		let file_path = self.d3cold_allowed_file_path();
		if likely!(file_path.exists())
		{
			file_path.write_value(allowed).expect("Could not write d3cold_allowed")
		}
	}

	/// Panics if a write error occurs.
	pub fn reset(&self)
	{
		self.device_file_or_folder_path("reset").write_value(true).expect("Could not write reset")
	}

	/// Panics if a write error occurs.
	pub fn remove(&self)
	{
		self.device_file_or_folder_path("remove").write_value(true).expect("Could not write remove")
	}

	/// Panics if a write error occurs.
	pub fn write_enable(&self, enable: bool)
	{
		self.enable_file_path().write_value(enable).expect("Could not write enable")
	}

	/// Panics if a write error occurs.
	pub fn write_msi_bus(&self, enable: bool)
	{
		self.enable_file_path().write_value(enable).expect("Could not write msi_bus")
	}

	/// Not all devices have a driver.
	#[inline(always)]
	pub fn current_pci_driver(&self) -> Option<PciDriverName>
	{
		let real_path = self.device_file_or_folder_path("driver").canonicalize().ok()?;
		let file_name = real_path.file_name().unwrap();
		Some(PciDriverName::from(DriverName::from(file_name)))
	}

	/// Take for use by userspace.
	///
	/// Returns original PCI driver name.
	#[inline(always)]
	pub fn take_for_use_with_userspace(&self, userspace_pci_driver_name: &PciDriverName) -> Option<PciDriverName>
	{
		let original_pci_driver_name = self.current_pci_driver();
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device '{:?}'", self));

		self.write_to_driver_override_file(userspace_pci_driver_name);

		userspace_pci_driver_name.bind(self.sys_path, self.pci_device_address);

		original_pci_driver_name
	}

	/// Release from use by userspace.
	#[inline(always)]
	pub fn release_from_use_with_userspace(&self, original_pci_driver_name: Option<PciDriverName>)
	{
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device `{:?}`", self));

		self.write_to_driver_override_file(false);

		if let Some(userspace_pci_driver_name) = self.current_pci_driver()
		{
			userspace_pci_driver_name.unbind(self.sys_path, self.pci_device_address);
		}

		if let Some(original_pci_driver_name) = original_pci_driver_name
		{
			original_pci_driver_name.bind(self.sys_path, self.pci_device_address)
		}
	}

	#[inline(always)]
	fn vendor_and_device(&self) -> PciVendorAndDevice
	{
		PciVendorAndDevice
		{
			vendor: self.new_from_file("vendor", PciVendorIdentifier::new),
			device: self.new_from_file("device", PciDeviceIdentifier::new),
		}
	}

	#[inline(always)]
	fn subsystem_vendor_and_subsystem_device(&self) -> PciVendorAndDevice
	{
		PciVendorAndDevice
		{
			vendor: self.new_from_file("subsystem_vendor", PciVendorIdentifier::new),
			device: self.new_from_file("subsystem_device", PciDeviceIdentifier::new),
		}
	}

	#[inline(always)]
	fn class(&self) -> Option<PciDeviceClass>
	{
		let u24 = self.device_file_or_folder_path("class").read_hexadecimal_value_with_prefix::<u32>(6).expect("Could not parse PCI class identifier");

		PciDeviceClass::parse(u24)
	}

	#[inline(always)]
	fn revision(&self) -> u8
	{
		self.device_file_or_folder_path("revision").read_hexadecimal_value_with_prefix::<u8>(2).expect("Could not parse PCI revision")
	}

	/// PCI device's associated NUMA node.
	///
	/// May not be present.
	#[inline(always)]
	fn associated_numa_node(&self) -> Option<NumaNode>
	{
		let numa_node_file_path = self.numa_node_file_path();

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
	fn associated_hyper_threads(&self) -> BTreeSet<HyperThread>
	{
		self.device_file_or_folder_path("local_cpulist").read_linux_core_or_numa_list(|value_u16| Ok(HyperThread::from(value_u16))).expect("Could not parse local_cpulist")
	}

	/// PCI device hyper threads that are permitted to use this device.
	///
	/// May report CPUs that don't actually exist; refine list against that known for a NUMA node.
	///
	/// ***Even more useless than `associated_hyper_threads()`; on a test machine, with one hyper thread, which  reports that hyper threads 0 through 31 were assocated in `associated_hyper_threads()`, reported hyper threads 0 through 2^31 - 1 were permitted!
	///
	/// Panics if file unreadable.
	#[inline(always)]
	fn permitted_hyper_threads_bitmask(&self) -> u32
	{
		self.device_file_or_folder_path("local_cpus").parse_linux_core_or_numa_bitmask().expect("Could not parse local_cpulist")
	}

	#[inline(always)]
	fn msi_and_msi_x_enabled(&self) -> bool
	{
		self.msi_bus_file_path().read_zero_or_one_bool().unwrap()
	}

	#[inline(always)]
	fn enable(&self) -> bool
	{
		self.enable_file_path().read_zero_or_one_bool().unwrap()
	}

	/// This value does not exist if the Kernel does not support ACPI.
	#[inline(always)]
	fn d3cold_allowed(&self) -> Option<bool>
	{
		let file_path = self.d3cold_allowed_file_path();
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
	fn interrupt_request_line(&self) -> u8
	{
		self.device_file_or_folder_path("irq").read_value().expect("Could not parse irq")
	}

	/// PCI express only.
	#[inline(always)]
	fn current_link_speed_and_width(&self) -> Option<LinkSpeedAndWidth>
	{
		let speed = self.device_file_or_folder_path("current_link_speed");
		if !speed.exists()
		{
			return None
		}

		Some
		(
			LinkSpeedAndWidth
			{
				speed: speed.read_value().unwrap(),
				width: self.device_file_or_folder_path("current_link_width").read_value().unwrap(),
			}
		)
	}

	/// PCI express only.
	#[inline(always)]
	fn maximum_link_speed_and_width(&self) -> Option<LinkSpeedAndWidth>
	{
		let speed = self.device_file_or_folder_path("max_link_speed");
		if !speed.exists()
		{
			return None
		}

		Some
		(
			LinkSpeedAndWidth
			{
				speed: speed.read_value().unwrap(),
				width: self.device_file_or_folder_path("max_link_width").read_value().unwrap(),
			}
		)
	}

	#[inline(always)]
	fn new_from_file<P: Sized>(&self, file_name: &str, constructor: impl FnOnce(u16) -> Option<P>) -> P
	{
		let file_path = self.device_file_or_folder_path(file_name);

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
	fn write_to_driver_override_file<'b>(&self, value: impl IntoLineFeedTerminatedByteString<'b>)
	{
		self.driver_override_file_path().write_value(value).unwrap()
	}

	#[inline(always)]
	fn msi_bus_file_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("msi_bus")
	}

	#[inline(always)]
	fn enable_file_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("enable")
	}

	#[inline(always)]
	fn d3cold_allowed_file_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("d3cold_allowed")
	}

	#[inline(always)]
	fn numa_node_file_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("numa_node")
	}

	#[inline(always)]
	fn driver_override_file_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("driver_override")
	}

	/// Often not present.
	#[allow(dead_code)]
	#[inline(always)]
	fn msi_irqs_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("msi_irqs")
	}

	#[allow(dead_code)]
	#[inline(always)]
	fn power_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("power")
	}

	#[allow(dead_code)]
	#[inline(always)]
	fn subsystem_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("subsystem")
	}

	/// Only exists if this device is a bus.
	#[inline(always)]
	fn pci_bus_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("pci_bus")
	}

	#[inline(always)]
	fn device_file_or_folder_path(&self, file_name: &str) -> PathBuf
	{
		self.cached_device_file_or_folder_path.clone().append(file_name)
	}
}

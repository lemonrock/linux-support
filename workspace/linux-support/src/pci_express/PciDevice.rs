// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Models a PCI device.
///
/// The following files are not parsed or used as they seem to be of very limited value:-
///
/// * `consistent_dma_mask_bits` (Used on x86_64 only, but present on other architectures).
/// * `dma_mask_bits`.
/// * `broken_parity_status`.
/// * `modalias` (a formatted string containing vendor, device, subsystem vendor, subsystem device and class).
/// * `link` (always empty on my Parallels VMs).
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
		self.address()
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
	
	/// Address.
	#[inline(always)]
	pub fn address(&self) -> PciDeviceAddress
	{
		self.pci_device_address
	}
	
	/// Parent Bus address.
	#[inline(always)]
	pub fn parent_bus_address(&self) -> PciBusAddress
	{
		self.pci_device_address.bus_address
	}
	
	/// Resources.
	#[inline(always)]
	pub fn resources(&self) -> io::Result<Resources>
	{
		Resources::parse_lines(self)
	}

	/// Configuration space.
	#[inline(always)]
	pub fn configuration_space(&self) -> Result<Option<MemoryMappedConfigurationSpace>, io::Error>
	{
		if let Some(config_file_path) = self.config_file_path()
		{
			let page_size_or_huge_page_size_settings = PageSizeOrHugePageSizeSettings::for_current_page_size();
			let memory_mapped_file = config_file_path.memory_map_read_write(0, AddressHint::any(), Sharing::Private, false, false, &page_size_or_huge_page_size_settings)?;
			
			Ok(Some(MemoryMappedConfigurationSpace(memory_mapped_file)))
		}
		else
		{
			Ok(None)
		}
	}
	
	/// PCI buses.
	#[inline(always)]
	pub fn child_pci_buses(&self) -> io::Result<HashMap<PciBusAddress, io::Result<PciBusDetails>>>
	{
		let read_dir = self.pci_bus_folder_path().read_dir()?;
		let mut pci_buses = HashMap::new();
		for dir_entry in read_dir
		{
			if let Ok(dir_entry) = dir_entry
			{
				if let Ok(file_type) = dir_entry.file_type()
				{
					if !file_type.is_dir()
					{
						continue
					}
					
					if let Some(pci_bus_address) = PciBusAddress::parse_pci_bus(&dir_entry)
					{
						let details = match pci_bus_address.bus(self.cached_device_file_or_folder_path.clone())
						{
							Err(error) => Err(error),
							Ok(bus) => bus.details()
						};
						
						pci_buses.insert(pci_bus_address, details);
					}
				}
			}
		}
		Ok(pci_buses)
	}
	
	/// Read ROM.
	#[inline(always)]
	pub fn read_rom(&self) -> Option<io::Result<Box<[u8]>>>
	{
		assert_effective_user_id_is_root("Read PCI rom");
		
		self.rom_file_path().map(|file_path| file_path.read_raw())
	}
	
	/// Write ROM.
	#[inline(always)]
	pub fn write_rom(&self, contents: Box<[u8]>) -> Option<io::Result<()>>
	{
		assert_effective_user_id_is_root("Write PCI rom");
		
		self.rom_file_path().map(|file_path| file_path.write_value(contents))
	}
	
	/// Details.
	#[inline(always)]
	pub fn details(&self) -> PciDeviceDetails
	{
		PciDeviceDetails
		{
			vendor_and_device: self.vendor_and_device().unwrap(),
			driver: self.current_pci_driver(),
			directly_associated_network_devices: self.directly_associated_network_devices(),
			subsystem_name: self.subsystem_name(),
			subsystem_vendor_and_subsystem_device: self.subsystem_vendor_and_subsystem_device().unwrap(),
			class: self.class().unwrap(),
			revision: self.revision().unwrap(),
			associated_numa_node: self.associated_numa_node(),
			associated_hyper_threads_bit_set: self.associated_hyper_threads_bit_set(),
			associated_hyper_threads_bitmask: self.associated_hyper_threads_bitmask(),
			d3cold_allowed: self.d3cold_allowed(),
			current_link_speed_and_width: self.current_link_speed_and_width(),
			maximum_link_speed_and_width: self.maximum_link_speed_and_width(),
			enabled: self.enable().unwrap(),
			interrupt_request: self.interrupt_request().unwrap(),
			msi_and_msi_x_enabled: self.msi_and_msi_x_enabled().unwrap(),
			msi_and_msi_x_interrupt_requests: self.msi_and_msi_x_interrupt_requests(),
			alternative_routing_identifier_interpretation_forwarding_enabled: self.ari_forwarding_enabled(),
			resource_files: self.resource_files().unwrap(),
			has_rom: self.has_rom(),
			has_config: self.has_config(),
			boot_vga: self.boot_vga(),
			bridge: self.bridge_details(),
		}
	}
	
	#[inline(always)]
	fn resource_files(&self) -> io::Result<Vec<ResourceFile>>
	{
		let read_dir = self.cached_device_file_or_folder_path.read_dir()?;
		
		let mut parsed_resource_files = HashMap::new();
		
		for dir_entry in read_dir
		{
			if let Ok(dir_entry) = dir_entry
			{
				if let Ok(file_type) = dir_entry.file_type()
				{
					if !file_type.is_file()
					{
						continue
					}
					
					let file_name_bytes = dir_entry.file_name().into_vec();
					const Prefix: &'static [u8] = b"resource";
					if !file_name_bytes.starts_with(Prefix)
					{
						continue
					}
					
					let suffix = &file_name_bytes[Prefix.len() .. ];
					let suffix_length = suffix.len();
					
					// There is also a file called 'resource' which is unrelated.
					if suffix_length == 0
					{
						continue
					}
					
					const WriteCombiningSuffix: &'static [u8] = b"_wc";
					let (without_write_combining_suffix, has_write_combining_suffix) = if suffix_length > WriteCombiningSuffix.len()
					{
						let length_less_write_combining_suffix = suffix_length - WriteCombiningSuffix.len();
						let has_write_combining_suffix = &suffix[length_less_write_combining_suffix .. ] == WriteCombiningSuffix;
						if has_write_combining_suffix
						{
							(&suffix[.. length_less_write_combining_suffix], true)
						}
						else
						{
							(suffix, false)
						}
					}
					else
					{
						(suffix, false)
					};
					
					if let Ok(index) = i32::from_bytes(without_write_combining_suffix)
					{
						let index = index as u32;
						
						use self::FastSecureHashMapEntry::*;
						match parsed_resource_files.entry(index)
						{
							Occupied(mut occupied) => if has_write_combining_suffix
							{
								*occupied.get_mut() = true
							},
							
							Vacant(vacant) =>
							{
								vacant.insert(has_write_combining_suffix);
							},
						}
					}
				}
			}
		}
		
		let mut resource_files = Vec::with_capacity(parsed_resource_files.len());
		for (index, write_combining) in parsed_resource_files
		{
			resource_files.push(ResourceFile { index, write_combining })
		}
		Ok(resource_files)
	}
	
	/// Tries to set the NUMA node of a PCI device.
	///
	/// Very brittle; only really to be used for broken system buses.
	#[allow(unused_must_use)]
	#[inline(always)]
	pub fn set_numa_node_swallowing_errors_as_this_is_brittle(&self, numa_node: NumaNode)
	{
		let numa_node: u16 = numa_node.into();
		
		// Strictly speaking, we should read a value of -1 first before attempting to set.
		self.numa_node_file_path().write_value(UnpaddedDecimalInteger(numa_node));
	}

	/// Validated associated hyper threads, even for a non-NUMA machine.
	pub fn validated_associated_hyper_threads(&self, all_pci_buses: &HashMap<PciBusAddress, io::Result<PciBusDetails>>) -> HyperThreads
	{
		let parent_bus_details = all_pci_buses.get(&self.parent_bus_address()).unwrap().as_ref().unwrap();
		debug_assert_eq!(parent_bus_details.associated_hyper_threads_bit_set, parent_bus_details.associated_hyper_threads_bitmask);
		let parent_bus_associated_hyper_threads = &parent_bus_details.associated_hyper_threads_bit_set;
		
		if let Some(Some(numa_node)) = self.associated_numa_node()
		{
			if let Some(associated_hyper_threads) = numa_node.associated_hyper_threads(self.sys_path)
			{
				return self.validate_associated_hyper_threads(associated_hyper_threads, parent_bus_associated_hyper_threads)
			}
		}
		
		if let Some(associated_hyper_threads) = self.associated_hyper_threads_bit_set()
		{
			if cfg!(debug_assertions)
			{
				let associated_hyper_threads_bitmask = self.associated_hyper_threads_bitmask();
				debug_assert_eq!(associated_hyper_threads_bitmask, Some(associated_hyper_threads.clone()))
			}
			
			return self.validate_associated_hyper_threads(associated_hyper_threads, parent_bus_associated_hyper_threads)
		}
		
		self.validate_associated_hyper_threads(HyperThreads::valid(self.sys_path, None), parent_bus_associated_hyper_threads)
	}
	
	#[inline(always)]
	fn validate_associated_hyper_threads(&self, associated_hyper_threads: HyperThreads, parent_bus_associated_hyper_threads: &HyperThreads) -> HyperThreads
	{
		let mut validated = associated_hyper_threads.validate(self.sys_path, None);
		validated.intersection(parent_bus_associated_hyper_threads);
		validated
	}
	
	/// This value does not exist if the Kernel does not support ACPI.
	///
	/// Panics if file exists but a write error occurs; does nothing if file does not exist.
	#[inline(always)]
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
	fn subsystem_name(&self) -> Box<[u8]>
	{
		let folder_path = self.subsystem_symlink_folder_path();
		
		let link = folder_path.read_link().unwrap();
		let file_name = link.file_name().unwrap();
		file_name.as_bytes().to_vec().into_boxed_slice()
	}

	/// Vendor and device.
	#[inline(always)]
	pub(crate) fn vendor_and_device(&self) -> Option<PciVendorAndDevice>
	{
		self.vendor_and_device_like("vendor", "device")
	}
	
	#[inline(always)]
	fn subsystem_vendor_and_subsystem_device(&self) -> Option<PciVendorAndDevice>
	{
		self.vendor_and_device_like("subsystem_vendor", "subsystem_device")
	}
	
	#[inline(always)]
	fn vendor_and_device_like(&self, vendor_file_name: &str, device_file_name: &str) -> Option<PciVendorAndDevice>
	{
		let vendor = self.read_value_if_exists(vendor_file_name);
		let device = self.read_value_if_exists(device_file_name);
		
		match (vendor, device)
		{
			(None, None) => None,
			
			(Some(vendor), Some(device)) => Some
			(
				PciVendorAndDevice
				{
					vendor,
					device,
				}
			),
			
			(Some(_), _) => panic!("{} but not {}", vendor_file_name, device_file_name),
			
			(_, Some(_)) => panic!("{} but not {}", device_file_name, vendor_file_name),
		}
	}
	
	#[inline(always)]
	fn ari_forwarding_enabled(&self) -> bool
	{
		self.device_file_or_folder_path("ari_enabled").read_zero_or_one_bool().unwrap()
	}
	
	#[inline(always)]
	fn boot_vga(&self) -> bool
	{
		self.device_file_or_folder_path("boot_vga").read_zero_or_one_bool().unwrap()
	}
	
	#[inline(always)]
	fn bridge_details(&self) -> Option<PciBridgeDeviceDetails>
	{
		let secondary_bus_number = self.read_value_if_exists("secondary_bus_number");
		let subordinate_bus_number = self.read_value_if_exists("subordinate_bus_number");
		
		match (secondary_bus_number, subordinate_bus_number)
		{
			(None, None) => None,
			
			(Some(secondary_bus_number), Some(subordinate_bus_number)) => Some
			(
				PciBridgeDeviceDetails
				{
					secondary_bus_number,
					subordinate_bus_number,
				}
			),
			
			(Some(_), _) => panic!("secondary_bus_number vendor but not subordinate_bus_number"),
			
			(_, Some(_)) => panic!("subordinate_bus_number but not secondary_bus_number"),
		}
	}
	
	/// This reports nothing for `virtio` as these sit under, say, a `virtio0` child folder.
	#[inline(always)]
	fn directly_associated_network_devices(&self) -> Option<HashSet<NetworkInterfaceName>>
	{
		let folder_path = self.device_file_or_folder_path("net");
		if !folder_path.exists()
		{
			return None
		}
		
		match folder_path.read_dir()
		{
			Err(_) => None,
			
			Ok(read_dir) =>
			{
				let mut network_interface_names = HashSet::new();
				
				for dir_entry in read_dir
				{
					if let Ok(dir_entry) = dir_entry
					{
						if let Ok(file_type) = dir_entry.file_type()
						{
							if !file_type.is_dir()
							{
								continue
							}
							
							let file_name = dir_entry.file_name();
							if let Ok(network_interface_name) = NetworkInterfaceName::from_bytes(file_name.as_bytes())
							{
								network_interface_names.insert(network_interface_name);
							}
						}
					}
				}
				
				Some(network_interface_names)
			}
		}
	}

	#[inline(always)]
	fn class(&self) -> Option<EitherPciDeviceClass>
	{
		self.read_value_if_exists("class")
	}
	
	#[inline(always)]
	fn revision(&self) -> Option<Revision>
	{
		self.read_value_if_exists("revision")
	}
	
	#[inline(always)]
	fn has_rom(&self) -> bool
	{
		self.rom_file_path().is_some()
	}
	
	#[inline(always)]
	fn has_config(&self) -> bool
	{
		self.config_file_path().is_some()
	}
	
	/// PCI device's associated NUMA node.
	///
	/// May not be present even if this is a NUMA machine (eg because this is a virtio device).
	/// Can also be `-1` as the driver doesn't support a NUMA node.
	///
	/// So:-
	///
	/// * `None` - no NUMA node (probably not a NUMA machine).
	/// * `Some(None)` - NUMA node unknown or unreported by the network device driver.
	/// * `Some(Some())` - NUMA node known and reported.
	#[inline(always)]
	pub(crate) fn associated_numa_node(&self) -> Option<Option<NumaNode>>
	{
		let numa_node_file_path = self.numa_node_file_path();

		if numa_node_file_path.exists()
		{
			let value: i32 = numa_node_file_path.read_value().unwrap();
			if value == -1
			{
				Some(None)
			}
			else if value >= 0
			{
				Some(Some(NumaNode::try_from(value).unwrap()))
			}
			else
			{
				panic!("Negative NUMA node value")
			}
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
	///
	/// Returns `None` if file absent (this does not necessarily mean this is not a NUMA machine; virtio devices do not have this information).
	#[inline(always)]
	fn associated_hyper_threads_bit_set(&self) -> Option<HyperThreads>
	{
		let file_path = self.device_file_or_folder_path("local_cpulist");
		if likely!(file_path.exists())
		{
			Some(HyperThreads(file_path.read_hyper_thread_or_numa_node_list().expect("Could not parse local_cpulist")))
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
	/// Should be identical to `associated_hyper_threads_bit_set()`.
	///
	/// Panics if file unreadable.
	///
	/// Returns `None` if file absent (this does not necessarily mean this is not a NUMA machine; virtio devices do not have this information).
	#[inline(always)]
	fn associated_hyper_threads_bitmask(&self) -> Option<HyperThreads>
	{
		let file_path = self.device_file_or_folder_path("local_cpus");
		if likely!(file_path.exists())
		{
			Some(HyperThreads(file_path.parse_comma_separated_bit_set().expect("Could not parse local_cpus")))
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn msi_and_msi_x_enabled(&self) -> Option<bool>
	{
		let file_path = self.msi_bus_file_path();
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
	fn enable(&self) -> Option<bool>
	{
		let file_path = self.enable_file_path();
		if likely!(file_path.exists())
		{
			Some(file_path.read_zero_or_one_bool().unwrap())
		}
		else
		{
			None
		}
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
	fn interrupt_request(&self) -> Option<InterruptRequest>
	{
		let file_path = self.device_file_or_folder_path("irq");
		
		if likely!(file_path.exists())
		{
			Some(file_path.read_value().expect("Could not parse irq"))
		}
		else
		{
			None
		}
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
	fn msi_and_msi_x_interrupt_requests(&self) -> Option<HashMap<InterruptRequest, MsiInterruptMode>>
	{
		let folder_path = self.msi_irqs_folder_path();
		if folder_path.exists()
		{
			match folder_path.read_dir()
			{
				Err(_) => None,
				
				Ok(read_dir) =>
				{
					let mut interrupt_requests = HashMap::new();
					
					for dir_entry in read_dir
					{
						if let Ok(dir_entry) = dir_entry
						{
							if let Ok(metadata) = dir_entry.metadata()
							{
								if metadata.is_file()
								{
									let file_name = dir_entry.file_name().into_vec();
									if let Ok(raw_interrupt_request) = u8::from_bytes(&file_name[..])
									{
										if let Ok(file_value) = dir_entry.path().read_raw_without_line_feed()
										{
											use self::MsiInterruptMode::*;
											let kind = match &file_value[..]
											{
												b"msix" => MSI_X,
												
												b"msi" => MSI,
												
												// Impossible in normal Linux.
												_ => continue,
											};
											
											interrupt_requests.insert(InterruptRequest::from(raw_interrupt_request), kind);
										}
									}
								}
							}
						}
					}
					
					Some(interrupt_requests)
				}
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn read_value_if_exists<F: FromBytes>(&self, file_name: &str) -> Option<F>
	where <F as FromBytes>::Error: 'static + Send + Sync + error::Error
	{
		let file_path = self.device_file_or_folder_path(file_name);
		if likely!(file_path.exists())
		{
			Some(file_path.read_value().unwrap())
		}
		else
		{
			None
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

	#[inline(always)]
	fn msi_irqs_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("msi_irqs")
	}

	#[allow(dead_code)]
	#[inline(always)]
	fn net_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("net")
	}

	#[allow(dead_code)]
	#[inline(always)]
	fn power_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("power")
	}

	#[inline(always)]
	fn subsystem_symlink_folder_path(&self) -> PathBuf
	{
		self.device_file_or_folder_path("subsystem")
	}
	
	/// Rare.
	#[inline(always)]
	fn rom_file_path(&self) -> Option<PathBuf>
	{
		let file_path = self.device_file_or_folder_path("rom");
		if unlikely!(file_path.exists())
		{
			Some(file_path)
		}
		else
		{
			None
		}
	}
	
	/// Not always present.
	#[inline(always)]
	fn config_file_path(&self) -> Option<PathBuf>
	{
		let file_path = self.device_file_or_folder_path("config");
		if unlikely!(file_path.exists())
		{
			Some(file_path)
		}
		else
		{
			None
		}
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

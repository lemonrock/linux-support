// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the unique address of a PCI bus in a system.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct PciBusAddress
{
	domain: u16,
	
	bus: BusNumber,
}

impl Into<String> for PciBusAddress
{
	#[inline(always)]
	fn into(self) -> String
	{
		(&self).into()
	}
}

impl<'a> Into<String> for &'a PciBusAddress
{
	#[inline(always)]
	fn into(self) -> String
	{
		format!("{:04x}:{:02x}", self.domain, self.bus)
	}
}

impl PciBusAddress
{
	/// All PCI buses, including root.
	#[inline(always)]
	pub fn all(sys_path: &SysPath) -> io::Result<HashMap<PciBusAddress, io::Result<PciBusDetails>>>
	{
		let read_dir = sys_path.class_pci_bus_folder_path().read_dir()?;
		let mut pci_buses: HashMap<PciBusAddress, io::Result<PciBusDetails>> = HashMap::new();
		for dir_entry in read_dir
		{
			if let Ok(dir_entry) = dir_entry
			{
				if let Ok(file_type) = dir_entry.file_type()
				{
					if !file_type.is_symlink()
					{
						continue
					}
					
					if let Some(pci_bus_address) = Self::parse_pci_bus(dir_entry)
					{
						if let Ok(canonical_parent_folder_path_of_pci_bus_folder_path) = dir_entry.path().append(dir_entry.file_name()).append("../..").canonicalize()
						{
							let details = match pci_bus_address.bus(canonical_parent_folder_path_of_pci_bus_folder_path)
							{
								Err(error) => Err(error),
								Ok(bus) => bus.details()
							};
							
							pci_buses.insert(pci_bus_address, details);
						}
					}
				}
			}
		}
		Ok(pci_buses)
	}
	
	#[inline(always)]
	fn parse_pci_bus(dir_entry: DirEntry) -> Option<Self>
	{
		let file_name = dir_entry.file_name();
		
		// `file_name` is of the format `XXXX:YY` where `XXXX` is a hexadecimal domain and `YY` is a hexadecimal bus.
		const Template: &'static [u8] = b"XXXX:YY";
		if file_name.len() != Template.len()
		{
			return None
		}
		
		let file_name_bytes = file_name.into_vec();
		
		if unlikely!(*unsafe { file_name_bytes.get_unchecked(4) } != b':')
		{
			return None
		}
		
		Some
		(
			Self
			{
				domain: match u16::parse_hexadecimal_number_upper_or_lower_case(&file_name_bytes[.. 4])
				{
					Err(_) => return None,
					Ok(domain) => domain,
				},
				
				bus: match u8::parse_hexadecimal_number_upper_or_lower_case(&file_name_bytes[5 .. 7])
				{
					Err(_) => return None,
					Ok(bus) => bus,
				},
			}
		)
	}
	
	/// Finds all primary PCI buses (those without a PCI device as a parent).
	///
	/// These have a different child file system layout.
	pub fn primary(sys_path: &SysPath) -> io::Result<HashSet<PrimaryPciBusAddress>>
	{
		let read_dir = sys_path.devices_folder_path().read_dir()?;
		let mut primary_pci_bus_addresses = HashSet::new();
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
				}
				
				let file_name = dir_entry.file_name();
				
				// `file_name` is of the format `pciXXXX:YY` where `XXXX` is a hexadecimal domain and `YY` is a hexadecimal bus.
				const Template: &'static [u8] = b"pciXXXX:YY";
				if file_name.len() != Template.len()
				{
					continue
				}
				
				if !file_name.starts_with(b"pci")
				{
					continue
				}
				
				let file_name_bytes = file_name.into_vec();
				
				if unlikely!(*unsafe { file_name_bytes.get_unchecked(7) } != b':')
				{
					continue
				}
				
				primary_pci_bus_addresses.insert
				(
					PrimaryPciBusAddress
					(
						PciBusAddress
						{
							domain: match u16::parse_hexadecimal_number_upper_or_lower_case(&file_name_bytes[3 .. 7])
							{
								Err(_) => continue,
								Ok(domain) => domain,
							},
							
							bus: match u8::parse_hexadecimal_number_upper_or_lower_case(&file_name_bytes[8 .. 10])
							{
								Err(_) => continue,
								Ok(bus) => bus,
							},
						}
					)
				);
			}
		}
		
		Ok(primary_pci_bus_addresses)
	}
	
	/// Bus.
	fn bus(&self, canonical_parent_folder_path_of_pci_bus_folder_path: PathBuf) -> io::Result<PciBus>
	{
		debug_assert_eq!(canonical_parent_folder_path_of_pci_bus_folder_path.is_absolute(), "Not an absolute path. This isn't a perfect check: whilst a canonical path is always absolute, the reverse is not ncessarily true");
		
		Ok
		(
			PciBus
			{
				pci_bus_address: *self,
				canonical_parent_folder_path_of_pci_bus_folder_path,
			}
		)
	}
	
	/// Rescans all PCI buses and devices.
	#[inline(always)]
	pub fn rescan_all_pci_buses_and_devices(sys_path: &SysPath) -> io::Result<()>
	{
		sys_path.pci_bus_file_path("rescan").write_value(true)
	}
	
	/// Autoprobe all drivers.
	#[inline(always)]
	pub fn autoprobe_all_drivers(sys_path: &SysPath) -> io::Result<()>
	{
		sys_path.pci_bus_file_path("drivers_autoprobe").write_value(true)
	}
	
	/// Probe all drivers (effectively re-initialize all drivers).
	#[inline(always)]
	pub fn probe_all_drivers(sys_path: &SysPath) -> io::Result<()>
	{
		sys_path.pci_bus_file_path("drivers_probe").write_value(true)
	}
	
	/// Mirrors Linux command line `resource_alignment` argument.
	///
	/// See <https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html> for format.
	///
	/// If unspecified on the command line (or never set) returns an empty value.
	pub fn all_resource_alignment(sys_path: &SysPath) -> io::Result<Box<[u8]>>
	{
		sys_path.pci_bus_file_path("resource_alignment").read_raw()
	}
	
	/// Mirrors Linux command line `resource_alignment` argument.
	///
	/// See <https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html> for format.
	///
	/// To unset, write an empty value.
	pub fn set_all_resource_alignment<'a>(sys_path: &SysPath, value: impl IntoLineFeedTerminatedByteString<'a>) -> io::Result<()>
	{
		sys_path.pci_bus_file_path("resource_alignment").write_value(value)
	}
	
	/// See `slot.c` in the Linux source.
	///
	/// TODO: Slots are not finished.
	pub fn all_slots_folder_path(sys_path: &SysPath) -> PathBuf
	{
		sys_path.pci_bus_file_path("slots")
	}
}

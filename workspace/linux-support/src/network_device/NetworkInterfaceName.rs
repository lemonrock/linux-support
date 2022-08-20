// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a network interface name, such as `eth0`.
///
/// Relies on the fact that `IF_NAMESIZE` and `IFNAMSIZ` are the same and both are the same as `TASK_COMM_LEN`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkInterfaceName(ObjectName16);

impl Display for NetworkInterfaceName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl FromBytes for NetworkInterfaceName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		ObjectName16::from_bytes(bytes).map(|object_name| Self(object_name))
	}
}

impl From<ObjectName16> for NetworkInterfaceName
{
	#[inline(always)]
	fn from(value: ObjectName16) -> Self
	{
		Self(value)
	}
}

impl Into<ObjectName16> for NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> ObjectName16
	{
		self.0
	}
}

impl AsRef<Path> for NetworkInterfaceName
{
	#[inline(always)]
	fn as_ref(&self) -> &Path
	{
		self.0.as_ref()
	}
}

impl<'a> Into<[c_char; 16]> for &'a NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> [c_char; 16]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; 16]> for NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> [c_char; 16]
	{
		self.0.into_object_name()
	}
}

impl<'a> Into<&'a str> for &'a NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> &'a str
	{
		(&self.0).into()
	}
}

impl Deref for NetworkInterfaceName
{
	type Target = ObjectName16;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl TryFrom<NetworkInterfaceIndex> for NetworkInterfaceName
{
	type Error = NetworkInterfaceIndexToNetworkInterfaceNameError;
	
	#[inline(always)]
	fn try_from(value: NetworkInterfaceIndex) -> Result<Self, Self::Error>
	{
		Self::try_from_network_interface_index(value)?.ok_or(NetworkInterfaceIndexToNetworkInterfaceNameError::DoesNotExistAsAnInterface)
	}
}

impl NetworkInterfaceName
{
	/// All network interface names.
	#[inline(always)]
	pub fn all() -> Result<impl Iterator<Item=NetworkInterfaceName>, String>
	{
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(|error| format!("Could not open netlink socket filet descriptor: {:?}", error))?;
		RouteNetlinkProtocol::get_links(&mut netlink_socket_file_descriptor).map(|links| links.into_iter().map(|link| link.network_interface_name))
	}
	
	/// Tries to get the network interface name.
	#[inline(always)]
	pub fn try_from_network_interface_index(value: NetworkInterfaceIndex) -> Result<Option<Self>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		NetworkDeviceSocketFileDescriptor::new()?.network_interface_index_to_network_interface_name(value)
	}
	
	/// Reads the `gro_flush_timeout` which is used in the NAPI layer.
	///
	/// Default is 0.
	#[inline(always)]
	pub fn generic_receive_offload_flush_timeout_in_nanoseconds(&self, sys_path: &SysPath) -> io::Result<u32>
	{
		self.file_path(sys_path, "gro_flush_timeout").read_value()
	}
	
	/// Writes the `gro_flush_timeout` which is used in the NAPI layer.
	///
	/// Default is 0.
	#[inline(always)]
	pub fn set_generic_receive_offload_flush_timeout_in_nanoseconds(&self, sys_path: &SysPath, generic_receive_offload_flush_timeout_in_nanoseconds: u32) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/gro_flush_timeout");
		
		let file_path = self.file_path(sys_path, "gro_flush_timeout");
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(generic_receive_offload_flush_timeout_in_nanoseconds))
		}
		else
		{
			Ok(())
		}
	}
	/// Writes the `gro_flush_timeout` which is used in the NAPI layer.
	///
	/// Default is 0.
	#[inline(always)]
	pub fn set_counter_to_decrement_before_processing_hard_interrupt_requests(&self, sys_path: &SysPath, counter_to_decrement_before_processing_hard_interrupt_requests: Option<NonZeroU32>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/napi_defer_hard_irqs");
		
		let file_path = self.file_path(sys_path, "napi_defer_hard_irqs");
		
		if file_path.exists()
		{
			let value: u32 = unsafe { transmute(counter_to_decrement_before_processing_hard_interrupt_requests) };
			file_path.write_value(UnpaddedDecimalInteger(value))
		}
		else
		{
			Ok(())
		}
	}
	
	/// `None` if no matching network interface.
	/// `Some(None)` if no PciDevice (eg because this is a loopback interface and has no device, or it is not a PCI device).
	#[inline(always)]
	pub fn pci_device<'a>(&self, sys_path: &'a SysPath) -> Result<Option<Option<PciDevice<'a>>>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		let pci_device_address = match BusDeviceAddress::try_from_network_interface_name(self.clone())?
		{
			None => return Ok(None),
			
			Some(None) => return Ok(Some(None)),
			
			Some(Some(bus_device_address)) => match PciDeviceAddress::try_from(bus_device_address)
			{
				Err(_) => return Ok(Some(None)),
				
				Ok(pci_device_address) => pci_device_address,
			},
		};
		
		let pci_device = PciDevice::new(pci_device_address, sys_path);
		Ok(Some(Some(pci_device)))
	}
	
	/// Does not exist for `lo` loopback interface.
	#[inline(always)]
	pub fn canonical_driver_folder_path(&self, sys_path: &SysPath) -> io::Result<Option<PathBuf>>
	{
		match self.device_folder_path(sys_path)
		{
			None => Ok(None),
			
			Some(device_folder_path) => Ok(Some(device_folder_path.append("driver").canonicalize()?))
		}
	}
	
	/// * Does not exist for the `lo` loopback interface
	/// * Will be `virtio0` or similar for virtio net devices.
	/// * Will be the string representation of the PciBusAddress (eg `0000:00:05.0`) for other ethernet devices.
	#[inline(always)]
	pub fn device_name_guess(&self, sys_path: &SysPath) -> Option<Box<[u8]>>
	{
		self.device_folder_path(sys_path).map(|device_folder_path| device_folder_path.file_name().unwrap().as_bytes().to_vec().into_boxed_slice())
	}
	
	/// Device folder path.
	///
	/// This isn't consistent:-
	///
	/// * May be the same as the PCI device folder (eg `/sys/devices/pci0000:00/0000:00:05.0`
	/// * May be a sub folder under it (eg `/sys/devices/pci0000:00/0000:00:05.0/virtio0`).
	/// * Does not exist for `lo` loopback interface.
	#[inline(always)]
	fn device_folder_path(&self, sys_path: &SysPath) -> Option<PathBuf>
	{
		let file_path = self.file_path(sys_path, "device");
		if likely!(file_path.exists())
		{
			Some(file_path)
		}
		else
		{
			None
		}
	}
	
	/// Reads the `dev_id`, used to differentiate devices that share the same link layer address.
	pub fn device_identifier(&self, sys_path: &SysPath) -> io::Result<u16>
	{
		let value = self.file_path(sys_path, "dev_id").read_raw_without_line_feed()?;
		u16::parse_hexadecimal_number_lower_case_with_0x_prefix(&value[..]).map_err(io_error_invalid_data)
	}
	
	/// Reads the `dev_port`, used to differentiate devices that share the same link layer address.
	#[inline(always)]
	pub fn device_port(&self, sys_path: &SysPath) -> io::Result<u16>
	{
		self.file_path(sys_path, "dev_port").read_value()
	}
	
	/// Has the link been changed to or from dormant, but the operational status may not yet have become `IF_OPER::IF_OPER_DORMANT`?
	#[inline(always)]
	pub fn is_dormant(&self, sys_path: &SysPath) -> io::Result<bool>
	{
		self.file_path(sys_path, "dormant").read_zero_or_one_bool()
	}
	
	/// Is the link being tested?
	#[inline(always)]
	pub fn is_testing(&self, sys_path: &SysPath) -> io::Result<bool>
	{
		self.file_path(sys_path, "testing").read_zero_or_one_bool()
	}
	
	/// Assigned hardware address type.
	#[inline(always)]
	pub fn assigned_hardware_address_type(&self, sys_path: &SysPath) -> io::Result<HardwareAddressType>
	{
		let value: u8 = self.file_path(sys_path, "addr_assign_type").read_value()?;
		if (value as usize) >= HardwareAddressType::COUNT
		{
			Err(io::Error::from(ErrorKind::InvalidData))
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
	
	/// Assigned hardware name type.
	#[inline(always)]
	pub fn assigned_hardware_name(&self, sys_path: &SysPath) -> io::Result<NET_NAME>
	{
		let result: io::Result<u8> = self.file_path(sys_path, "hardware_addr_type").read_value();
		
		let value = match result
		{
			Ok(value) => value,
			
			Err(error) => return if SystemCallErrorNumber::try_from(error) == Ok(EINVAL) || error.kind() == ErrorKind::InvalidInput
			{
				Ok(NET_NAME::NET_NAME_UNKNOWN)
			}
			else
			{
				Err(error)
			},
		};
		
		if (value as usize) >= NET_NAME::COUNT
		{
			Err(io::Error::from(ErrorKind::InvalidData))
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
	
	/// Receive or transmit queues.
	#[inline(always)]
	pub fn queues<'a, SQ: SysfsQueue<'a>>(&'a self, sys_path: &SysPath) -> io::Result<HashSet<SQ>>
	{
		let queues_folder_path = sys_path.network_interface_class_net_queues_folder_path(self);
		let mut queue_identifiers = HashSet::new();
		for dir_entry in queues_folder_path.read_dir()?
		{
			if let Ok(dir_entry) = dir_entry
			{
				if let Ok(file_type) = dir_entry.file_type()
				{
					if file_type.is_dir()
					{
						if let Ok(queue_identifier) = SQ::validate_file_name(dir_entry)
						{
							queue_identifiers.insert(SQ::new(self, queue_identifier));
						}
					}
				}
			}
		}
		Ok(queue_identifiers)
	}
	
	#[inline(always)]
	fn file_path(&self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		self.self_file_path(sys_path).append(file_name)
	}
	
	#[inline(always)]
	fn self_file_path(&self, sys_path: &SysPath)-> PathBuf
	{
		sys_path.network_interface_class_net_folder_path(self)
	}
}

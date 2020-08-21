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

impl<'a> Into<[c_char; ObjectName16::MaximumLengthIncludingAsciiNull]> for &'a NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName16::MaximumLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; ObjectName16::MaximumLengthIncludingAsciiNull]> for NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName16::MaximumLengthIncludingAsciiNull]
	{
		self.0.into_object_name()
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
	pub fn generic_receive_offload_flush_timeout_in_nanoseconds(self, sys_path: &SysPath) -> io::Result<u32>
	{
		self.file_path(sys_path, "gro_flush_timeout").read_value()
	}
	
	/// Writes the `gro_flush_timeout` which is used in the NAPI layer.
	///
	/// Default is 0.
	#[inline(always)]
	pub fn set_generic_receive_offload_flush_timeout_in_nanoseconds(self, sys_path: &SysPath, generic_receive_offload_flush_timeout_in_nanoseconds: u32) -> io::Result<()>
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
	
	/// Reads the `dev_id`, used to differentiate devices that share the same link layer address.
	#[inline(always)]
	pub fn device_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		let value = self.file_path(sys_path, "dev_id").read_raw_without_line_feed()?;
		u16::parse_hexadecimal_number_lower_case_with_0x_prefix(&value[..]).map_err(io_error_invalid_data)
	}
	
	/// Reads the `dev_port`, used to differentiate devices that share the same link layer address.
	#[inline(always)]
	pub fn device_port(self, sys_path: &SysPath) -> io::Result<u16>
	{
		self.file_path(sys_path, "dev_port").read_value()
	}
	
	/// Has the link been changed to or from dormant, but the operational status may not yet have become `IF_OPER::IF_OPER_DORMANT`?
	#[inline(always)]
	pub fn is_dormant(self, sys_path: &SysPath) -> io::Result<bool>
	{
		self.file_path(sys_path, "dormant").read_zero_or_one_bool()
	}
	
	/// Assigned hardware address type.
	#[inline(always)]
	pub fn assigned_hardware_address_type(self, sys_path: &SysPath) -> io::Result<NET_ADDR>
	{
		let value: u8 = self.file_path(sys_path, "addr_assign_type").read_value()?;
		if (value as usize) >= NET_ADDR::COUNT
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
	pub fn assigned_hardware_name(self, sys_path: &SysPath) -> io::Result<NET_NAME>
	{
		let result: io::Result<u8> = self.file_path(sys_path, "hardware_addr_type").read_value();
		
		let value = match result
		{
			Ok(value) => value,
			
			Err(error) => return if error.raw_os_error() == Some(EINVAL) || error.kind() == ErrorKind::InvalidInput
			{
				Ok(NET_NAME::NET_NAME_UNKNOWN)
			}
			else
			{
				Err(error)
			}
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
	
	#[inline(always)]
	fn file_path(self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		sys_path.network_interface_class_net_folder_path(&self).append(file_name)
	}
}

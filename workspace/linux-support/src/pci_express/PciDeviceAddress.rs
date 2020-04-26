// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the unique address of a PCI device in a system, such as an individual ethernet port (connector).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct PciDeviceAddress
{
	domain: u16,
	bus: u8,
	devid: u8,
	function: u8,
}

impl Into<String> for PciDeviceAddress
{
	#[inline(always)]
	fn into(self) -> String
	{
		(&self).into()
	}
}

impl<'a> Into<String> for &'a PciDeviceAddress
{
	#[inline(always)]
	fn into(self) -> String
	{
		format!("{:04x}:{:02x}:{:02x}.{:01x}", self.domain, self.bus, self.devid, self.function)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for PciDeviceAddress
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let string: String = self.into();
		let mut bytes = string.into_bytes();
		bytes.push(b'\n');
		Cow::from(bytes)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a PciDeviceAddress
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let string: String = self.into();
		let mut bytes = string.into_bytes();
		bytes.push(b'\n');
		Cow::from(bytes)
	}
}

impl TryFrom<NetworkInterfaceName> for PciDeviceAddress
{
	type Error = NetworkInterfaceNameToPciDeviceAddressConversionError;

	#[inline(always)]
	fn try_from(network_interface_name: NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		let network_interface_index = NetworkInterfaceIndex::try_from(&network_interface_name)?;
		let pci_device_address: PciDeviceAddress = Self::try_from(network_interface_index)?;
		Ok(pci_device_address)
	}
}

#[doc(hidden)]
impl TryFrom<NetworkInterfaceIndex> for PciDeviceAddress
{
	type Error = ConvertNetworkInterfaceIndexToPciDeviceAddressError;

	#[inline(always)]
	fn try_from(network_interface_index: NetworkInterfaceIndex) -> Result<Self, Self::Error>
	{
		let socket_file_descriptor = Self::open_socket_for_ioctl()?;

		let mut interface_request = ifreq::default();

		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;

		// Specify ifr_ifindex 'field'.
		let x = network_interface_index.into();
		unsafe { interface_request.ifr_ifru.ifru_ivalue().write(x) };

		// Specify ifr_data 'field'.
		unsafe { interface_request.ifr_ifru.ifru_data().write(&mut command as * mut _ as *mut c_void) };

		let result = unsafe { ioctl(socket_file_descriptor, SIOCETHTOOL, &mut interface_request as *mut _ as *mut c_void) };

		match result
		{
			-1 =>
			{
				// NOTE: Order here is important, as the `close()` system call can cause the error number to change.
				let error_number = errno();
				result.close();

				match error_number.0
				{
					EINVAL => Err(ConvertNetworkInterfaceIndexToPciDeviceAddressError::IoctlCallFailed),

					EBADF => panic!("fd is not a valid file descriptor"),
					EFAULT => panic!("argp references an inaccessible memory area"),
					ENOTTY => panic!("fd is not associated with a character special device, or, the specified request does not apply to the kind of object that the file descriptor fd references"),

					error_number @ _ => panic!("Unexpected error number `{}`", error_number),
				}
			},

			_ =>
			{
				result.close();

				let bus_info_length = unsafe { strnlen(command.bus_info.as_ptr(), ETHTOOL_BUSINFO_LEN) };
				let bus_info: [u8; ETHTOOL_BUSINFO_LEN] = unsafe { transmute(command.bus_info) };
				let bytes = &bus_info[0 .. bus_info_length];

				Ok(PciDeviceAddress::try_from(bytes)?)
			}
		}
	}
}

impl TryFrom<String> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: String) -> Result<Self, Self::Error>
	{
		Self::try_from(value.into_bytes())
	}
}

impl<'a> TryFrom<&'a str> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: &'a str) -> Result<Self, Self::Error>
	{
		Self::try_from(value.as_bytes())
	}
}

impl TryFrom<Box<[u8]>> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error>
	{
		Self::try_from(&value[..])
	}
}

impl TryFrom<Vec<u8>> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error>
	{
		Self::try_from(&value[..])
	}
}

impl<'a> TryFrom<&'a [u8]> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		// Number of bytes in a PCI address string formatted as `XXXX:XX:XX.XX`.
		const NumberOfBytesInPciAddressString: usize = 13;
		let length = value.len();
		if length != NumberOfBytesInPciAddressString
		{
			return Err(PciDeviceAddressStringParseError::LengthIsWrong { length });
		}

		let mut match_count = 0;
		let mut split = value.splitn(4, |character|
		{
			match match_count
			{
				0 | 1 =>
				{
					match_count += 1;
					*character == b':'
				},

				2 =>
				{
					match_count += 1;
					*character == b'.'
				},

				_ => unreachable!("Should not be possible"),
			}
		});

		use self::PciDeviceAddressStringParseError::*;

		Ok
		(
			Self
			{
				domain:
				{
					let value = split.next().ok_or(NoDomain)?;
					u16::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseDomain { value: value.to_owned(), cause})?
				},
				bus:
				{
					let value = split.next().ok_or(NoDeviceIdentifier)?;
					u8::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseBus { value: value.to_owned(), cause})?
				},
				devid:
				{
					let value = split.next().ok_or(NoDeviceIdentifier)?;
					let value = u8::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseDeviceIdentifier { value: value.to_owned(), cause})?;
					if unlikely!(value > 31)
					{
						return Err(DeviceNumberExceeds5BitValue { value })
					}
					value
				},
				function:
				{

					let value = split.next().ok_or(NoFunction)?;
					let value = u8::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseFunction { value: value.to_owned(), cause})?;
					if unlikely!(value > 15)
					{
						return Err(FunctionExceeds4BitValue { value })
					}
					value
				},
			}
		)
	}
}

impl PciDeviceAddress
{
	#[inline(always)]
	fn open_socket_for_ioctl() -> Result<RawFd, ConvertNetworkInterfaceIndexToPciDeviceAddressError>
	{
		use self::ConvertNetworkInterfaceIndexToPciDeviceAddressError::*;
		match unsafe { socket(AF_INET, SOCK_DGRAM, IPPROTO_IP) }
		{
			socket_file_descriptor if socket_file_descriptor >= 0 => Ok(socket_file_descriptor),

			-1 => match { errno().0 }
			{
				EACCES => Err(PermissionDenied),
				EAFNOSUPPORT => Err(Unsupported("Address family not supported")),
				EPROTOTYPE => Err(Unsupported("The socket type is not supported by the protocol")),
				EPROTONOSUPPORT => Err(Unsupported("The protocol type or the specified protocol is not supported within this domain")),

				EMFILE => Err(OutOfMemoryOrResources("The per-process descriptor table is full")),
				ENFILE => Err(OutOfMemoryOrResources("The system file table is full")),
				ENOBUFS => Err(OutOfMemoryOrResources("Insufficient buffer space is available; the socket cannot be created until sufficient resources are freed")),
				ENOMEM => Err(OutOfMemoryOrResources("Insufficient memory was available to fulfill the request")),

				illegal @ _ => panic!("socket() had illegal errno '{}'", illegal),
			},

			illegal @ _ => panic!("Illegal result '{}' from socket()", illegal),
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the unique address of a PCI device in a system, such as an individual ethernet port (connector).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
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

				// Technically incorrect, as the length can be ETHTOOL_BUSINFO_LEN with no terminating NUL; too bad.
				let bytes: &[u8] = unsafe { transmute(&command.bus_info[..]) };
				let c_string = CStr::from_bytes_with_nul(bytes)?;
				let string = c_string.to_str()?;
				Ok(PciDeviceAddress::try_from(string)?)
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
		let x: &str = &value;
		Self::try_from(x)
	}
}

impl TryFrom<&str> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		// Number of bytes in a PCI address string formatted as `XXXX:XX:XX.XX`.
		const NumberOfBytesInPciAddressString: usize = 13;

		let mut match_count = 0;

		let length = value.len();
		if length != NumberOfBytesInPciAddressString
		{
			return Err(PciDeviceAddressStringParseError::LengthIsWrong { length });
		}

		let mut split = value.split(|character|
		{
			match match_count
			{
				0 | 1 =>
				{
					match_count += 1;
					character == ':'
				},

				2 =>
				{
					match_count += 1;
					character == '.'
				},

				_ => false,
			}
		});

		let domain = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoDomain),
			Some(value) =>
			{
				match u16::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseDomain { value: value.to_owned(), cause }),
					Ok(value) => value,
				}
			}
		};

		let bus = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoBus),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseBus { value: value.to_owned(), cause }),
					Ok(value) => value,
				}
			}
		};

		let devid = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoDeviceIdentifier),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseDeviceIdentifier { value: value.to_owned(), cause }),
					Ok(value) => if value > 31
					{
						return Err(PciDeviceAddressStringParseError::DeviceNumberExceeds5BitValue { value })
					}
					else
					{
						value
					},
				}
			}
		};

		let function = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoFunction),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseFunction { value: value.to_owned(), cause }),
					Ok(value) => if value > 15
					{
						return Err(PciDeviceAddressStringParseError::FunctionExceeds4BitValue { value })
					}
					else
					{
						value
					},
				}
			}
		};

		Ok
		(
			Self
			{
				domain,
				bus,
				devid,
				function
			}
		)
	}
}

impl PciDeviceAddress
{
	/// A PCI device file.
	#[inline(always)]
	pub(crate) fn pci_device_file_path(&self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		self.pci_device_folder_path(sys_path).append(file_name)
	}

	/// PCI device folder path.
	#[inline(always)]
	fn pci_device_folder_path(&self, sys_path: &SysPath) -> PathBuf
	{
		let string_address: String = self.into();
		Self::pci_devices_path(sys_path, &string_address)
	}

	#[inline(always)]
	fn pci_devices_path(sys_path: &SysPath, string_address: &str) -> PathBuf
	{
		Self::pci_devices_parent_path(sys_path).append(string_address)
	}

	#[inline(always)]
	fn pci_devices_parent_path(sys_path: &SysPath) -> PathBuf
	{
		sys_path.pci_bus_file_path("devices")
	}

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

// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// PCI Express Bus Information.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PciExpressBusInformation(RawFd);

impl Drop for PciExpressBusInformation
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let socket_file_descriptor = self.0;

		// Please see <http://austingroupbugs.net/view.php?id=529> and <http://austingroupbugs.net/view.php?id=529> for why ignoring the `EINTR` error on close is actually sane.
		//
		// Frankly, the defects here are those of POSIX: (a) signals, and (b) using a file descriptor so small that it isn't thread safe.
		//
		// To be fair, both signals and file descriptors predate threads by a long way.
		unsafe { close(socket_file_descriptor) };
	}
}

impl PciExpressBusInformation
{
	/// Number of bytes in a PCI address string such as `XXXX:XX:XX.XX`.
	pub const NumberOfBytesInPciAddressString: usize = 13;

	/// Obtains a raw PCI bus address string in the format `XXXX:XX:XX.XX`.
	pub fn raw_pci_bus_address_for_network_interface_index(network_interface_one_based_index: u32) -> Result<String, OpenPciExpressBusInformationError>
	{
		debug_assert!(ETHTOOL_BUSINFO_LEN > Self::NumberOfBytesInPciAddressString + 1, "ETHTOOL_BUSINFO_LEN must exceed by at least one (for a Nul byte)");
		
		let socket_file_descriptor = Self::open_socket_for_ioctl()?;
		
		let mut interface_request = ifreq::default();
		
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		// Specify ifr_ifindex 'field'.
		unsafe { write(interface_request.ifr_ifru.ifru_ivalue(), network_interface_one_based_index as i32) };
	
		// Specify ifr_data 'field'.
		unsafe { write(interface_request.ifr_ifru.ifru_data(), &mut command as * mut _ as *mut c_void) };

		let raw_pci_bus_address = match unsafe { ioctl(socket_file_descriptor.0, SIOCETHTOOL, &mut interface_request as *mut _ as *mut c_void) }
		{
			-1 => Err(OpenPciExpressBusInformationError::IoctlCallFailed),

			_ =>
			{
				// Technically incorrect, as the length can be ETHTOOL_BUSINFO_LEN with no terminating NUL; too bad.
				let bytes: &[u8] = unsafe { transmute(&command.bus_info[..]) };
				match CStr::from_bytes_with_nul(bytes)
				{
					Err(_) => Err(OpenPciExpressBusInformationError::InvalidCString),

					Ok(c_string) => match c_string.to_str()
					{
						Err(_) => Err(OpenPciExpressBusInformationError::InvalidUtf8String),
						
						Ok(str) => if str.len() != Self::NumberOfBytesInPciAddressString
						{
							Err(OpenPciExpressBusInformationError::InvalidNumberOfBytesInPciBusAddress)
						}
						else
						{
							Ok(str.to_owned())
						}
					},
				}
			}
		};
		
		raw_pci_bus_address
	}

	#[inline(always)]
	fn open_socket_for_ioctl() -> Result<Self, OpenPciExpressBusInformationError>
	{
		use self::OpenPciExpressBusInformationError::*;
		match unsafe { socket(AF_INET, SOCK_DGRAM, IPPROTO_IP) }
		{
			socket_file_descriptor if socket_file_descriptor >= 0 => Ok(Self(socket_file_descriptor)),

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

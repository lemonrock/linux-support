// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a socket instance suitable for use with network_device ioctl commands.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct NetworkDeviceSocketFileDescriptor(RawFd);

impl Drop for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for NetworkDeviceSocketFileDescriptor
{
}

impl NetworkDeviceSocketFileDescriptor
{
	/// Derived from implementation of `if_indextoname` in musl libc, but without a malloc and copy.
	pub(crate) fn network_interface_index_to_network_interface_name(network_interface_index: NetworkInterfaceIndex) -> Result<Option<NetworkInterfaceName>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		Self::ifreq
		(
			SIOCGIFNAME,
			ifreq
			{
				ifr_ifrn: Default::default(),
				ifr_ifru: ifreq_ifru
				{
					ifru_ivalue: network_interface_index.into(),
				},
			},
			|ifreq| Ok(NetworkInterfaceName::from(ObjectName16::try_from(unsafe { ifreq.ifr_ifrn.ifrn_name })?)),
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFNAME)", unexpected),
			}
		)
	}
	
	/// Derived from implementation of `if_nametoindex()` in musl libc, but with stronger error handling.
	pub(crate) fn network_interface_name_to_network_interface_index(network_interface_name: NetworkInterfaceName) -> Result<Option<NetworkInterfaceIndex>, NetworkDeviceInputOutputControlError<ParseNumberError>>
	{
		Self::ifreq_from_name
		(
			SIOCGIFINDEX,
			network_interface_name,
			|ifreq| Ok(NetworkInterfaceIndex::try_from(unsafe { ifreq.ifr_ifru.ifru_ivalue })?),
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFINDEX)", unexpected),
			}
		)
	}
	
	pub(crate) fn bus_device_address(network_interface_name: NetworkInterfaceName) -> Result<Option<BusDeviceAddress>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		Self::ethtool_command
		(
			network_interface_name,
			command,
			|command| Ok(BusDeviceAddress::from(ObjectName32::try_from(command.bus_info)?)),
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	pub(crate) fn maximum_number_of_channels(network_interface_name: NetworkInterfaceName) -> Result<Option<NonZeroU32>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		const One: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
		
		Self::ethtool_command
		(
			network_interface_name,
			ethtool_channels
			{
				cmd: ETHTOOL_GCHANNELS,
				max_rx: 0,
				max_tx: 0,
				max_other: 0,
				max_combined: 0,
				rx_count: None,
				tx_count: None,
				other_count: None,
				combined_count: None,
			},
			|command|
			{
				let maximum_number_of_channels = max(max(command.max_rx, command.max_tx), command.max_combined);
				Ok
				(
					if unlikely!(maximum_number_of_channels == 0)
					{
						One
					}
					else
					{
						unsafe { NonZeroU32::new_unchecked(maximum_number_of_channels) }
					}
				)
			},
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(One)),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn new_inet() -> Result<Self, CreationError>
	{
		// Have also seen:
		// * `AF_UNIX` used for `domain`.
		// * `IPPROTO_IP` used for `ethernet_protocol`.
		new_socket(AF_INET, SOCK_DGRAM, 0, false).map(|file_descriptor| Self(file_descriptor))
	}
	
	#[inline(always)]
	fn ifreq_from_name<V: Sized, E: error::Error + 'static>(request: i32, network_interface_name: NetworkInterfaceName, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		Self::ifreq
		(
			request,
			ifreq
			{
				ifr_ifrn: ifreq_ifrn
				{
					ifrn_name: network_interface_name.into(),
				},
				ifr_ifru: Default::default(),
			},
			ok_handler,
			error_handler
		)
	}
	
	#[inline(always)]
	fn ifreq<V: Sized, E: error::Error + 'static>(request: i32, mut ifreq: ifreq, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		match Self::new_inet()?.input_output_control(request, &mut ifreq)
		{
			Ok(()) => Ok(Some(ok_handler(ifreq).map_err(NetworkDeviceInputOutputControlError::ControlOperation)?)),
			
			Err(errno) => error_handler(errno).map_err(NetworkDeviceInputOutputControlError::ControlOperation),
			
			Err(unexpected @ _) => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
		}
	}
	
	#[inline(always)]
	fn ethtool_command<C: Sized, V: Sized, E: error::Error + 'static>(network_interface_name: NetworkInterfaceName, mut command: C, ok_handler: impl FnOnce(C) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		let mut ifr = ifreq
		{
			ifr_ifrn: ifreq_ifrn
			{
				ifrn_name: network_interface_name.into(),
			},
			ifr_ifru: ifreq_ifru
			{
				ifru_data: &mut command as *mut C as *mut c_void,
			},
		};
		
		match Self::new_inet()?.input_output_control(SIOCETHTOOL, &mut ifr)
		{
			Ok(()) => Ok(Some(ok_handler(command).map_err(NetworkDeviceInputOutputControlError::ControlOperation)?)),
			
			Err(errno) => error_handler(errno).map_err(NetworkDeviceInputOutputControlError::ControlOperation),
			
			Err(unexpected @ _) => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
		}
	}
	
	#[inline(always)]
	fn input_output_control(&self, request: i32, ifreq: &mut ifreq) -> Result<(), Errno>
	{
		let result = unsafe { ioctl(self.as_raw_fd(), request, &mut ifreq as *mut _ as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result {} from ioctl()", result)
		}
	}
}

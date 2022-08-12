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
	#[inline(always)]
	pub fn new() -> Result<Self, CreationError>
	{
		// Use of `AF_INET` and `SOCK_DGRAM` is the same as the code in ethtool, ethtool.c, function `ioctl_init()`.
		new_socket(AF_INET, SOCK_DGRAM, 0, false).map(|file_descriptor| Self(file_descriptor))
	}
	
	/// Derived from implementation of `if_indextoname` in musl libc, but without a malloc and copy.
	pub fn network_interface_index_to_network_interface_name(&self, network_interface_index: NetworkInterfaceIndex) -> Result<Option<NetworkInterfaceName>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		self.ifreq
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
			|error_number| match error_number
			{
				ENODEV | ENXIO => Ok(None),
				
				EPERM => panic!("Permission denied"),
				
				_ => unreachable_code(format_args!("Unexpected error {} from ioctl(SIOCGIFNAME)", error_number)),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn ifreq_from_name<V: Sized, E: error::Error + 'static>(&self, request: i32, network_interface_name: NetworkInterfaceName, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.set_ifreq_from_name(request, network_interface_name, Default::default(), ok_handler, error_handler)
	}
	
	#[inline(always)]
	pub(crate) fn set_ifreq_from_name<V: Sized, E: error::Error + 'static>(&self, request: i32, network_interface_name: NetworkInterfaceName, ifr_ifru: ifreq_ifru, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.ifreq
		(
			request,
			ifreq
			{
				ifr_ifrn: ifreq_ifrn
				{
					ifrn_name: network_interface_name.into(),
				},
				ifr_ifru,
			},
			ok_handler,
			error_handler
		)
	}
	
	#[inline(always)]
	pub(crate) fn ifreq<V: Sized, E: error::Error + 'static>(&self, request: i32, mut ifreq: ifreq, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(SystemCallErrorNumber) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self.input_output_control(request, &mut ifreq)
		{
			Ok(()) => Ok(Some(ok_handler(ifreq).map_err(ControlOperation)?)),
			
			Err(error_number) => error_handler(error_number).map_err(ControlOperation),
		}
	}
	
	#[inline(always)]
	pub(crate) fn ethtool_command<C: EthtoolCommand, V: Sized, E: error::Error + 'static>(&self, network_interface_name: NetworkInterfaceName, mut command: C, ok_handler: impl FnOnce(C) -> Result<V, E>, error_handler: impl FnOnce(SystemCallErrorNumber) -> Result<Option<V>, E>, not_supported: impl FnOnce(C) -> V) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
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
		
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self.input_output_control(SIOCETHTOOL, &mut ifr)
		{
			Ok(()) => Ok(Some(ok_handler(command).map_err(ControlOperation)?)),
			
			Err(ENODEV | ENXIO) => Ok(None),
			
			Err(EOPNOTSUPP) => Ok(Some(not_supported(command))),
			
			Err(EPERM) => Err(PermissionDenied),
			
			Err(ENOMEM) => Err(OutOfKernelMemory),
			
			Err(EFAULT) => unreachable_code(format_args!("We passed a bad memory address")),
			
			Err(other) => error_handler(other).map_err(ControlOperation),
		}
	}
	
	#[inline(always)]
	fn input_output_control(&self, request: i32, ifreq: &mut ifreq) -> Result<(), SystemCallErrorNumber>
	{
		let result = unsafe { ioctl(self.as_raw_fd(), request, ifreq as *mut _ as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(SystemCallErrorNumber::from_errno())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from ioctl()", result))
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents an eXpress Data Path (XDP) socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExpressDataPathSocketFileDescriptor(RawFd);

impl Drop for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for ExpressDataPathSocketFileDescriptor
{
}

impl ExpressDataPathSocketFileDescriptor
{
	/// New.
	#[inline(always)]
	pub fn new(blocking: &Blocking) -> Result<Self, io::Error>
	{
		const protocol: i32 = 0;
		const Flags: i32 = SOCK_RAW | SOCK_CLOEXEC;
		let type_ = if blocking.is_non_blocking()
		{
			Flags | SOCK_NONBLOCK
		}
		else
		{
			Flags
		};
		
		let result = unsafe { socket(AF_XDP, type_, protocol) };
		if likely!(result >= 0)
		{
			Ok(Self(result))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected error {} from `socket(AF_XDP, SOCK_RAW | SOCK_CLOEXEC, {})`", result, protocol);
		}
	}
}

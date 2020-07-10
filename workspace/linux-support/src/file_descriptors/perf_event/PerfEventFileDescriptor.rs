// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a perf event file descriptor.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerfEventFileDescriptor(RawFd);

impl Drop for PerfEventFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for PerfEventFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for PerfEventFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for PerfEventFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for PerfEventFileDescriptor
{
}

impl PerfEventFileDescriptor
{
	/// Opens a new instance using `perf_event_open()`.
	pub fn open() -> Result<Self, io::Error>
	{
		unimplemented!("Create a wrapper for perf_event_open()")
	}
}

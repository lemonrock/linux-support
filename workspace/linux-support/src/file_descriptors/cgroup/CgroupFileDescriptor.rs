// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Cgroup file descriptor which is backed by a `File`.
#[derive(Debug)]
pub struct CgroupFileDescriptor(RawFd);

impl From<File> for CgroupFileDescriptor
{
	#[inline(always)]
	fn from(value: File) -> Self
	{
		unsafe { Self::from_raw_fd(value.into_raw_fd()) }
	}
}

impl Into<File> for CgroupFileDescriptor
{
	#[inline(always)]
	fn into(self) -> File
	{
		unsafe { File::from_raw_fd(self.into_raw_fd()) }
	}
}

impl Drop for CgroupFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for CgroupFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for CgroupFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for CgroupFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for CgroupFileDescriptor
{
}

impl UsedAsValueInArrayMapDescriptor for CgroupFileDescriptor
{
	#[inline(always)]
	fn transmute_to_file_descriptor_copies(values: Vec<RawFd>) -> Vec<FileDescriptorCopy<Self>>
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[FileDescriptorCopy<Self>]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_to_file_descriptor_copy(value: RawFd) -> FileDescriptorCopy<Self>
	{
		unsafe { transmute(value) }
	}
}

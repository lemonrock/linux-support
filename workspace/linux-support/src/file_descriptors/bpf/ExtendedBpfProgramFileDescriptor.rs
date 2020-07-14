// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents an extended BPF program file descriptor.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtendedBpfProgramFileDescriptor(NonZeroI32);

impl Drop for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.get()
	}
}

impl IntoRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(NonZeroI32::new_unchecked(fd))
	}
}

impl FileDescriptor for ExtendedBpfProgramFileDescriptor
{
}

impl BpfFileDescriptor for ExtendedBpfProgramFileDescriptor
{
}

impl UsedAsValueInArrayMapDescriptor for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn transmute_to_file_descriptor_copies(values: Vec<RawFd>) -> Vec<Option<FileDescriptorCopy<Self>>>
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[Option<FileDescriptorCopy<Self>>]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_to_file_descriptor_copy(value: RawFd) -> Option<FileDescriptorCopy<Self>>
	{
		unsafe { transmute(value) }
	}
}

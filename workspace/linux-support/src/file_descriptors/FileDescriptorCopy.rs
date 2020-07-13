// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A copy of a file descriptor that when dropped does not close the file descriptor.
#[derive(Debug)]
#[repr(transparent)]
pub struct FileDescriptorCopy<FD: FileDescriptor>(ManuallyDrop<FD>);

impl<FD: FileDescriptor> Into<FileDescriptorCopy<FD>> for RawFd
{
	#[inline(always)]
	fn into(self) -> FileDescriptorCopy<FD>
	{
		FileDescriptorCopy::new(self)
	}
}

impl<FD: FileDescriptor> Deref for FileDescriptorCopy<FD>
{
	type Target = FD;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<FD: FileDescriptor> DerefMut for FileDescriptorCopy<FD>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.0.deref_mut()
	}
}

impl<FD: FileDescriptor> FileDescriptorCopy<FD>
{
	#[inline(always)]
	pub(crate) const fn new(raw_fd: RawFd) -> Self
	{
		Self(ManuallyDrop::new(unsafe { FD::from_raw_fd(raw_fd) }))
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor supportd by io_uring.
///
/// Defaults to `Self::Sparse`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SupportedFileDescriptor(RawFd);

impl<'a> From<&'a File> for SupportedFileDescriptor
{
	#[inline(always)]
	fn from(value: &'a File) -> Self
	{
		Self(value.as_raw_fd())
	}
}

impl<'a> From<&'a MemoryFileDescriptor> for SupportedFileDescriptor
{
	#[inline(always)]
	fn from(value: &'a MemoryFileDescriptor) -> Self
	{
		Self(value.as_raw_fd())
	}
}

impl Default for SupportedFileDescriptor
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Sparse
	}
}

impl SupportedFileDescriptor
{
	const Sparse: Self = Self(-1);
}

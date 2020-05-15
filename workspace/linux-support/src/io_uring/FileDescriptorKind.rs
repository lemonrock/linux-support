// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Copy, Clone)]
pub union FileDescriptorKind
{
	index: RegisteredFileDescriptorIndex,
	absolute: RawFd,
}

impl<'a, FD: 'a + FileDescriptor> From<&'a FD> for FileDescriptorKind
{
	#[inline(always)]
	fn from(fd: &'a FD) -> Self
	{
		Self::Absolute(fd.as_raw_fd())
	}
}

impl From<RawFd> for FileDescriptorKind
{
	#[inline(always)]
	fn from(fd: RawFd) -> Self
	{
		Self::Absolute(fd)
	}
}

impl Default for FileDescriptorKind
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for FileDescriptorKind
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "FileDescriptorKind({:?})", unsafe { self.index })
	}
}

impl PartialEq for FileDescriptorKind
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.index == other.index }
	}
}

impl Eq for FileDescriptorKind
{
}

impl PartialOrd for FileDescriptorKind
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.index.partial_cmp(&other.index) }
	}
}

impl Ord for FileDescriptorKind
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.index.cmp(&other.index) }
	}
}

impl Hash for FileDescriptorKind
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		unsafe { self.index.hash(state) }
	}
}

impl FileDescriptorKind
{
	#[inline(always)]
	pub(super) const fn NumberOfBuffers(number_of_buffers: NonZeroU32) -> Self
	{
		Self::Index(RegisteredFileDescriptorIndex(number_of_buffers.get()))
	}
	
	#[inline(always)]
	pub(super) const fn Index(index: RegisteredFileDescriptorIndex) -> Self
	{
		Self
		{
			index
		}
	}

	#[inline(always)]
	pub(super) const fn Absolute(absolute: RawFd) -> Self
	{
		Self
		{
			absolute
		}
	}

	pub(super) const Irrelevant: Self = Self::Absolute(-1);
}

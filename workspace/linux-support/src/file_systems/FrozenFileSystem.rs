// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A lock for a frozen file system.
///
/// Dropping this with `drop()` thaws the file system.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrozenFileSystem<'a>(pub(crate) &'a DirectoryFileDescriptor);

impl<'a> Drop for FrozenFileSystem<'a>
{
	#[allow(unused)]
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.thaw_file_system();
	}
}

impl<'a> Deref for FrozenFileSystem<'a>
{
	type Target = DirectoryFileDescriptor;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl<'a> FrozenFileSystem<'a>
{
	/// Freeze.
	#[inline(always)]
	pub fn freeze(mount_point: &'a DirectoryFileDescriptor) -> io::Result<Self>
	{
		mount_point.freeze_file_system()
	}
}

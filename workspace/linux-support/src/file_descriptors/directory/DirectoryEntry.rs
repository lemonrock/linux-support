// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A directory entry.
#[repr(transparent)]
pub struct DirectoryEntry<'a>(dirent, PhantomData<&'a ()>);

impl<'a> DirectoryEntry<'a>
{
	/// Inode.
	#[inline(always)]
	pub fn inode(&self) -> Inode
	{
		self.0.inode()
	}

	/// File type.
	#[inline(always)]
	pub fn file_type(&self) -> FileType
	{
		self.0.file_type()
	}

	/// Name.
	#[inline(always)]
	pub fn name(&self) -> &CStr
	{
		self.0.name()
	}

	/// A position that can be rewinded to.
	#[inline(always)]
	pub fn DirectoryEntryRewindPosition(&self) -> DirectoryEntryRewindPosition<'a>
	{
		DirectoryEntryRewindPosition(self.0.d_off, PhantomData)
	}
}

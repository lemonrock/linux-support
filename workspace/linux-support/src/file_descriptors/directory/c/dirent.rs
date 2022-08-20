// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct dirent
{
	pub(super) d_ino: ino_t,
	pub(super) d_off: off_t,
	pub(super) d_reclen: c_ushort,
	pub(super) d_type: c_uchar,

	/// Note: This isn't necessarily 256 bytes long; it can be shorter or longer, but is NUL-terminated.
	///
	/// It is a variable-sized array.
	d_name: [c_char; 256],
}

impl dirent
{
	#[inline(always)]
	pub(super) fn inode(&self) -> Inode
	{
		Inode::from(self.d_ino)
	}

	#[inline(always)]
	pub(super) fn file_type(&self) -> FileType
	{
		FileType::from_dtype(self.d_type)
	}

	#[inline(always)]
	pub(super) fn name(&self) -> &CStr
	{
		unsafe { CStr::from_ptr(self.d_name.as_ptr()) }
	}
}

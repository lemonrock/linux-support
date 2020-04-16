// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FileType
{
	/// Unknown.
	Unknown = 0,

	#[allow(missing_docs)]
	RegularFile = S_IFREG,

	#[allow(missing_docs)]
	Directory = S_IFDIR,

	#[allow(missing_docs)]
	SymbolicLink = S_IFLNK,

	#[allow(missing_docs)]
	BlockDevice = S_IFBLK,

	#[allow(missing_docs)]
	CharacterDevice = S_IFCHR,

	#[allow(missing_docs)]
	Fifo = S_IFIFO,

	#[allow(missing_docs)]
	UnixDomainSocket = S_IFSOCK,
}

impl From<u16> for FileType
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::from(value as u32)
	}
}

impl From<mode_t> for FileType
{
	#[inline(always)]
	fn from(value: mode_t) -> Self
	{
		Self::const_from(value)
	}
}

impl Into<mode_t> for FileType
{
	#[inline(always)]
	fn into(self) -> mode_t
	{
		self as mode_t
	}
}

impl FileType
{
	#[inline(always)]
	const fn const_from(value: mode_t) -> Self
	{
		unsafe { transmute(value & S_IFMT) }
	}
}

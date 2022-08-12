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

	/// Documented in the Coda file system.
	BsdStyleWhiteout = 14 << 12,
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
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_unknown(self) -> bool
	{
		self == FileType::Unknown
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_regular_file(self) -> bool
	{
		self == FileType::RegularFile
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_directory(self) -> bool
	{
		self == FileType::Directory
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_symbolic_link(self) -> bool
	{
		self == FileType::SymbolicLink
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_block_device(self) -> bool
	{
		self == FileType::BlockDevice
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_character_device(self) -> bool
	{
		self == FileType::CharacterDevice
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_fifo(self) -> bool
	{
		self == FileType::Fifo
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_unix_domain_socket(self) -> bool
	{
		self == FileType::UnixDomainSocket
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_bsd_style_whiteout(self) -> bool
	{
		self == FileType::BsdStyleWhiteout
	}

	#[inline(always)]
	const fn const_from(value: mode_t) -> Self
	{
		unsafe { transmute(value & S_IFMT) }
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn from_dtype(d_type: c_uchar) -> Self
	{
		unsafe { transmute((d_type as u32) << 12) }
	}
}

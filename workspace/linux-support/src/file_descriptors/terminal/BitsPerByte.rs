// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts bits per byte.
///
/// Defaults to 8.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum BitsPerByte
{
	/// Five.
	Five = CS5,

	/// Six.
	Six = CS6,

	/// Seven.
	Seven = CS7,

	/// Eight.
	Eight = CS8,
}

impl Into<tcflag_t> for BitsPerByte
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl Default for BitsPerByte
{
	#[inline(always)]
	fn default() -> Self
	{
		BitsPerByte::Eight
	}
}

impl MultipleBits for BitsPerByte
{
	const Bitmask: tcflag_t = CSIZE;

	#[inline(always)]
	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(clean_mode_flags) }
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a constant pointer to a C string.
///
/// Must be constructed from a byte string literal with a final `\0`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCStr(pub &'static [u8]);

impl Display for ConstCStr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{:?}", self.0)
	}
}

impl ConstCStr
{
	/// As a pointer to a nul-terminated C string.
	#[inline(always)]
	pub fn as_ptr(self) -> *const c_char
	{
		self.0.as_ptr() as *const _
	}

	/// As a pointer to a nul-terminated C string.
	#[inline(always)]
	pub fn as_cstr<'a>(self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.as_ptr()) }
	}

	/// Length excluding nul.
	#[inline(always)]
	pub fn length_excluding_trailing_nul(self) -> usize
	{
		self.0.len() - 1
	}
}

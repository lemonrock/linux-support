// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Also known as 'version number'.
///
/// Not present for all file systems (eg `vfat` and possibly `fuse`-based filesystems).
///
/// Obtain it using `OnDiskFileDescriptor`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InodeGenerationNumber(i32);

impl From<i32> for InodeGenerationNumber
{
	#[inline(always)]
	fn from(value: i32) -> Self
	{
		Self(value)
	}
}

impl Into<i32> for InodeGenerationNumber
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0
	}
}

impl InodeGenerationNumber
{
	/// Const-variant of `Self::from()`.
	#[inline(always)]
	pub const fn from_i32(value: i32) -> Self
	{
		Self(value)
	}
}

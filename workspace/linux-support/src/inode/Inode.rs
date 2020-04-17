// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An inode
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Inode(ino_t);

impl From<ino_t> for Inode
{
	#[inline(always)]
	fn from(value: ino_t) -> Self
	{
		Self(value)
	}
}

impl Into<ino_t> for Inode
{
	#[inline(always)]
	fn into(self) -> ino_t
	{
		self.0
	}
}

impl ParseNumber for Inode
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Ok(Self(u64::parse_number(bytes, radix, parse_byte)?))
	}
}

impl Inode
{
	/// Zero.
	pub const Zero: Self = Self(0);
}

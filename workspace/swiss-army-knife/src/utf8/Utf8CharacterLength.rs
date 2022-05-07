// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Utf8CharacterLength
{
	One = 1,

	Two = 2,

	Three = 3,

	Four = 4,
}

impl const Into<u8> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl const Into<u32> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let into: u8 = self.into();
		into as u32
	}
}

impl const Into<usize> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> usize
	{
		let into: u8 = self.into();
		into as usize
	}
}

impl const Into<NonZeroUsize> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> NonZeroUsize
	{
		let into: usize = self.into();
		new_non_zero_usize(into)
	}
}

impl Utf8CharacterLength
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn add_from_str(self, remaining: &str) -> usize
	{
		self.add(remaining.len())
	}
	
	#[inline(always)]
	const fn add(self, increment: usize) -> usize
	{
		let into: usize = self.into();
		into + increment
	}
}

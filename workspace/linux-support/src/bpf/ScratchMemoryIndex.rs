// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A scratch memory index.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScratchMemoryIndex(u32);

impl From<u8> for ScratchMemoryIndex
{
	#[inline(always)]
	fn from(index: u8) -> Self
	{
		Self(index as u32)
	}
}

impl TryFrom<u16> for ScratchMemoryIndex
{
	type Error = ();

	#[inline(always)]
	fn try_from(index: u16) -> Result<Self, Self::Error>
	{
		if unlikely!(index >= BPF_MEMWORDS as u16)
		{
			Err(())
		}
		else
		{
			Ok(Self(index as u32))
		}
	}
}

impl Into<u16> for ScratchMemoryIndex
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for ScratchMemoryIndex
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl ScratchMemoryIndex
{
	/// Inclusive minimum index of 0.
	pub const InclusiveMinimum: Self = Self(0);

	/// Inclusive maximum index of 4095.
	pub const InclusiveMaximum: Self = Self(BPF_MEMWORDS - 1);
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can never be `u32::MAX`.
///
/// Internal value is shifted by 1 to exploit Rust's optimizations.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LifeTimeMicroseconds(NonZeroU32);

impl Into<u32> for LifeTimeMicroseconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.get() - 1
	}
}

impl TryFrom<u32> for LifeTimeMicroseconds
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if value == ifa_cacheinfo::INFINITY_LIFE_TIME
		{
			Err(())
		}
		else
		{
			Ok(Self::from_unchecked(value))
		}
	}
}

impl LifeTimeMicroseconds
{
	#[inline(always)]
	fn from_unchecked(value: u32) -> Self
	{
		Self(unsafe { NonZeroU32::new_unchecked(value + 1) })
	}
}

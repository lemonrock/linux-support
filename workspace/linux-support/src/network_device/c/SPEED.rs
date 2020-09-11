// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Forced speed in units of 1Mb.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct SPEED(i32);

impl Default for SPEED
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::SPEED_UNKNOWN
	}
}

impl TryFrom<u32> for SPEED
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > i32::MAX as u32)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value as i32))
		}
	}
}

impl SPEED
{
	#[inline(always)]
	pub fn is_unknown(self) -> bool
	{
		self == Self::SPEED_UNKNOWN
	}
	
	pub const SPEED_10: Self = Self(10);
	
	pub const SPEED_100: Self = Self(100);
	
	pub const SPEED_1000: Self = Self(1_000);
	
	pub const SPEED_2500: Self = Self(2_500);
	
	pub const SPEED_5000: Self = Self(5_000);
	
	pub const SPEED_10000: Self = Self(10_000);
	
	pub const SPEED_14000: Self = Self(14_000);
	
	pub const SPEED_20000: Self = Self(20_000);
	
	pub const SPEED_25000: Self = Self(25_000);
	
	pub const SPEED_40000: Self = Self(40_000);
	
	pub const SPEED_50000: Self = Self(50_000);
	
	pub const SPEED_56000: Self = Self(56_000);
	
	pub const SPEED_100000: Self = Self(100_000);
	
	pub const SPEED_200000: Self = Self(200_000);
	
	pub const SPEED_400000: Self = Self(400_000);
	
	pub const SPEED_UNKNOWN: Self = Self(-1);
}


// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Explicit setting, in interrupts-to-coalesce per interval.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct IntelIxgbevfInterruptThrottleRate(NonZeroU32);

impl Default for IntelIxgbevfInterruptThrottleRate
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::InclusiveMinimum
	}
}

impl Into<u32> for IntelIxgbevfInterruptThrottleRate
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.get()
	}
}

impl TryFrom<NonZeroU32> for IntelIxgbevfInterruptThrottleRate
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if value < Self::InclusiveMinimum.0 || value > Self::InclusiveMaximum.0
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl TryFrom<u32> for IntelIxgbevfInterruptThrottleRate
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(new_non_zero_u32(value))
		}
	}
}

impl IntelIxgbevfInterruptThrottleRate
{
	const IXGBE_MAX_EITR: u32 = 0x00000FF8;
	
	/// Inclusive minimum.
	///
	/// `2`.
	///
	/// Intel says `956` in its `README`, but this doesn't match the source code.
	pub const InclusiveMinimum: Self = Self::new_unchecked(2);
	
	/// Inclusive maximum.
	///
	/// `1022`.
	///
	/// Intel says `488281` in its `README`, but this doesn't match the source code.
	pub const InclusiveMaximum: Self = Self::new_unchecked(Self::IXGBE_MAX_EITR >> 2);
	
	#[inline(always)]
	const fn new_unchecked(value: u32) -> Self
	{
		Self(new_non_zero_u32(value))
	}
}

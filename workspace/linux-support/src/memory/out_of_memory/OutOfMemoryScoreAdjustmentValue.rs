// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An adjustment value from 1 to 1000.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct OutOfMemoryScoreAdjustmentValue(NonZeroU16);

impl TryFrom<NonZeroU16> for OutOfMemoryScoreAdjustmentValue
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, ParseNumberError>
	{
		if unlikely!(value > Self::InclusiveMaximum)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl OutOfMemoryScoreAdjustmentValue
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new_unchecked(1);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new_unchecked(1000);
	
	const fn new_unchecked(value: u16) -> Self
	{
		Self(unsafe { NonZeroU16::new_unchecked(value) })
	}
}

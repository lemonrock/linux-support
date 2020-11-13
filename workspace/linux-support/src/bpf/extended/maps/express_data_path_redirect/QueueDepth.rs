// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue depth.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueDepth(NonZeroU32);

impl Into<u16> for QueueDepth
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.get() as u16
	}
}

impl Into<u32> for QueueDepth
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.get()
	}
}

impl TryFrom<u16> for QueueDepth
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		Self::try_from(value as u32)
	}
}

impl TryFrom<u32> for QueueDepth
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
impl TryFrom<NonZeroU32> for QueueDepth
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl QueueDepth
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new(1);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new(16384);
	
	const fn new(value: u32) -> Self
	{
		Self(new_non_zero_u32(value))
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue depth.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueDepth(u16);

impl Into<u32> for QueueDepth
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl TryFrom<u32> for QueueDepth
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > u16::MAX as u32)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u16)
		}
	}
}

impl TryFrom<u16> for QueueDepth
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value > Self::InclusiveMaximum.0
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
	pub const InclusiveMinimum: Self = Self(0);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(16384);
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue identifier.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueIdentifier(u16);

bit_set_aware!(QueueIdentifier);

impl Into<u16> for QueueIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<i16> for QueueIdentifier
{
	#[inline(always)]
	fn into(self) -> i16
	{
		self.0 as i16
	}
}

impl BitSetAware for QueueIdentifier
{
	const LinuxExclusiveMaximum: u16 = MAX_NUM_QUEUE;

	const InclusiveMinimum: Self = Self(0);

	const InclusiveMaximum: Self = Self((Self::LinuxExclusiveMaximum - 1) as u16);

	const Prefix: &'static [u8] = b"(irrelevant)";

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!(value < Self::LinuxExclusiveMaximum);

		Self(value)
	}
}

impl TryFrom<u32> for QueueIdentifier
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0 as u32)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value as u16))
		}
	}
}

impl TryFrom<ExpressDataPathQueueIdentifier> for QueueIdentifier
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: ExpressDataPathQueueIdentifier) -> Result<Self, Self::Error>
	{
		let value = value.into_u32();
		Self::try_from(value)
	}
}

impl QueueIdentifier
{
	/// Minimum.
	pub const InclusiveMinimum: Self = QueueCount::InclusiveMinimum.to_queue_identifier();
	
	/// Maximum.
	pub const InclusiveMaximum: Self = QueueCount::InclusiveMaximum.to_queue_identifier();
	
	/// Into u16.
	#[inline(always)]
	pub const fn into_u16(self) -> u16
	{
		self.0
	}
	
	/// Into u64.
	#[inline(always)]
	pub const fn into_u64(self) -> u64
	{
		self.into_u16() as u64
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Number of queues (non-zero).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueCount(NonZeroU16);

impl Default for QueueCount
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::InclusiveMinimum
	}
}

impl Add for QueueCount
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		let lhs = self.0.get();
		let rhs = rhs.0.get();
		
		if cfg!(debug_assertions)
		{
			let outcome = lhs.checked_add(rhs);
			debug_assert!(outcome.is_some());
			debug_assert!(outcome.unwrap() <= MAX_NUM_QUEUE)
		}
		
		Self::new_unchecked(lhs + rhs)
	}
}

impl TryFrom<usize> for QueueCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if unlikely!(value > (u16::MAX as usize))
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u16)
		}
	}
}

impl TryFrom<u64> for QueueCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u64) -> Result<Self, Self::Error>
	{
		if unlikely!(value > (u16::MAX as u64))
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u16)
		}
	}
}

impl TryFrom<u32> for QueueCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > (u16::MAX as u32))
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u16)
		}
	}
}

impl TryFrom<u16> for QueueCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(new_non_zero_u16(value))
		}
	}
}

impl TryFrom<NonZeroU16> for QueueCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
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

impl TryFrom<NonZeroU32> for QueueCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		let value = value.get();
		if unlikely!(value > u16::MAX as u32)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(new_non_zero_u16(value as u16))
		}
	}
}

impl Into<NonZeroU32> for QueueCount
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		new_non_zero_u32(self.0.get() as u32)
	}
}

impl Into<NonZeroU16> for QueueCount
{
	#[inline(always)]
	fn into(self) -> NonZeroU16
	{
		self.0
	}
}

impl Into<u16> for QueueCount
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.get()
	}
}

impl Into<u32> for QueueCount
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.get() as u32
	}
}

impl Into<usize> for QueueCount
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0.get() as usize
	}
}

impl QueueCount
{
	/// Minimum.
	pub const InclusiveMinimum: Self = Self::new_unchecked(1);
	
	/// Maximum.
	pub const InclusiveMaximum: Self = Self::new_unchecked(MAX_NUM_QUEUE);
	
	/// To queue identifier.
	pub const fn to_queue_identifier(self) -> QueueIdentifier
	{
		QueueIdentifier(self.0.get() - 1)
	}
	
	/// Try-from.
	pub fn try_from_non_zero_u16_saturated(value: NonZeroU16) -> Self
	{
		if value > Self::InclusiveMaximum.0
		{
			Self::InclusiveMaximum
		}
		else
		{
			Self(value)
		}
	}
	
	#[inline(always)]
	pub(crate) const fn new_unchecked(value: u16) -> Self
	{
		Self(new_non_zero_u16(value))
	}
	
	/// Queue identifiers.
	#[inline(always)]
	pub fn queue_identifiers(self) -> impl Iterator<Item=QueueIdentifier>
	{
		let exclusive_maximum: u16 = self.into();
		(0 .. exclusive_maximum).map(QueueIdentifier)
	}
	
	/// Caps number of hyper threads.
	#[inline(always)]
	pub fn from_number_of_hyper_threads_capped_to_inclusive_maximum(mut non_zero_number_of_hyper_threads: HyperThreads) -> (HyperThreads, Self)
	{
		const MaximumLength: usize = QueueCount::InclusiveMaximum.0.get() as usize;
		
		let length = non_zero_number_of_hyper_threads.len();
		debug_assert_ne!(length, 0);
		
		if length > MaximumLength
		{
			non_zero_number_of_hyper_threads.cap_retaining_lowest_values(MaximumLength);
			(non_zero_number_of_hyper_threads, QueueCount::InclusiveMaximum)
		}
		else
		{
			(non_zero_number_of_hyper_threads, Self(new_non_zero_u16(length as u16)))
		}
	}
	
	#[inline(always)]
	pub(crate) fn new_queue_configurations<Configuration>(self) -> HashMap<QueueIdentifier, Configuration>
	{
		HashMap::with_capacity(self.into())
	}
}

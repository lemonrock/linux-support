// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Value size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ValueSizeU16(NonZeroU16);

impl TryFrom<usize> for ValueSizeU16
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if unlikely!(value > u16::MAX as usize)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u16)
		}
	}
}

impl TryFrom<u16> for ValueSizeU16
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

impl TryFrom<NonZeroU16> for ValueSizeU16
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if value >= Self::ExclusiveMaximum.0
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl ValueSizeU16
{
	const PAGE_SIZE: u16 = (1 << ValueSizeU32::PAGE_SHIFT) as u16;
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new_unsafe(1);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new_unsafe(Self::PAGE_SIZE);
	
	/// Exclusive maximum.
	pub const ExclusiveMaximum: Self = Self::new_unsafe(Self::PAGE_SIZE + 1);
	
	#[inline(always)]
	const fn new_unsafe(value: u16) -> Self
	{
		Self(new_non_zero_u16(value))
	}
	
	#[inline(always)]
	pub(crate) const fn to_non_zero_u32(self) -> NonZeroU32
	{
		new_non_zero_u32(self.0.get() as u32)
	}
	
	#[inline(always)]
	pub(crate) fn try_from_value_size<V: Sized>() -> Result<Self, ParseNumberError>
	{
		Self::try_from(size_of::<V>())
	}
}

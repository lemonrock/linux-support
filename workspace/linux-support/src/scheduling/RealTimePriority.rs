// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Real-time priority, from 1 to 99 inclusive.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RealTimePriority(NonZeroU8);

impl TryFrom<NonZeroU8> for RealTimePriority
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		use self::ParseNumberError::*;

		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl TryFrom<u8> for RealTimePriority
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		use self::ParseNumberError::*;

		if unlikely!(value == 0)
		{
			return Err(WasZero)
		}
		Self::try_from(unsafe { NonZeroU8::new_unchecked(value) })
	}
}

impl Into<NonZeroU8> for RealTimePriority
{
	#[inline(always)]
	fn into(self) -> NonZeroU8
	{
		self.0
	}
}

impl Into<u8> for RealTimePriority
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0.get()
	}
}

impl ParseNumber for RealTimePriority
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Ok(Self(NonZeroU8::parse_number(bytes, radix, parse_byte)?))
	}
}

impl ParseNumber for Option<RealTimePriority>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let value = u8::parse_number(bytes, radix, parse_byte)?;
		if value == 0
		{
			Ok(None)
		}
		else
		{
			Ok(Some(RealTimePriority::try_from(unsafe { NonZeroU8::new_unchecked(value) })?))
		}
	}
}

impl RealTimePriority
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(unsafe { NonZeroU8::new_unchecked(1) });

	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(unsafe { NonZeroU8::new_unchecked(99) });
}

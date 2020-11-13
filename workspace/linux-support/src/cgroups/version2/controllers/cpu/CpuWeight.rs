// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A weight from 1 to 10,000.
///
/// Larger values correspond to more negative nice values.
///
/// Default is 100.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct CpuWeight(NonZeroU16);

impl TryFrom<NonZeroU16> for CpuWeight
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl ParseNumber for CpuWeight
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let value = NonZeroU16::parse_number(bytes, radix, parse_byte)?;
		Self::try_from(value)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for CpuWeight
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(self.0).into_line_feed_terminated_byte_string()
	}
}

impl CpuWeight
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new_unchecked(1);
	
	/// Default.
	pub const Default: Self = Self::new_unchecked(100);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new_unchecked(10_000);
	
	const fn new_unchecked(value: u16) -> Self
	{
		Self(new_non_zero_u16(value))
	}
}

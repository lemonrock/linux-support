// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum number.
///
/// Defaults to `Maximum`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaximumNumber<V: ParseNumber + Copy + IntegerIntoLineFeedTerminatedByteString>
{
	/// A finite value.
	///
	/// Can be zero.
	Finite(V),

	/// A system defined maximum.
	Maximum,
}

impl<V: ParseNumber + Copy + IntegerIntoLineFeedTerminatedByteString> Default for MaximumNumber<V>
{
	#[inline(always)]
	fn default() -> Self
	{
		MaximumNumber::Maximum
	}
}

impl<V: ParseNumber + Copy + IntegerIntoLineFeedTerminatedByteString> Into<Option<V>> for MaximumNumber<V>
{
	#[inline(always)]
	fn into(self) -> Option<V>
	{
		use self::MaximumNumber::*;
		
		match self
		{
			Finite(value) => Some(value),
			
			Maximum => None,
		}
	}
}

impl<V: ParseNumber + Copy + IntegerIntoLineFeedTerminatedByteString> From<Option<V>> for MaximumNumber<V>
{
	#[inline(always)]
	fn from(value: Option<V>) -> Self
	{
		use self::MaximumNumber::*;
		
		match value
		{
			Some(value) => Finite(value),
			
			None => Maximum,
		}
	}
}

impl<V: ParseNumber + Copy + IntegerIntoLineFeedTerminatedByteString> ParseNumber for MaximumNumber<V>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		use self::MaximumNumber::*;
		
		if &bytes[..] == b"max"
		{
			Ok(Maximum)
		}
		else
		{
			Ok(Finite(V::parse_number(bytes, radix, parse_byte)?))
		}
	}
}

impl<V: ParseNumber + Copy + IntegerIntoLineFeedTerminatedByteString> IntoLineFeedTerminatedByteString<'static> for MaximumNumber<V>
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'static, [u8]>
	{
		use self::MaximumNumber::*;

		match self
		{
			Finite(value) => value.unpadded_decimal(),

			Maximum => Cow::from(b"max\n" as &[u8]),
		}
	}
}

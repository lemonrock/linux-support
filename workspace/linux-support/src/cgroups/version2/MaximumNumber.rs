// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum number.
///
/// Defaults to `Maximum`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaximumNumber
{
	/// A finite value.
	///
	/// Can be zero.
	Finite(usize),

	/// A system defined maximum.
	Maximum,
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for MaximumNumber
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::MaximumNumber::*;

		match self
		{
			Finite(value) => value.into_line_feed_terminated_byte_string(),

			Maximum => Cow::from(b"max\n" as &[u8]),
		}
	}
}

impl Default for MaximumNumber
{
	#[inline(always)]
	fn default() -> Self
	{
		MaximumNumber::Maximum
	}
}

impl Into<Option<usize>> for MaximumNumber
{
	#[inline(always)]
	fn into(self) -> Option<usize>
	{
		use self::MaximumNumber::*;

		match self
		{
			Finite(value) => Some(value),
			Maximum => None,
		}
	}
}

impl From<Option<usize>> for MaximumNumber
{
	#[inline(always)]
	fn from(value: Option<usize>) -> Self
	{
		use self::MaximumNumber::*;

		match value
		{
			Some(value) => Finite(value),
			None => Maximum,
		}
	}
}

impl MaximumNumber
{
	#[inline(always)]
	fn from_file(file_path: &Path) -> Result<Self, MaximumNumberParseError>
	{
		Self::from_file_contents(file_path.read_raw_without_line_feed()?)
	}

	#[inline(always)]
	fn from_file_contents(contents: Box<[u8]>) -> Result<Self, MaximumNumberParseError>
	{
		use self::MaximumNumber::*;
		if &contents[..] == b"max"
		{
			Ok(Maximum)
		}
		else
		{
			Ok(Finite(usize::parse_decimal_number(&contents)?))
		}
	}
}
// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// List parse error.
#[derive(Debug)]
pub enum ListParseError
{
	/// Contains an empty index or range.
	ContainsAnEmptyIndexOrRange,

	/// Could not parse index (not a string).
	CouldNotParseIndexAsNotAString
	{
		/// Description.
		description: &'static str,

		/// Unparsable index.
		unparsable_index: Box<[u8]>,

		/// Cause.
		cause: Utf8Error,
	},

	/// Could not parse index.
	CouldNotParseIndex
	{
		/// Description.
		description: &'static str,

		/// Unparsable index.
		unparsable_index: String,

		/// Cause.
		cause: ParseIntError,
	},

	/// Index out of range (eg only an u8 is acceptable but a value larger than 255 was parsed).
	IndexOutOfRange(BitSetAwareTryFromU16Error),

	/// Contains mis-sorted indices.
	ContainsMisSortedIndices
	{
		/// First part of index.
		first: u16,

		/// Minimum expected for next index.
		next_minimum_index_expected: u16
	},

	/// Range is not an ascending range with more than one element.
	RangeIsNotAnAscendingRangeWithMoreThanOneElement
	{
		/// First part of index.
		first: u16,

		/// Second part of index.
		second: u16
	},
}

impl Display for ListParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ListParseError as Debug>::fmt(self, f)
	}
}

impl error::Error for ListParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ListParseError::*;

		match self
		{
			&ContainsAnEmptyIndexOrRange => None,

			&CouldNotParseIndexAsNotAString { ref cause, .. } => Some(cause),

			&CouldNotParseIndex { ref cause, .. } => Some(cause),

			&IndexOutOfRange(ref cause) => Some(cause),

			&ContainsMisSortedIndices { .. } => None,

			&RangeIsNotAnAscendingRangeWithMoreThanOneElement { .. } => None,
		}
	}
}

impl From<BitSetAwareTryFromU16Error> for ListParseError
{
	#[inline(always)]
	fn from(error: BitSetAwareTryFromU16Error) -> Self
	{
		ListParseError::IndexOutOfRange(error)
	}
}

impl ListParseError
{
}

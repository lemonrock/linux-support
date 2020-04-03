// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug)]
pub enum ProcessStatusStatisticParseError
{
	/// No value.
	NoValue,

	/// Value was not preceeded by a horizontal tab.
	ValueNotPreceededByHorizontalTab,

	/// Length was invalid.
	InvalidLength,

	/// Ending was invalid.
	InvalidEnding,

	/// Separator of components of value was invalid in some way; either not present, the wrong kind or too few or too many.
	InvalidSeparator,

	/// Value was out-of-range, eg `2` for a `bool`.
	OutOfRange,

	/// Statistic was present more than once.
	DuplicatedStatistic,

	/// Statistic value sub-set had a duplicated entry.
	DuplicatedStatisticValue,

	/// Value was not a valid CPU or NUMA node list.
	NotAValidListOfCpusOrNumaNodes(ListParseError),

	/// Value was not a valid number.
	NotAValidNumber(ParseNumberError),

	/// Value was not a valid number.
	NotAValidBitSetAware(BitSetAwareTryFromU16Error),
}

impl Display for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessStatusStatisticParseError::*;

		match self
		{
			&NoValue => None,

			&ValueNotPreceededByHorizontalTab => None,

			&InvalidLength => None,

			&InvalidEnding => None,

			&InvalidSeparator => None,

			&OutOfRange => None,

			&DuplicatedStatistic => None,

			&DuplicatedStatisticValue => None,

			&NotAValidNumber(ref error) => Some(error),

			&NotAValidListOfCpusOrNumaNodes(ref error) => Some(error),

			&NotAValidBitSetAware(ref error) => Some(error),
		}
	}
}

impl From<ListParseError> for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn from(error: ListParseError) -> Self
	{
		ProcessStatusStatisticParseError::NotAValidListOfCpusOrNumaNodes(error)
	}
}

impl From<ParseNumberError> for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn from(error: ParseNumberError) -> Self
	{
		ProcessStatusStatisticParseError::NotAValidNumber(error)
	}
}

impl From<BitSetAwareTryFromU16Error> for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn from(error: BitSetAwareTryFromU16Error) -> Self
	{
		ProcessStatusStatisticParseError::NotAValidBitSetAware(error)
	}
}
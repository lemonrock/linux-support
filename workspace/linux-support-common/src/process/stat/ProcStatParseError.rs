// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug)]
pub enum ProcStatParseError
{
	/// Could not open file.
	CouldNotOpenFile(io::Error),

	/// Not enough fields.
	NotEnoughFields,

	/// Invalid number.
	ParseNumber(ParseNumberError),

	/// Invalid statistic.
	InvalidStatistic(ProcessStatusStatisticParseError),

	/// Not enough bytes.
	NotEnoughBytesForExecutabeName,

	/// Executable name does not start with `(`.
	ExecutableNameDoesNotStartWithOpenParenthesis,

	/// Executable name is empty.
	ExecutableNameIsEmpty,

	/// Executable name is not terminated with `( `.
	ExecutableNameIsUnterminated,
}

impl Display for ProcStatParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcStatParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcStatParseError::*;

		match self
		{
			&CouldNotOpenFile(ref error) => Some(error),

			&NotEnoughFields => None,

			&ParseNumber(ref error) => Some(error),

			&InvalidStatistic(ref error) => Some(error),

			&NotEnoughBytesForExecutabeName => None,

			&ExecutableNameDoesNotStartWithOpenParenthesis => None,

			&ExecutableNameIsEmpty => None,

			&ExecutableNameIsUnterminated => None,
		}
	}
}

impl From<io::Error> for ProcStatParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcStatParseError::CouldNotOpenFile(error)
	}
}

impl From<ParseNumberError> for ProcStatParseError
{
	#[inline(always)]
	fn from(error: ParseNumberError) -> Self
	{
		ProcStatParseError::ParseNumber(error)
	}
}

impl From<ProcessStatusStatisticParseError> for ProcStatParseError
{
	#[inline(always)]
	fn from(error: ProcessStatusStatisticParseError) -> Self
	{
		ProcStatParseError::InvalidStatistic(error)
	}
}

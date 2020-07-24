// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug)]
pub enum StatParseError
{
	/// Could not open file.
	CouldNotOpenFile(io::Error),

	/// Could not parse command name.
	CouldNotParseCommandName(ObjectNameFromBytesError),

	/// Not enough fields.
	NotEnoughFields
	{
		/// Field index.
		index: NonZeroU8,

		/// Field name.
		name: &'static str,
	},

	/// Invalid number.
	ParseNumber
	{
		/// Field index.
		index: NonZeroU8,

		/// Field name.
		name: &'static str,

		/// Cause.
		cause: ParseNumberError,
	},

	/// Invalid statistic.
	InvalidStatistic
	{
		/// Field index.
		index: NonZeroU8,

		/// Field name.
		name: &'static str,


		/// Cause.
		cause: StatusStatisticParseError,
	},

	/// Not enough bytes.
	NotEnoughBytesForExecutabeName,

	/// Executable name does not start with `(`.
	ExecutableNameDoesNotStartWithOpenParenthesis,

	/// Executable name is empty.
	ExecutableNameIsEmpty,

	/// Executable name is not terminated with `) `.
	ExecutableNameIsUnterminated,

	/// The time in jiffies before the next SIGALRM is sent to the process due to an interval timer.
	///
	/// Since Linux 2.6.17, this field is no longer maintained, and is hard coded as 0.
	///
	/// Known as `itrealvalue`.
	IntervalTimerRealValueWasNotZero
	{
		/// Non-zero value.
		value: NonZeroU64,
	}
}

impl Display for StatParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for StatParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::StatParseError::*;

		match self
		{
			&CouldNotOpenFile(ref cause) => Some(cause),

			&CouldNotParseCommandName(ref cause) => Some(cause),

			&NotEnoughFields { .. } => None,

			&ParseNumber { ref cause, .. } => Some(cause),

			&InvalidStatistic { ref cause, .. } => Some(cause),

			&NotEnoughBytesForExecutabeName => None,

			&ExecutableNameDoesNotStartWithOpenParenthesis => None,

			&ExecutableNameIsEmpty => None,

			&ExecutableNameIsUnterminated => None,

			&IntervalTimerRealValueWasNotZero { .. }=> None,
		}
	}
}

impl From<io::Error> for StatParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		StatParseError::CouldNotOpenFile(error)
	}
}

impl From<ObjectNameFromBytesError> for StatParseError
{
	#[inline(always)]
	fn from(error: ObjectNameFromBytesError) -> Self
	{
		StatParseError::CouldNotParseCommandName(error)
	}
}

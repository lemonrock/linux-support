// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors when parsing a statistic.
#[derive(Debug)]
pub enum StatisticsParseError
{
	/// Input error.
	Input(io::Error),

	/// Duplicate statistic name.
	DuplicateStatisticName
	{
		/// Name.
		name: Vec<u8>,
	},

	/// Missing statistic value.
	MissingStatisticValue
	{
		/// Name.
		name: &'static [u8],
	},

	/// Invalid statistic value.
	InvalidStatisticValue
	{
		/// Name.
		name: Vec<u8>,

		/// Value.
		value: Vec<u8>,

		/// Cause.
		cause: ParseNumberError,
	},
	
	/// Not a boolean.
	NotABooleanValue
	{
		/// Name.
		name: Vec<u8>,
	
		/// Value.
		value: usize,
	},
	
	/// Not a u32.
	NotAnU32Value
	{
		/// Name.
		name: Vec<u8>,
	
		/// Value.
		value: usize,
	},
	
	/// Missing statistic value.
	MissingStatistic
	{
		name: &'static [u8]
	},
	
	/// Missing one or more statistic value.
	MissingOneOrMoreStatistics,
}

impl Display for StatisticsParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for StatisticsParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::StatisticsParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&DuplicateStatisticName { .. } => None,

			&MissingStatisticValue { .. } => None,

			&InvalidStatisticValue { ref cause, .. } => Some(cause),

			&NotABooleanValue { .. } => Some(cause),

			&NotAnU32Value { .. } => Some(cause),

			&MissingStatistic { .. } => None,
			
			&MissingOneOrMoreStatistics => None,
		}
	}
}

impl From<io::Error> for StatisticsParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		StatisticsParseError::Input(value)
	}
}

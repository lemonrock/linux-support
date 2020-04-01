// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors possible when parsing memory statistics.
#[derive(Debug,)]
pub enum MemoryInformationParseError
{
	/// Could not open a file of memory statistics.
	CouldNotOpenFile(io::Error),

	/// Could not parse a memory statistic.
	NoValue
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,
	},

	/// Invalid unit.
	InvalidUnit
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,
	},

	/// Could not parse a memory statistic.
	TooMuchWhitespace
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,
	},

	/// Could not parse a memory statistic.
	BadValue
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// Value.
		cause: ParseNumberError,
	},

	/// Could not parse a memory statistic because it was a duplicate.
	DuplicateMemoryInformation
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,
	},
}

impl Display for MemoryInformationParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<MemoryInformationParseError as Debug>::fmt(self, f)
	}
}

impl error::Error for MemoryInformationParseError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::MemoryInformationParseError::*;

		match self
		{
			&CouldNotOpenFile(ref cause) => Some(cause),

			&NoValue { .. } => None,

			&InvalidUnit { .. } => None,

			&TooMuchWhitespace { .. } => None,

			&BadValue { ref cause, .. } => Some(cause),

			&DuplicateMemoryInformation { .. } => None,
		}
	}
}

impl From<io::Error> for MemoryInformationParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		MemoryInformationParseError::CouldNotOpenFile(error)
	}
}

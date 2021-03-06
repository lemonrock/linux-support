// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parsing of a `/proc/<pid>/status` file failed.
#[derive(Debug)]
pub enum StatusFileParseError
{
	/// Could not open a file.
	CouldNotOpenFile(io::Error),

	/// Could not parse a line of data.
	CouldNotParseLine
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Cause.
		cause: StatusStatisticParseError,
	},

	/// Missing a required field (field name is tuple value).
	MissingRequiredField(&'static str),
}

impl Display for StatusFileParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for StatusFileParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::StatusFileParseError::*;

		match self
		{
			&CouldNotOpenFile(ref error) => Some(error),

			&CouldNotParseLine { ref cause, .. } => Some(cause),

			&MissingRequiredField(..) => None,
		}
	}
}

impl From<io::Error> for StatusFileParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		StatusFileParseError::CouldNotOpenFile(error)
	}
}

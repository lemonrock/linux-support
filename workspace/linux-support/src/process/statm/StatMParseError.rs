// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parsing of a process statm file failed.
#[derive(Debug)]
pub enum StatMParseError
{
	/// Could not open a file.
	CouldNotOpenFile(io::Error),

	/// Missing a field.
	MissingField
	{
		/// One-based column index.
		index: NonZeroU8,

		/// Field name as per `http://man7.org/linux/man-pages/man5/proc.5.html`.
		name: &'static str,
	},

	/// Parsing of field number failed.
	ParseNumber
	{
		/// One-based column index.
		index: NonZeroU8,

		/// Field name as per `http://man7.org/linux/man-pages/man5/proc.5.html`.
		name: &'static str,

		/// Cause.
		cause: ParseNumberError,
	},

	/// Field which should always be zero wasn't.
	FieldWasNotZero
	{
		/// One-based column index.
		index: NonZeroU8,

		/// Field name as per `http://man7.org/linux/man-pages/man5/proc.5.html`.
		name: &'static str,
	},
}

impl Display for StatMParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for StatMParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::StatMParseError::*;

		match self
		{
			&CouldNotOpenFile(ref cause) => Some(cause),

			&MissingField { .. } => None,

			&ParseNumber { ref cause, .. } => Some(cause),

			&FieldWasNotZero { .. } => None,
		}
	}
}

impl From<io::Error> for StatMParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		StatMParseError::CouldNotOpenFile(error)
	}
}

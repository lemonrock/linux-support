// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors when parsing a maximum number.
#[derive(Debug)]
pub enum MaximumNumberParseError
{
	/// Input error.
	Input(io::Error),

	/// Could not parse value.
	WasNotMaximumOrAFiniteInteger(ParseNumberError),
}

impl Display for MaximumNumberParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for MaximumNumberParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::MaximumNumberParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&WasNotMaximumOrAFiniteInteger(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for MaximumNumberParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		MaximumNumberParseError::Input(value)
	}
}

impl From<ParseNumberError> for MaximumNumberParseError
{
	#[inline(always)]
	fn from(value: ParseNumberError) -> Self
	{
		MaximumNumberParseError::WasNotMaximumOrAFiniteInteger(value)
	}
}

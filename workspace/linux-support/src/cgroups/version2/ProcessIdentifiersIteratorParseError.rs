// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parse error of a process identifier.
#[derive(Debug)]
pub enum ProcessIdentifiersIteratorParseError
{
	/// Input error.
	Input(io::Error),

	/// Could not parse process identifier.
	CouldNotParseProcessIdentifier(ParseNumberError),
}

impl Display for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::ProcessIdentifiersIteratorParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&CouldNotParseProcessIdentifier(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcessIdentifiersIteratorParseError::Input(error)
	}
}

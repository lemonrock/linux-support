// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors when parsing a maximum number.
#[derive(Debug)]
pub enum ControllersFileError
{
	/// Input error.
	Input(io::Error),

	/// Invalid controller.
	Parse(ParseControllerError),

	/// Duplicate controller.
	DuplicateController(Controller),
}

impl Display for ControllersFileError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for ControllersFileError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::ControllersFileError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&Parse(..) => None,

			&DuplicateController(..) => None,
		}
	}
}

impl From<io::Error> for ControllersFileError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ControllersFileError::Input(value)
	}
}

impl From<ParseControllerError> for ControllersFileError
{
	#[inline(always)]
	fn from(value: ParseControllerError) -> Self
	{
		ControllersFileError::Parse(value)
	}
}

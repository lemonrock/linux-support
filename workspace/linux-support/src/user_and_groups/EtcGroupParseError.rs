// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors parsing `/etc/passwd`
#[derive(Debug)]
pub enum EtcGroupParseError
{
	/// Input/Output error.
	Io(io::Error),

	#[allow(missing_docs)]
	MissingLastLineFeed,

	#[allow(missing_docs)]
	MissingNameField,

	#[allow(missing_docs)]
	MissingPasswordField,

	/// The password field should just be `x`.
	PasswordFieldIsInvalid,

	#[allow(missing_docs)]
	MissingGidField,

	#[allow(missing_docs)]
	ParseNumberGid(ParseNumberError),

	#[allow(missing_docs)]
	MissingUserListField,
}

impl Display for EtcGroupParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for EtcGroupParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::EtcGroupParseError::*;

		match self
		{
			&Io(ref cause) => Some(cause),

			&MissingLastLineFeed => None,

			&MissingNameField => None,

			&MissingPasswordField => None,

			&PasswordFieldIsInvalid => None,

			&MissingGidField => None,

			&ParseNumberGid(ref cause) => Some(cause),

			&MissingUserListField => None,
		}
	}
}

impl From<io::Error> for EtcGroupParseError
{
	#[inline(always)]
	fn from(cause: io::Error) -> Self
	{
		EtcGroupParseError::Io(cause)
	}
}

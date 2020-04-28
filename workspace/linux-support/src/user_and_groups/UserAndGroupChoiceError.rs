// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors finding users or groups.
#[derive(Debug)]
pub enum UserAndGroupChoiceError
{
	#[allow(missing_docs)]
	EtcPasswdParse(EtcPasswdParseError),

	#[allow(missing_docs)]
	EtcGroupParse(EtcGroupParseError),

	/// User name not present in `/etc/passwd`.
	UserNameNotPresentInEtcPasswd,

	/// User identifier not present in `/etc/passwd`.
	UserIdentifierNotPresentInEtcPasswd,

	/// Group name not present in `/etc/group`.
	GroupNameNotPresentInEtcGroup,

	#[allow(missing_docs)]
	DuplicateGroupIdentifier(GroupIdentifier),

	#[allow(missing_docs)]
	TooManySupplementaryGroups(usize),
}

impl Display for UserAndGroupChoiceError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for UserAndGroupChoiceError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::UserAndGroupChoiceError::*;

		match self
		{
			&EtcPasswdParse(ref cause) => Some(cause),

			&EtcGroupParse(ref cause) => Some(cause),

			&UserNameNotPresentInEtcPasswd => None,

			&UserIdentifierNotPresentInEtcPasswd => None,

			&GroupNameNotPresentInEtcGroup => None,

			&DuplicateGroupIdentifier(..) => None,

			&TooManySupplementaryGroups(..) => None,
		}
	}
}

impl From<EtcPasswdParseError> for UserAndGroupChoiceError
{
	#[inline(always)]
	fn from(cause: EtcPasswdParseError) -> Self
	{
		UserAndGroupChoiceError::EtcPasswdParse(cause)
	}
}

impl From<EtcGroupParseError> for UserAndGroupChoiceError
{
	#[inline(always)]
	fn from(cause: EtcGroupParseError) -> Self
	{
		UserAndGroupChoiceError::EtcGroupParse(cause)
	}
}

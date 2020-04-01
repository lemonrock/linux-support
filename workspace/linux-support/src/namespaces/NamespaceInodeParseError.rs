// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Namespace inode parse error.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum NamespaceInodeParseError
{
	Input(io::Error),

	NamespaceLinkIsNotUtf8,

	NamespaceLinkIsTooShort,

	NamespaceLinkDoesNotStartWithNamespace,

	NamespaceLinkDoesNotContainColonSquareBracket,

	NamespaceLinkDoesNotEndWithSquareBracket,

	CouldNotParseInodeValue(ParseIntError),
}

impl Display for NamespaceInodeParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NamespaceInodeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NamespaceInodeParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&NamespaceLinkIsNotUtf8 => None,

			&NamespaceLinkIsTooShort => None,

			&NamespaceLinkDoesNotStartWithNamespace => None,

			&NamespaceLinkDoesNotContainColonSquareBracket => None,

			&NamespaceLinkDoesNotEndWithSquareBracket => None,

			&CouldNotParseInodeValue(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for NamespaceInodeParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		NamespaceInodeParseError::Input(value)
	}
}

impl From<ParseIntError> for NamespaceInodeParseError
{
	#[inline(always)]
	fn from(value: ParseIntError) -> Self
	{
		NamespaceInodeParseError::CouldNotParseInodeValue(value)
	}
}

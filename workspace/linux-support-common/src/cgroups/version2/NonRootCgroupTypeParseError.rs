// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug)]
pub enum NonRootCgroupTypeParseError
{
	/// Input error.
	Input(io::Error),

	/// Does not end with line feed.
	DoesNotEndWithLineFeed,

	/// Invalid type name.
	InvalidTypeName
	{
		/// Name.
		name: String,
	},
}

impl Display for NonRootCgroupTypeParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for NonRootCgroupTypeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::NonRootCgroupTypeParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&DoesNotEndWithLineFeed => None,

			&InvalidTypeName { .. } => None,
		}
	}
}

impl From<io::Error> for NonRootCgroupTypeParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		NonRootCgroupTypeParseError::Input(value)
	}
}

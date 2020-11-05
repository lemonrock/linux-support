// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned character string.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OwnedCharacterString(Box<[u8]>);

impl HasTypeEquality for OwnedCharacterString
{
	type TypeEquality = OwnedTypeEquality;
}

impl CharacterString for OwnedCharacterString
{
}

impl<'message, 'a> From<&'a ParsedCharacterString<'message>> for OwnedCharacterString
{
	#[inline(always)]
	fn from(value: &'a ParsedCharacterString<'message>) -> Self
	{
		Self(value.0.to_vec().into_boxed_slice())
	}
}

impl<'message> From<ParsedCharacterString<'message>> for OwnedCharacterString
{
	#[inline(always)]
	fn from(value: ParsedCharacterString<'message>) -> Self
	{
		Self::from(&value)
	}
}

impl Deref for OwnedCharacterString
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

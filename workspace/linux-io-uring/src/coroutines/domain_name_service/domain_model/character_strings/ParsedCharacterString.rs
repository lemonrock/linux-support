// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parsed character string.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ParsedCharacterString<'message>(&'message [u8]);

impl<'message> HasTypeEquality for ParsedCharacterString<'message>
{
	type TypeEquality = ParsedTypeEquality;
}

impl<'message> CharacterString for ParsedCharacterString<'message>
{
}

impl<'message> Deref for ParsedCharacterString<'message>
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

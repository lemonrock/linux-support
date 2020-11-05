// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Arbitrary bytes parsed from a message.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ParsedBytes<'message>(&'message [u8]);

impl<'message> HasTypeEquality for ParsedBytes<'message>
{
	type TypeEquality = ParsedTypeEquality;
}

impl<'message> OwnedOrParsedBytes for ParsedBytes<'message>
{
}

impl<'message> Deref for ParsedBytes<'message>
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl<'message> From<&'message [u8]> for ParsedBytes<'message>
{
	#[inline(always)]
	fn from(value: &'message [u8]) -> Self
	{
		Self(value)
	}
}

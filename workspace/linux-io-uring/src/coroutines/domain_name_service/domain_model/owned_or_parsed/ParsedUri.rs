// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// URI parsed from a message.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ParsedUri<'message>(Uri<'message>);

impl<'message> HasTypeEquality for ParsedUri<'message>
{
	type TypeEquality = ParsedTypeEquality;
}

impl<'message> OwnedOrParsedUri for ParsedUri<'message>
{
}

impl<'message> Deref for ParsedUri<'message>
{
	type Target = Uri<'message>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'message> From<Uri<'message>> for ParsedUri<'message>
{
	#[inline(always)]
	fn from(value: Uri<'message>) -> Self
	{
		Self(value)
	}
}

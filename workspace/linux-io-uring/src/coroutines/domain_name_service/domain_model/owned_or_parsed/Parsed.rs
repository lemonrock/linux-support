// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Arbitrary data parsed from a message.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Parsed<'message, OOPV>(&'message OOPV);

impl<'message, OOPV: Clone> HasTypeEquality for Parsed<'message, OOPV>
{
	type TypeEquality = ParsedTypeEquality;
}

impl<'message, OOPV> Deref for Parsed<'message, OOPV>
{
	type Target = OOPV;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl<'message, OOPV> From<&'message OOPV> for Parsed<'message, OOPV>
{
	#[inline(always)]
	fn from(value: &'message OOPV) -> Self
	{
		Self(value)
	}
}

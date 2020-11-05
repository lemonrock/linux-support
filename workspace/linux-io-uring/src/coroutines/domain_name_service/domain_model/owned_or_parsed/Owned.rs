// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Arbitrary data.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Owned<OOPV>(OOPV);

impl<OOPV: Clone> HasTypeEquality for Owned<OOPV>
{
	type TypeEquality = ParsedTypeEquality;
}

impl<OOPV: Clone> OwnedOrParsed<OOPV> for Owned<OOPV>
{
}

impl<OOPV> Deref for Owned<OOPV>
{
	type Target = OOPV;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'message, 'a, OOPV: Clone> From<&'a Parsed<'message, OOPV>> for Owned<OOPV>
{
	#[inline(always)]
	fn from(value: &'a Parsed<'message, OOPV>) -> Self
	{
		Self(value.0.clone())
	}
}

impl<'message, OOPV: Clone> From<Parsed<'message, OOPV>> for Owned<OOPV>
{
	#[inline(always)]
	fn from(value: Parsed<'message, OOPV>) -> Self
	{
		Self::from(&value)
	}
}

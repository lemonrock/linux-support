// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name<'de>(pub Cow<'de, str>);

impl<'de> From<String> for Name<'de>
{
	#[inline(always)]
	fn from(value: String) -> Self
	{
		Self(Cow::from(value))
	}
}

impl<'de> From<&'de str> for Name<'de>
{
	#[inline(always)]
	fn from(value: &'de str) -> Self
	{
		Self(Cow::from(value))
	}
}

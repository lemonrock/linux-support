// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Name<'name>(pub Cow<'name, str>);

impl<'name> Deref for Name<'name>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'name> Into<String> for Name<'name>
{
	#[inline(always)]
	fn into(self) -> String
	{
		self.0.to_string()
	}
}

impl<'name> From<Cow<'name, str>> for Name<'name>
{
	#[inline(always)]
	fn from(value: Cow<'name, str>) -> Self
	{
		Self(value)
	}
}

impl<'name> From<String> for Name<'name>
{
	#[inline(always)]
	fn from(value: String) -> Self
	{
		Self(Cow::from(value))
	}
}

impl<'name> From<&'name str> for Name<'name>
{
	#[inline(always)]
	fn from(value: &'name str) -> Self
	{
		Self(Cow::from(value))
	}
}

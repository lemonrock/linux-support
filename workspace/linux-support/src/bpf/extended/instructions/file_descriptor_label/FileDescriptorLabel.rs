// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A map file descriptor label.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct FileDescriptorLabel<'name>(pub Name<'name>);

impl<'name> Deref for FileDescriptorLabel<'name>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'name> Into<String> for FileDescriptorLabel<'name>
{
	#[inline(always)]
	fn into(self) -> String
	{
		self.0.into()
	}
}

impl<'name, V: Into<Name<'name>>> From<V> for FileDescriptorLabel<'name>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}

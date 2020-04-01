// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a network interface name, such as `eth0`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct NetworkInterfaceName(String);

impl From<String> for NetworkInterfaceName
{
	fn from(value: String) -> Self
	{
		Self(value)
	}
}

impl<'a> From<&'a str> for NetworkInterfaceName
{
	fn from(value: &'a str) -> Self
	{
		Self(value.to_string())
	}
}

impl Deref for NetworkInterfaceName
{
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

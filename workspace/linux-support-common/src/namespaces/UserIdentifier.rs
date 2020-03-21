// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A user identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct UserIdentifier(uid_t);

impl UserOrGroupIdentifier for UserIdentifier
{
	const Zero: Self = UserIdentifier(0);

	const FileName: &'static str = "uid_map";

	#[inline(always)]
	fn current() -> Self
	{
		Self(unsafe { getuid() })
	}

	#[inline(always)]
	fn get(self) -> u32
	{
		self.0
	}
}

impl From<uid_t> for UserIdentifier
{
	#[inline(always)]
	fn from(value: uid_t) -> Self
	{
		Self(value)
	}
}

impl Into<uid_t> for UserIdentifier
{
	#[inline(always)]
	fn into(self) -> uid_t
	{
		self.0
	}
}

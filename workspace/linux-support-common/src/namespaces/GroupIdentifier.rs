// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A group identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct GroupIdentifier(gid_t);

impl UserOrGroupIdentifier for GroupIdentifier
{
	const Zero: Self = GroupIdentifier(0);

	const FileName: &'static str = "gid_map";

	#[inline(always)]
	fn current() -> Self
	{
		Self(unsafe { getgid() })
	}

	#[inline(always)]
	fn get(self) -> u32
	{
		self.0
	}
}

impl From<gid_t> for GroupIdentifier
{
	#[inline(always)]
	fn from(value: gid_t) -> Self
	{
		Self(value)
	}
}

impl Into<gid_t> for GroupIdentifier
{
	#[inline(always)]
	fn into(self) -> gid_t
	{
		self.0
	}
}

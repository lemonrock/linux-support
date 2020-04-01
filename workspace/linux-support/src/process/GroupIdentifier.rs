// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A group identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GroupIdentifier(gid_t);

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

impl FromBytes for GroupIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		Ok(Self(gid_t::parse_decimal_number(value)?))
	}
}

impl Default for GroupIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::current()
	}
}

impl UserOrGroupIdentifier for GroupIdentifier
{
	const Zero: Self = GroupIdentifier(0);

	const FileName: &'static str = "gid_map";

	#[inline(always)]
	fn current() -> Self
	{
		Self(unsafe { getgid() })
	}
}

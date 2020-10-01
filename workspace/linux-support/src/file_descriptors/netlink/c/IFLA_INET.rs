// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Nested attribute values for `IFLA_AF_SPEC::IFLA_AF_SPEC_INET`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA_INET
{
	#[allow(dead_code)]
	IFLA_INET_UNSPEC = 0,
	
	#[allow(dead_code)]
	IFLA_INET_CONF = 1,
}

impl From<u16> for IFLA_INET
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IFLA_INET
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFLA_INET
{
	const __IFLA_INET_MAX: u16 = 2;
	
	#[allow(dead_code)] pub(crate) const IFLA_INET_MAX: Self = unsafe { transmute(Self::__IFLA_INET_MAX - 1) };
}

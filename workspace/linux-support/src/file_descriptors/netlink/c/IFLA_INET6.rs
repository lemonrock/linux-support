// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Nested attribute values for `IFLA_AF_SPEC::IFLA_AF_SPEC_INET6`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA_INET6
{
	#[allow(dead_code)]
	IFLA_INET6_UNSPEC = 0,
	
	#[allow(dead_code)]
	IFLA_INET6_FLAGS = 1,
	
	#[allow(dead_code)]
	IFLA_INET6_CONF = 2,
	
	#[allow(dead_code)]
	IFLA_INET6_STATS = 3,
	
	#[allow(dead_code)]
	IFLA_INET6_MCAST = 4,
	
	#[allow(dead_code)]
	IFLA_INET6_CACHEINFO = 5,
	
	#[allow(dead_code)]
	IFLA_INET6_ICMP6STATS = 6,
	
	#[allow(dead_code)]
	IFLA_INET6_TOKEN = 7,
	
	#[allow(dead_code)]
	IFLA_INET6_ADDR_GEN_MODE = 8,
}

impl From<u16> for IFLA_INET6
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IFLA_INET6
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFLA_INET6
{
	const __IFLA_INET6_MAX: u16 = 9;
	
	#[allow(dead_code)] pub(crate) const IFLA_INET_MAX: Self = unsafe { transmute(Self::__IFLA_INET6_MAX - 1) };
}

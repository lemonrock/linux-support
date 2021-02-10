// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Nested attribute values for `IFLA::IFLA_XDP`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA_XDP
{
	#[allow(dead_code)]
	IFLA_XDP_UNSPEC = 0,
	
	IFLA_XDP_FD = 1,
	
	IFLA_XDP_ATTACHED = 2,
	
	IFLA_XDP_FLAGS = 3,
	
	#[allow(dead_code)]
	IFLA_XDP_PROG_ID = 4,
	
	#[allow(dead_code)]
	IFLA_XDP_DRV_PROG_ID = 5,
	
	#[allow(dead_code)]
	IFLA_XDP_SKB_PROG_ID = 6,
	
	#[allow(dead_code)]
	IFLA_XDP_HW_PROG_ID = 7,
	
	IFLA_XDP_EXPECTED_FD = 8,
}

impl From<u16> for IFLA_XDP
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IFLA_XDP
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFLA_XDP
{
	const __IFLA_XDP_MAX: u16 = 9;
	
	#[allow(dead_code)] pub(crate) const IFLA_XDP_MAX: Self = unsafe { transmute(Self::__IFLA_XDP_MAX - 1) };
}

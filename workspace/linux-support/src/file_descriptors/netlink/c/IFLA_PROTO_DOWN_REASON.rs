// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Nested attribute values for `IFLA::IFLA_PROTO_DOWN_REASON`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA_PROTO_DOWN_REASON
{
	#[allow(dead_code)] IFLA_PROTO_DOWN_REASON_UNSPEC = 0,
	
	/// Mask for reason value.
	///
	/// `u32`; does not seem to be used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_PROTO_DOWN_REASON_MASK = 1,
	
	/// Reason value.
	///
	/// `NonZeroU32`.
	#[allow(dead_code)]
	IFLA_PROTO_DOWN_REASON_VALUE = 2,
}

impl From<u16> for IFLA_PROTO_DOWN_REASON
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IFLA_PROTO_DOWN_REASON
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFLA_PROTO_DOWN_REASON
{
	const __IFLA_PROTO_DOWN_REASON_CNT: u16 = 3;
	
	#[allow(dead_code)] pub(crate) const IFLA_PROTO_DOWN_REASON_MAX: Self = unsafe { transmute(Self::__IFLA_PROTO_DOWN_REASON_CNT - 1) };
}

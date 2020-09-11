// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWADDR`, `RTM_DELADDR` and `RTM_GETADDR`.
///
/// See Linux header `if_addr.h`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFA
{
	IFA_UNSPEC = 0,
	
	/// `IFA_ADDRESS` is a prefix address, rather than local interface address.
	/// It makes no difference for normally configured broadcast interfaces, but for point-to-point `IFA_ADDRESS` is destination address, local address is supplied in the `IFA_LOCAL` attribute.
	IFA_ADDRESS = 1,
	
	IFA_LOCAL = 2,
	
	IFA_LABEL = 3,
	
	IFA_BROADCAST = 4,
	
	IFA_ANYCAST = 5,
	
	IFA_CACHEINFO = 6,
	
	IFA_MULTICAST = 7,
	
	/// `IFA_FLAGS` is a `u32` attribute that replaces the `u8` field `ifa_flags` in `ifaddrmsg`.
	/// If present, the field `ifa_flags` in the struct `ifaddrmsg` will be ignored by the Linux kernel.
	IFA_FLAGS = 8,
	
	/// Priority or metric for prefix route.
	///
	/// `u32`.
	IFA_RT_PRIORITY = 9,
	
	IFA_TARGET_NETNSID = 10,
}

impl From<u16> for IFA
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IFA
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFA
{
	const __IFA_MAX: u16 = IFA::IFA_TARGET_NETNSID as u16 + 1;
	
	pub(crate) const IFA_MAX: Self = unsafe { transmute(Self::__IFA_MAX - 1) };
}

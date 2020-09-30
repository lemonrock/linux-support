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
	#[allow(dead_code)]
	IFA_UNSPEC = 0,
	
	/// `IFA_ADDRESS` is a prefix address, rather than local interface address.
	/// It makes no difference for normally configured broadcast interfaces, but for point-to-point `IFA_ADDRESS` is destination address, local address is supplied in the `IFA_LOCAL` attribute.
	///
	/// Valid for get address for:-
	/// * IPv4
	/// * IPv6
	///
	/// Valid for delete address for:-
	// * IPv4.
	// * IPv6.
	#[allow(dead_code)]
	IFA_ADDRESS = 1,
	
	///
	/// Valid for get address for:-
	/// * IPv4
	/// * IPv6
	///
	/// Valid for delete address for:-
	// * IPv4.
	// * IPv6.
	IFA_LOCAL = 2,
	
	///
	/// Valid for get address for:-
	/// * IPv4
	///
	/// Valid for delete address for:-
	// * IPv4.
	#[allow(dead_code)]
	IFA_LABEL = 3,
	
	///
	/// Valid for get address for:-
	/// * IPv4
	///
	/// Valid for delete address for:-
	// * IPv4.
	IFA_BROADCAST = 4,
	
	/// Only valid for `RTM_GETANYCAST` for Internet Protocol version 6.
	///
	/// See `inet6_fill_ifacaddr()` in Linux source `addrconf.c`.
	///
	/// If present, then the only other attributes present will be `IFA_CACHEINFO` and optionally `IFA_TARGET_NETNSID`.
	IFA_ANYCAST = 5,
	
	/// Valid for get address for:-
	/// * IPv4
	///
	/// Valid for delete address for:-
	// * IPv4.
	// * IPv6.
	#[allow(dead_code)]
	IFA_CACHEINFO = 6,
	
	/// Only valid for `RTM_GETMULTICAST` for Internet Protocol version 6.
	///
	/// See `inet6_fill_ifmcaddr()` in Linux source `addrconf.c`.
	///
	/// If present, then the only other attributes present will be `IFA_CACHEINFO` and optionally `IFA_TARGET_NETNSID`.
	IFA_MULTICAST = 7,
	
	/// `IFA_FLAGS` is a `u32` attribute that replaces the `u8` field `ifa_flags` in `ifaddrmsg`.
	/// If present, the field `ifa_flags` in the struct `ifaddrmsg` will be ignored by the Linux kernel.
	///
	/// Valid for get address for:-
	/// * IPv4
	///
	/// Valid for delete address for:-
	// * IPv4.
	// * IPv6.
	IFA_FLAGS = 8,
	
	/// Priority or metric for prefix route.
	///
	/// `NonZeroU32`.
	///
	/// Valid for get address for:-
	/// * IPv4
	/// * IPv6
	///
	/// Valid for delete address for:-
	// * IPv4.
	// * IPv6.
	#[allow(dead_code)]
	IFA_RT_PRIORITY = 9,
	
	/// Valid for get address for:-
	/// * IPv4
	/// * IPv6
	///
	/// Valid for delete address for:-
	// * IPv4.
	// * IPv6.
	#[allow(dead_code)]
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
	
	#[allow(dead_code)]
	pub(crate) const IFA_MAX: Self = unsafe { transmute(Self::__IFA_MAX - 1) };
}

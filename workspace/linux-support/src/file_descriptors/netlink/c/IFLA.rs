// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWLINK`, `RTM_DELLINK` and `RTM_GETLINK`.
///
/// See Linux header `if_link.h`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA
{
	#[allow(dead_code)]
	IFLA_UNSPEC = 0,
	
	#[allow(dead_code)]
	IFLA_ADDRESS = 1,
	
	IFLA_BROADCAST = 2,
	
	#[allow(dead_code)]
	IFLA_IFNAME = 3,
	
	#[allow(dead_code)]
	IFLA_MTU = 4,
	
	#[allow(dead_code)]
	IFLA_LINK = 5,
	
	#[allow(dead_code)]
	IFLA_QDISC = 6,
	
	#[allow(dead_code)]
	IFLA_STATS = 7,
	
	#[allow(dead_code)]
	IFLA_COST = 8,
	
	#[allow(dead_code)]
	IFLA_PRIORITY = 9,
	
	#[allow(dead_code)]
	IFLA_MASTER = 10,
	
	/// Wireless Extension event - see `wireless.h`.
	#[allow(dead_code)]
	IFLA_WIRELESS = 11,
	
	/// Protocol specific information for a link.
	#[allow(dead_code)]
	IFLA_PROTINFO = 12,
	
	#[allow(dead_code)]
	IFLA_TXQLEN = 13,
	
	#[allow(dead_code)]
	IFLA_MAP = 14,
	
	#[allow(dead_code)]
	IFLA_WEIGHT = 15,
	
	IFLA_OPERSTATE = 16,
	
	IFLA_LINKMODE = 17,
	
	#[allow(dead_code)]
	IFLA_LINKINFO = 18,
	
	#[allow(dead_code)]
	IFLA_NET_NS_PID = 19,
	
	#[allow(dead_code)]
	IFLA_IFALIAS = 20,
	
	/// Number of PCI Virtual Functions if device is SR-IOV Physical Function.
	#[allow(dead_code)]
	IFLA_NUM_VF = 21,
	
	#[allow(dead_code)]
	IFLA_VFINFO_LIST = 22,
	
	#[allow(dead_code)]
	IFLA_STATS64 = 23,
	
	#[allow(dead_code)]
	IFLA_VF_PORTS = 24,
	
	#[allow(dead_code)]
	IFLA_PORT_SELF = 25,
	
	#[allow(dead_code)]
	IFLA_AF_SPEC = 26,
	
	/// Group the device belongs to.
	#[allow(dead_code)]
	IFLA_GROUP = 27,
	
	#[allow(dead_code)]
	IFLA_NET_NS_FD = 28,
	
	/// "Extended infomation mask, PCI Virtual Functions, etc".
	#[allow(dead_code)]
	IFLA_EXT_MASK = 29,
	
	/// Promiscuity count: > 0 means acts PROMISC.
	///
	/// Refers to ethernet promiscuity.
	#[allow(dead_code)]
	IFLA_PROMISCUITY = 30,
	
	#[allow(dead_code)]
	IFLA_NUM_TX_QUEUES = 31,
	
	#[allow(dead_code)]
	IFLA_NUM_RX_QUEUES = 32,
	
	#[allow(dead_code)]
	IFLA_CARRIER = 33,
	
	#[allow(dead_code)]
	IFLA_PHYS_PORT_ID = 34,
	
	#[allow(dead_code)]
	IFLA_CARRIER_CHANGES = 35,
	
	#[allow(dead_code)]
	IFLA_PHYS_SWITCH_ID = 36,
	
	#[allow(dead_code)]
	IFLA_LINK_NETNSID = 37,
	
	#[allow(dead_code)]
	IFLA_PHYS_PORT_NAME = 38,
	
	#[allow(dead_code)]
	IFLA_PROTO_DOWN = 39,
	
	#[allow(dead_code)]
	IFLA_GSO_MAX_SEGS = 40,
	
	#[allow(dead_code)]
	IFLA_GSO_MAX_SIZE = 41,
	
	#[allow(dead_code)]
	IFLA_PAD = 42,
	
	IFLA_XDP = 43,
	
	#[allow(dead_code)]
	IFLA_EVENT = 44,
	
	#[allow(dead_code)]
	IFLA_NEW_NETNSID = 45,
	
	#[allow(dead_code)]
	IFLA_IF_NETNSID = 46,
	
	#[allow(dead_code)]
	IFLA_CARRIER_UP_COUNT = 47,
	
	#[allow(dead_code)]
	IFLA_CARRIER_DOWN_COUNT = 48,
	
	#[allow(dead_code)]
	IFLA_NEW_IFINDEX = 49,
	
	#[allow(dead_code)]
	IFLA_MIN_MTU = 50,
	
	#[allow(dead_code)]
	IFLA_MAX_MTU = 51,
	
	#[allow(dead_code)]
	IFLA_PROP_LIST = 52,
	
	/// Alternative network interface name.
	#[allow(dead_code)]
	IFLA_ALT_IFNAME = 53,
	
	IFLA_PERM_ADDRESS = 54,
}

impl From<u16> for IFLA
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IFLA
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFLA
{
	const __IFLA_MAX: u16 = 55;
	
	/// New alias.
	pub(crate) const IFLA_TARGET_NETNSID: Self = IFLA::IFLA_IF_NETNSID;
	
	#[allow(dead_code)]
	pub(crate) const IFLA_MAX: Self = unsafe { transmute(Self::__IFLA_MAX - 1) };
}

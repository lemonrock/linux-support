// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWLINK`, `RTM_DELLINK` and `RTM_GETLINK`.
///
/// See Linux header `if_link.h`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA
{
	IFLA_UNSPEC = 0,
	
	IFLA_ADDRESS = 1,
	
	IFLA_BROADCAST = 2,
	
	IFLA_IFNAME = 3,
	
	IFLA_MTU = 4,
	
	IFLA_LINK = 5,
	
	IFLA_QDISC = 6,
	
	IFLA_STATS = 7,
	
	IFLA_COST = 8,
	
	IFLA_PRIORITY = 9,
	
	IFLA_MASTER = 10,
	
	/// Wireless Extension event - see `wireless.h`.
	IFLA_WIRELESS = 11,
	
	/// Protocol specific information for a link.
	IFLA_PROTINFO = 12,
	
	IFLA_TXQLEN = 13,
	
	IFLA_MAP = 14,
	
	IFLA_WEIGHT = 15,
	
	IFLA_OPERSTATE = 16,
	
	IFLA_LINKMODE = 17,
	
	IFLA_LINKINFO = 18,
	
	IFLA_NET_NS_PID = 19,
	
	IFLA_IFALIAS = 20,
	
	/// Number of PCI Virtual Functions if device is SR-IOV Physical Function.
	IFLA_NUM_VF = 21,
	
	IFLA_VFINFO_LIST = 22,
	
	IFLA_STATS64 = 23,
	
	IFLA_VF_PORTS = 24,
	
	IFLA_PORT_SELF = 25,
	
	IFLA_AF_SPEC = 26,
	
	/// Group the device belongs to.
	IFLA_GROUP = 27,
	
	IFLA_NET_NS_FD = 28,
	
	/// "Extended infomation mask, PCI Virtual Functions, etc".
	IFLA_EXT_MASK = 29,
	
	/// Promiscuity count: > 0 means acts PROMISC.
	///
	/// Refers to ethernet promiscuity.
	IFLA_PROMISCUITY = 30,
	
	IFLA_NUM_TX_QUEUES = 31,
	
	IFLA_NUM_RX_QUEUES = 32,
	
	IFLA_CARRIER = 33,
	
	IFLA_PHYS_PORT_ID = 34,
	
	IFLA_CARRIER_CHANGES = 35,
	
	IFLA_PHYS_SWITCH_ID = 36,
	
	IFLA_LINK_NETNSID = 37,
	
	IFLA_PHYS_PORT_NAME = 38,
	
	IFLA_PROTO_DOWN = 39,
	
	IFLA_GSO_MAX_SEGS = 40,
	
	IFLA_GSO_MAX_SIZE = 41,
	
	IFLA_PAD = 42,
	
	IFLA_XDP = 43,
	
	IFLA_EVENT = 44,
	
	IFLA_NEW_NETNSID = 45,
	
	IFLA_IF_NETNSID = 46,
	
	IFLA_CARRIER_UP_COUNT = 47,
	
	IFLA_CARRIER_DOWN_COUNT = 48,
	
	IFLA_NEW_IFINDEX = 49,
	
	IFLA_MIN_MTU = 50,
	
	IFLA_MAX_MTU = 51,
	
	IFLA_PROP_LIST = 52,
	
	/// Alternative network interface name.
	IFLA_ALT_IFNAME = 53,
	
	IFLA_PERM_ADDRESS = 54,
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
	
	pub(crate) const IFLA_MAX: Self = unsafe { transmute(Self::__IFLA_MAX - 1) };
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWLINK`, `RTM_DELLINK`, `RTM_GETLINK` and `RTM_SETLINK`.
///
/// Nearly all of these are readable from `RTM_GETLINK`.
///
/// See Linux header `if_link.h`.
///
/// From the function `rtnl_ensure_unique_netns()` in `rtnelink.c` in the Linux source:-
///
/// * The attributes `IFLA_NET_NS_PID`, `IFLA_NET_NS_FD` and `IFLA_TARGET_NETNSID` are mutually exclusive; only one must be present in `RTM_NEWLINK`, `RTM_SETLINK` or `RTM_DELLINK` unless;
/// * To refer *only* by net namespace identifier (`netns id`), one of `IFLA_NET_NS_PID` or `IFLA_NET_NS_FD` must be present;
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum IFLA
{
	/// Should never be returned by `rtnetlink.c`.
	#[allow(dead_code)]
	IFLA_UNSPEC = 0,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_ADDRESS = 1,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_BROADCAST = 2,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_DELLINK` if `ifi_index` is `None` in `ifinfomsg`.
	/// Used in `RTM_SETLINK` (to change name).
	/// Used in request to `RTM_GETLINK` if `if_index` in `ifinfomsg` is `None`.
	#[allow(dead_code)]
	IFLA_IFNAME = 3,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_MTU = 4,
	
	/// ?Used for bonded ('linked') devices.
	///
	/// If present then `IFLA_LINK_NETNSID` should be present.
	///
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_LINK = 5,
	
	/// Used in `RTM_GETLINK`.
	///
	/// Size is `IFNAMSIZ`.
	#[allow(dead_code)]
	IFLA_QDISC = 6,
	
	/// Legacy 32-bit stats; still used but `IFLA_STATS64` is preferred.
	///
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_STATS = 7,
	
	/// Unused by Route Netlink.
	#[deprecated]
	#[allow(dead_code)]
	IFLA_COST = 8,
	
	/// Unused by Route Netlink.
	#[deprecated]
	#[allow(dead_code)]
	IFLA_PRIORITY = 9,
	
	/// Used for bonded ('linked') devices.
	///
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_MASTER = 10,
	
	/// Wireless Extension event - see `wireless.h`.
	///
	/// Used in `RTM_NEWLINK` when an event is sent.
	#[allow(dead_code)]
	IFLA_WIRELESS = 11,
	
	/// Nested.
	///
	/// See enum `IFLA_INET6`.
	///
	/// Protocol specific information for a link.
	///
	/// Explicitly *not supported* in `RTM_NEWLINK`.
	/// Used in `RTM_GETLINK` but only for `PF_BRIDGE`.
	#[allow(dead_code)]
	IFLA_PROTINFO = 12,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_TXQLEN = 13,
	
	/// Used in `RTM_GETLINK`.
	/// Explicitly *not supported* in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_MAP = 14,
	
	/// An `u32`.
	///
	/// Space is reserved for this but it does not seem to be used.
	#[deprecated]
	#[allow(dead_code)]
	IFLA_WEIGHT = 15,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	IFLA_OPERSTATE = 16,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	IFLA_LINKMODE = 17,
	
	/// Used for bonded ('linked') devices.
	///
	/// Nested.
	///
	/// See enum `IFLA_INFO` (eg for `IFLA_INFO_KIND`, `IFLA_INFO_DATA`, `IFLA_INFO_SLAVE_KIND` or `IFLA_INFO_SLAVE_DATA`).
	///
	/// See also `IFLA_LINK` and `IFLA_LINK_NETNSID`.
	///
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_LINKINFO = 18,
	
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	/// Used in `RTM_DELLINK`.
	#[allow(dead_code)]
	IFLA_NET_NS_PID = 19,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK` (to change alias).
	#[allow(dead_code)]
	IFLA_IFALIAS = 20,
	
	/// Used as part of PCI Virtual Function (VF) control (read about SR-IOV Physical Function).
	///
	/// Number of PCI Virtual Functions (VF) if device has SR-IOV Physical Function.
	///
	/// Used in `RTM_GETLINK`; `IFLA_VFINFO_LIST` will be populated if this is present.
	#[allow(dead_code)]
	IFLA_NUM_VF = 21,
	
	/// Used as part of PCI Virtual Function (VF) control (read about SR-IOV Physical Function).
	///
	/// Nested.
	///
	/// See enum `IFLA_VF_INFO`.
	///
	/// Used in `RTM_GETLINK`; `IFLA_NUM_VF` will be populated if this is present.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_VFINFO_LIST = 22,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_STATS64 = 23,
	
	/// Used as part of PCI Virtual Function (VF) control (read about SR-IOV Physical Function).
	///
	/// Nested.
	///
	/// See enum `IFLA_VF_PORT`.
	///
	/// Used in conjunction with `IFLA_PORT_SELF`.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_VF_PORTS = 24,
	
	/// Used as part of PCI Virtual Function (VF) control (read about SR-IOV Physical Function).
	///
	/// Nested.
	///
	/// See enum `IFLA_?`.
	///
	/// Used in conjunction with `IFLA_VF_PORTS`.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_PORT_SELF = 25,
	
	/// Address family ?
	///
	/// Nested.
	///
	/// See enum `IFLA_?`.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	///
	/// A nested list of families (eg `AF_INET`).
	/// In practice only `AF_INET` and `AF_INET6` (see `inet6_fill_link_af()` are supported.
	///
	/// For `AF_INET6`:-
	///
	/// * The enum is `IFLA_INET6`.
	/// * Values include:-
	/// 	* `IFLA_INET6_FLAGS`
	/// 	* `IFLA_INET6_CACHEINFO`
	/// 	* `IFLA_INET6_CONF`
	/// 	* `IFLA_INET6_STATS`
	/// 	* `IFLA_INET6_ICMP6STATS`
	/// 	* `IFLA_INET6_ADDR_GEN_MODE`
	///
	/// Each families attribute is itself nested.
	#[allow(dead_code)]
	IFLA_AF_SPEC = 26,
	
	/// Group the device belongs to.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	/// Used in `RTM_DELLINK` if `ifi_index` is `None` in `ifinfomsg` and `IFLA_IFNAME` and `IFLA_ALT_IFNAAME` are not specified.
	#[allow(dead_code)]
	IFLA_GROUP = 27,
	
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	/// Used in `RTM_DELLINK`.
	#[allow(dead_code)]
	IFLA_NET_NS_FD = 28,
	
	/// "Extended infomation mask, PCI Virtual Functions, etc".
	/// 
	/// `u32`.
	/// 
	/// Used in request to `RTM_GETLINK` if `if_index` in `ifinfomsg` is `None`.
	#[allow(dead_code)]
	IFLA_EXT_MASK = 29,
	
	/// Promiscuity count: > 0 means acts PROMISC.
	///
	/// Refers to ethernet promiscuity.
	/// Required to be `> 0` to receive port mirrored frames.
	///
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_PROMISCUITY = 30,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	#[allow(dead_code)]
	IFLA_NUM_TX_QUEUES = 31,
	
	/// Only if the Linux kernel was configured for Receive Packet Steering (RPS) with `CONFIG_RPS`.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	#[allow(dead_code)]
	IFLA_NUM_RX_QUEUES = 32,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_CARRIER = 33,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_PHYS_PORT_ID = 34,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_CARRIER_CHANGES = 35,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_PHYS_SWITCH_ID = 36,
	
	/// ?Used for bonded ('linked') devices.
	///
	/// If present then `IFLA_LINK` should be present.
	///
	/// See also `IFLA_LINKINFO`.
	///
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_LINK_NETNSID = 37,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_PHYS_PORT_NAME = 38,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_PROTO_DOWN = 39,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_GSO_MAX_SEGS = 40,
	
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_GSO_MAX_SIZE = 41,
	
	/// Internal usage by `rtnelink.c` only.
	#[allow(dead_code)]
	IFLA_PAD = 42,
	
	/// Nested.
	///
	/// See enum `IFLA_XDP`.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	IFLA_XDP = 43,
	
	/// Used in `RTM_GETLINK` when an event is sent.
	#[allow(dead_code)]
	IFLA_EVENT = 44,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_NEW_NETNSID = 45,
	
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_SETLINK`.
	/// Used in `RTM_DELLINK`.
	/// Used in request to `RTM_GETLINK` if `if_index` in `ifinfomsg` is `None`.
	#[allow(dead_code)]
	IFLA_TARGET_NETNSID = 46,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_CARRIER_UP_COUNT = 47,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_CARRIER_DOWN_COUNT = 48,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_NEW_IFINDEX = 49,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_MIN_MTU = 50,
	
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_MAX_MTU = 51,
	
	/// Nested.
	///
	/// See enum `IFLA_?`.
	///
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_NEWLINKPROP`.
	/// Used in `RTM_DELLINKPROP`.
	///
	/// Currently just a list of `IFLA_ALT_IFNAME`.
	#[allow(dead_code)]
	IFLA_PROP_LIST = 52,
	
	/// Alternative network interface name.
	///
	/// Used in `RTM_NEWLINK`.
	/// Used in `RTM_DELLINK` if `ifi_index` is `None` in `ifinfomsg` and `IFLA_IFNAME` not specified.
	/// Used in request to `RTM_GETLINK` if `if_index` in `ifinfomsg` is `None`.
	#[allow(dead_code)]
	IFLA_ALT_IFNAME = 53,
	
	/// Permanent hardware address ('burned in' MAC).
	///
	/// Used in `RTM_GETLINK`.
	#[allow(dead_code)]
	IFLA_PERM_ADDRESS = 54,
	
	/// Nested.
	///
	/// See enum `IFLA_PROTO_DOWN_REASON` with value `IFLA_PROTO_DOWN_REASON_VALUE` (a `NonZeroU32`).
	///
	/// Only present if `IFLA_PROTO_DOWN` is present *and* `IFLA_PROTO_DOWN_REASON_VALUE` is non-zero.
	/// Used in `RTM_GETLINK`.
	/// Used in `RTM_SETLINK`.
	#[allow(dead_code)]
	IFLA_PROTO_DOWN_REASON = 55,
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
	
	#[allow(dead_code)]
	#[deprecated]
	pub(crate) const IFLA_IF_NETNSID: Self = IFLA::IFLA_TARGET_NETNSID;
	
	#[allow(dead_code)]
	pub(crate) const IFLA_MAX: Self = unsafe { transmute(Self::__IFLA_MAX - 1) };
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet Protocol version 4 configuration value keys.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum IPV4_DEVCONF
{
	/// `forwarding`.
	IPV4_DEVCONF_FORWARDING = 1,

	/// `multicast_forwarding`.
	IPV4_DEVCONF_MC_FORWARDING = 2,

	/// `proxy_arp`.
	IPV4_DEVCONF_PROXY_ARP = 3,

	/// `accept_redirects`.
	IPV4_DEVCONF_ACCEPT_REDIRECTS = 4,

	/// `secure_redirects`.
	IPV4_DEVCONF_SECURE_REDIRECTS = 5,

	/// `send_redirects`.
	IPV4_DEVCONF_SEND_REDIRECTS = 6,

	/// `shared_media`.
	IPV4_DEVCONF_SHARED_MEDIA = 7,

	/// `rp_filter`.
	IPV4_DEVCONF_RP_FILTER = 8,

	/// `accept_source_route`.
	IPV4_DEVCONF_ACCEPT_SOURCE_ROUTE = 9,

	/// `bootp_relay`.
	IPV4_DEVCONF_BOOTP_RELAY = 10,

	/// `log_martians`.
	IPV4_DEVCONF_LOG_MARTIANS = 11,

	/// `tag`.
	IPV4_DEVCONF_TAG = 12,

	/// `arp_filter`.
	IPV4_DEVCONF_ARPFILTER = 13,

	/// `medium_id`.
	IPV4_DEVCONF_MEDIUM_ID = 14,

	/// `noxfrm`.
	IPV4_DEVCONF_NOXFRM = 15,

	/// `nopolicy`.
	IPV4_DEVCONF_NOPOLICY = 16,

	/// `force_igmp_version`.
	IPV4_DEVCONF_FORCE_IGMP_VERSION = 17,

	/// `arp_announce`.
	IPV4_DEVCONF_ARP_ANNOUNCE = 18,

	/// `arp_ignore`.
	IPV4_DEVCONF_ARP_IGNORE = 19,

	/// `promote_secondaries`.
	IPV4_DEVCONF_PROMOTE_SECONDARIES = 20,

	/// `arp_accept`.
	IPV4_DEVCONF_ARP_ACCEPT = 21,

	/// `arp_notify`.
	IPV4_DEVCONF_ARP_NOTIFY = 22,

	/// `accept_local`.
	IPV4_DEVCONF_ACCEPT_LOCAL = 23,

	/// `source_vmark`.
	IPV4_DEVCONF_SRC_VMARK = 24,

	/// `proxy_arp_pvlan`.
	IPV4_DEVCONF_PROXY_ARP_PVLAN = 25,

	/// `route_localnet`.
	IPV4_DEVCONF_ROUTE_LOCALNET = 26,

	/// `igmpv2_unsolicited_report_interval`.
	IPV4_DEVCONF_IGMPV2_UNSOLICITED_REPORT_INTERVAL = 27,

	/// `igmpv3_unsolicited_report_interval`.
	IPV4_DEVCONF_IGMPV3_UNSOLICITED_REPORT_INTERVAL = 28,

	/// `ignore_routes_with_link_down`.
	IPV4_DEVCONF_IGNORE_ROUTES_WITH_LINKDOWN = 29,

	/// `drop_unicast_in_l2_multicast`.
	IPV4_DEVCONF_DROP_UNICAST_IN_L2_MULTICAST = 30,

	/// `drop_gratuitous_arp`.
	IPV4_DEVCONF_DROP_GRATUITOUS_ARP = 31,

	/// `bc_forwarding`.
	IPV4_DEVCONF_BC_FORWARDING = 32,
}

impl From<u16> for IPV4_DEVCONF
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for IPV4_DEVCONF
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl IFLA_XDP
{
	const __IPV4_DEVCONF_MAX: u16 = 33;
	
	#[allow(dead_code)] pub(crate) const IPV4_DEVCONF_MAX: Self = unsafe { transmute(Self::__IPV4_DEVCONF_MAX - 1) };
}

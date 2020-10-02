// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet Protocol version 6 configuration value keys.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub(crate) enum DEVCONF
{
	#[allow(dead_code)]
	DEVCONF_FORWARDING = 0,
	
	#[allow(dead_code)]
	DEVCONF_HOPLIMIT = 1,
	
	#[allow(dead_code)]
	DEVCONF_MTU6 = 2,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA = 3,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_REDIRECTS = 4,
	
	#[allow(dead_code)]
	DEVCONF_AUTOCONF = 5,
	
	#[allow(dead_code)]
	DEVCONF_DAD_TRANSMITS = 6,
	
	#[allow(dead_code)]
	DEVCONF_RTR_SOLICITS = 7,
	
	#[allow(dead_code)]
	DEVCONF_RTR_SOLICIT_INTERVAL = 8,
	
	#[allow(dead_code)]
	DEVCONF_RTR_SOLICIT_DELAY = 9,
	
	#[allow(dead_code)]
	DEVCONF_USE_TEMPADDR = 10,
	
	#[allow(dead_code)]
	DEVCONF_TEMP_VALID_LFT = 11,
	
	#[allow(dead_code)]
	DEVCONF_TEMP_PREFERED_LFT = 12,
	
	#[allow(dead_code)]
	DEVCONF_REGEN_MAX_RETRY = 13,
	
	#[allow(dead_code)]
	DEVCONF_MAX_DESYNC_FACTOR = 14,
	
	#[allow(dead_code)]
	DEVCONF_MAX_ADDRESSES = 15,
	
	#[allow(dead_code)]
	DEVCONF_FORCE_MLD_VERSION = 16,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_DEFRTR = 17,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_PINFO = 18,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_RTR_PREF = 19,
	
	#[allow(dead_code)]
	DEVCONF_RTR_PROBE_INTERVAL = 20,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_RT_INFO_MAX_PLEN = 21,
	
	#[allow(dead_code)]
	DEVCONF_PROXY_NDP = 22,
	
	#[allow(dead_code)]
	DEVCONF_OPTIMISTIC_DAD = 23,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_SOURCE_ROUTE = 24,
	
	#[allow(dead_code)]
	DEVCONF_MC_FORWARDING = 25,
	
	#[allow(dead_code)]
	DEVCONF_DISABLE_IPV6 = 26,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_DAD = 27,
	
	#[allow(dead_code)]
	DEVCONF_FORCE_TLLAO = 28,
	
	#[allow(dead_code)]
	DEVCONF_NDISC_NOTIFY = 29,
	
	#[allow(dead_code)]
	DEVCONF_MLDV1_UNSOLICITED_REPORT_INTERVAL = 30,
	
	#[allow(dead_code)]
	DEVCONF_MLDV2_UNSOLICITED_REPORT_INTERVAL = 31,
	
	#[allow(dead_code)]
	DEVCONF_SUPPRESS_FRAG_NDISC = 32,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_FROM_LOCAL = 33,
	
	#[allow(dead_code)]
	DEVCONF_USE_OPTIMISTIC = 34,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_MTU = 35,
	
	#[allow(dead_code)]
	DEVCONF_STABLE_SECRET = 36,
	
	#[allow(dead_code)]
	DEVCONF_USE_OIF_ADDRS_ONLY = 37,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_MIN_HOP_LIMIT = 38,
	
	#[allow(dead_code)]
	DEVCONF_IGNORE_ROUTES_WITH_LINKDOWN = 39,
	
	#[allow(dead_code)]
	DEVCONF_DROP_UNICAST_IN_L2_MULTICAST = 40,
	
	#[allow(dead_code)]
	DEVCONF_DROP_UNSOLICITED_NA = 41,
	
	#[allow(dead_code)]
	DEVCONF_KEEP_ADDR_ON_DOWN = 42,
	
	#[allow(dead_code)]
	DEVCONF_RTR_SOLICIT_MAX_INTERVAL = 43,
	
	#[allow(dead_code)]
	DEVCONF_SEG6_ENABLED = 44,
	
	#[allow(dead_code)]
	DEVCONF_SEG6_REQUIRE_HMAC = 45,
	
	#[allow(dead_code)]
	DEVCONF_ENHANCED_DAD = 46,
	
	#[allow(dead_code)]
	DEVCONF_ADDR_GEN_MODE = 47,
	
	#[allow(dead_code)]
	DEVCONF_DISABLE_POLICY = 48,
	
	#[allow(dead_code)]
	DEVCONF_ACCEPT_RA_RT_INFO_MIN_PLEN = 49,
	
	#[allow(dead_code)]
	DEVCONF_NDISC_TCLASS = 50,
	
	#[allow(dead_code)]
	DEVCONF_RPL_SEG_ENABLED = 51,
}

impl From<u16> for DEVCONF
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for DEVCONF
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl DEVCONF
{
	#[allow(dead_code)]
	pub(crate) const DEVCONF_MAX: u16 = 52;
}

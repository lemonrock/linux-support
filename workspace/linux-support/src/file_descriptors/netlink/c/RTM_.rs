// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum RTM_
{
	RTM_NEWLINK = 16,

	RTM_DELLINK = 17,

	RTM_GETLINK = 18,

	RTM_SETLINK = 19,

	RTM_NEWADDR = 20,

	RTM_DELADDR = 21,

	RTM_GETADDR = 22,

	RTM_NEWROUTE = 24,

	RTM_DELROUTE = 25,

	RTM_GETROUTE = 26,

	RTM_NEWNEIGH = 28,

	RTM_DELNEIGH = 29,

	RTM_GETNEIGH = 30,

	RTM_NEWRULE = 32,

	RTM_DELRULE = 33,

	RTM_GETRULE = 34,

	RTM_NEWQDISC = 36,

	RTM_DELQDISC = 37,

	RTM_GETQDISC = 38,

	RTM_NEWTCLASS = 40,

	RTM_DELTCLASS = 41,

	RTM_GETTCLASS = 42,

	RTM_NEWTFILTER = 44,

	RTM_DELTFILTER = 45,

	RTM_GETTFILTER = 46,

	RTM_NEWACTION = 48,

	RTM_DELACTION = 49,

	RTM_GETACTION = 50,

	RTM_NEWPREFIX = 52,

	RTM_GETMULTICAST = 58,

	RTM_GETANYCAST = 62,

	RTM_NEWNEIGHTBL = 64,

	RTM_GETNEIGHTBL = 66,

	RTM_SETNEIGHTBL = 67,

	RTM_NEWNDUSEROPT = 68,

	RTM_NEWADDRLABEL = 72,

	RTM_DELADDRLABEL = 73,

	RTM_GETADDRLABEL = 74,

	RTM_GETDCB = 78,

	RTM_SETDCB = 79,

	RTM_NEWNETCONF = 80,

	#[allow(dead_code)] RTM_DELNETCONF = 81,

	RTM_GETNETCONF = 82,

	RTM_NEWMDB = 84,

	RTM_DELMDB = 85,

	RTM_GETMDB = 86,

	RTM_NEWNSID = 88,

	RTM_DELNSID = 89,

	RTM_GETNSID = 90,

	RTM_NEWSTATS = 92,

	RTM_GETSTATS = 94,

	RTM_NEWCACHEREPORT = 96,

	RTM_NEWCHAIN = 100,

	RTM_DELCHAIN = 101,

	RTM_GETCHAIN = 102,

	RTM_NEWNEXTHOP = 104,

	RTM_DELNEXTHOP = 105,

	RTM_GETNEXTHOP = 106,

	RTM_NEWLINKPROP = 108,

	RTM_DELLINKPROP = 109,

	RTM_GETLINKPROP = 110,

	RTM_NEWVLAN = 112,

	RTM_DELVLAN = 113,

	RTM_GETVLAN = 114,
}

impl RTM_
{
	/// This value *must* be the same value as `NLMSG_MIN_TYPE`.
	#[allow(dead_code)]
	const RTM_BASE: RTM_ = RTM_::RTM_NEWLINK;
	
	const __RTM_MAX: u16 = 115;
	
	#[allow(dead_code)]
	const RTM_MAX: u16 = ((Self::__RTM_MAX + 3) & !3) - 1;
	
	#[allow(dead_code)]
	const RTM_NR_MSGTYPES: u16 = Self::RTM_MAX + 1 - (Self::RTM_BASE as u16);
	
	#[allow(dead_code)]
	const RTM_NR_FAMILIES: u16 = Self::RTM_NR_MSGTYPES >> 2;
}

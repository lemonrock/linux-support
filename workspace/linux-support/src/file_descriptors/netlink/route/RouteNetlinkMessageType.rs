// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum RouteNetlinkMessageType
{
	#[allow(missing_docs)]
	NEWLINK = RTM_NEWLINK,

	#[allow(missing_docs)]
	DELLINK = RTM_DELLINK,

	#[allow(missing_docs)]
	GETLINK = RTM_GETLINK,

	#[allow(missing_docs)]
	SETLINK = RTM_SETLINK,

	#[allow(missing_docs)]
	NEWADDR = RTM_NEWADDR,

	#[allow(missing_docs)]
	DELADDR = RTM_DELADDR,

	#[allow(missing_docs)]
	GETADDR = RTM_GETADDR,

	#[allow(missing_docs)]
	NEWROUTE = RTM_NEWROUTE,

	#[allow(missing_docs)]
	DELROUTE = RTM_DELROUTE,

	#[allow(missing_docs)]
	GETROUTE = RTM_GETROUTE,

	#[allow(missing_docs)]
	NEWNEIGH = RTM_NEWNEIGH,

	#[allow(missing_docs)]
	DELNEIGH = RTM_DELNEIGH,

	#[allow(missing_docs)]
	GETNEIGH = RTM_GETNEIGH,

	#[allow(missing_docs)]
	NEWRULE = RTM_NEWRULE,

	#[allow(missing_docs)]
	DELRULE = RTM_DELRULE,

	#[allow(missing_docs)]
	GETRULE = RTM_GETRULE,

	#[allow(missing_docs)]
	NEWQDISC = RTM_NEWQDISC,

	#[allow(missing_docs)]
	DELQDISC = RTM_DELQDISC,

	#[allow(missing_docs)]
	GETQDISC = RTM_GETQDISC,

	#[allow(missing_docs)]
	NEWTCLASS = RTM_NEWTCLASS,

	#[allow(missing_docs)]
	DELTCLASS = RTM_DELTCLASS,

	#[allow(missing_docs)]
	GETTCLASS = RTM_GETTCLASS,

	#[allow(missing_docs)]
	NEWTFILTER = RTM_NEWTFILTER,

	#[allow(missing_docs)]
	DELTFILTER = RTM_DELTFILTER,

	#[allow(missing_docs)]
	GETTFILTER = RTM_GETTFILTER,

	#[allow(missing_docs)]
	NEWACTION = RTM_NEWACTION,

	#[allow(missing_docs)]
	DELACTION = RTM_DELACTION,

	#[allow(missing_docs)]
	GETACTION = RTM_GETACTION,

	#[allow(missing_docs)]
	NEWPREFIX = RTM_NEWPREFIX,

	#[allow(missing_docs)]
	GETMULTICAST = RTM_GETMULTICAST,

	#[allow(missing_docs)]
	GETANYCAST = RTM_GETANYCAST,

	#[allow(missing_docs)]
	NEWNEIGHTBL = RTM_NEWNEIGHTBL,

	#[allow(missing_docs)]
	GETNEIGHTBL = RTM_GETNEIGHTBL,

	#[allow(missing_docs)]
	SETNEIGHTBL = RTM_SETNEIGHTBL,

	#[allow(missing_docs)]
	NEWNDUSEROPT = RTM_NEWNDUSEROPT,

	#[allow(missing_docs)]
	NEWADDRLABEL = RTM_NEWADDRLABEL,

	#[allow(missing_docs)]
	DELADDRLABEL = RTM_DELADDRLABEL,

	#[allow(missing_docs)]
	GETADDRLABEL = RTM_GETADDRLABEL,

	#[allow(missing_docs)]
	GETDCB = RTM_GETDCB,

	#[allow(missing_docs)]
	SETDCB = RTM_SETDCB,

	#[allow(missing_docs)]
	NEWNETCONF = RTM_NEWNETCONF,

	#[allow(missing_docs)]
	GETNETCONF = RTM_GETNETCONF,

	#[allow(missing_docs)]
	NEWMDB = RTM_NEWMDB,

	#[allow(missing_docs)]
	DELMDB = RTM_DELMDB,

	#[allow(missing_docs)]
	GETMDB = RTM_GETMDB,

	#[allow(missing_docs)]
	NEWNSID = RTM_NEWNSID,

	#[allow(missing_docs)]
	DELNSID = RTM_DELNSID,

	#[allow(missing_docs)]
	GETNSID = RTM_GETNSID,
}

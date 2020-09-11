// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The lowest 2 bits encode a 'kind'.
///
/// These are: `New`, `Del(ete)`, `Get` and `Set`.
///
/// Apart from `Get`, all other 'kind's need a caller to have the capability `CAP_NET_ADMIN`.
///
/// A 'dump' is only supported for the `Get` kind.
#[doc(hidden)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum RouteNetlinkMessageType
{
	#[allow(missing_docs)]
	NEWLINK = RTM_::RTM_NEWLINK as u16,

	#[allow(missing_docs)]
	DELLINK = RTM_::RTM_DELLINK as u16,

	#[allow(missing_docs)]
	GETLINK = RTM_::RTM_GETLINK as u16,

	#[allow(missing_docs)]
	SETLINK = RTM_::RTM_SETLINK as u16,

	#[allow(missing_docs)]
	NEWADDR = RTM_::RTM_NEWADDR as u16,

	#[allow(missing_docs)]
	DELADDR = RTM_::RTM_DELADDR as u16,

	#[allow(missing_docs)]
	GETADDR = RTM_::RTM_GETADDR as u16,

	#[allow(missing_docs)]
	NEWROUTE = RTM_::RTM_NEWROUTE as u16,

	#[allow(missing_docs)]
	DELROUTE = RTM_::RTM_DELROUTE as u16,

	#[allow(missing_docs)]
	GETROUTE = RTM_::RTM_GETROUTE as u16,

	#[allow(missing_docs)]
	NEWNEIGH = RTM_::RTM_NEWNEIGH as u16,

	#[allow(missing_docs)]
	DELNEIGH = RTM_::RTM_DELNEIGH as u16,

	#[allow(missing_docs)]
	GETNEIGH = RTM_::RTM_GETNEIGH as u16,

	#[allow(missing_docs)]
	NEWRULE = RTM_::RTM_NEWRULE as u16,

	#[allow(missing_docs)]
	DELRULE = RTM_::RTM_DELRULE as u16,

	#[allow(missing_docs)]
	GETRULE = RTM_::RTM_GETRULE as u16,

	#[allow(missing_docs)]
	NEWQDISC = RTM_::RTM_NEWQDISC as u16,

	#[allow(missing_docs)]
	DELQDISC = RTM_::RTM_DELQDISC as u16,

	#[allow(missing_docs)]
	GETQDISC = RTM_::RTM_GETQDISC as u16,

	#[allow(missing_docs)]
	NEWTCLASS = RTM_::RTM_NEWTCLASS as u16,

	#[allow(missing_docs)]
	DELTCLASS = RTM_::RTM_DELTCLASS as u16,

	#[allow(missing_docs)]
	GETTCLASS = RTM_::RTM_GETTCLASS as u16,

	#[allow(missing_docs)]
	NEWTFILTER = RTM_::RTM_NEWTFILTER as u16,

	#[allow(missing_docs)]
	DELTFILTER = RTM_::RTM_DELTFILTER as u16,

	#[allow(missing_docs)]
	GETTFILTER = RTM_::RTM_GETTFILTER as u16,

	#[allow(missing_docs)]
	NEWACTION = RTM_::RTM_NEWACTION as u16,

	#[allow(missing_docs)]
	DELACTION = RTM_::RTM_DELACTION as u16,

	#[allow(missing_docs)]
	GETACTION = RTM_::RTM_GETACTION as u16,

	#[allow(missing_docs)]
	NEWPREFIX = RTM_::RTM_NEWPREFIX as u16,

	#[allow(missing_docs)]
	GETMULTICAST = RTM_::RTM_GETMULTICAST as u16,

	#[allow(missing_docs)]
	GETANYCAST = RTM_::RTM_GETANYCAST as u16,

	#[allow(missing_docs)]
	NEWNEIGHTBL = RTM_::RTM_NEWNEIGHTBL as u16,

	#[allow(missing_docs)]
	GETNEIGHTBL = RTM_::RTM_GETNEIGHTBL as u16,

	#[allow(missing_docs)]
	SETNEIGHTBL = RTM_::RTM_SETNEIGHTBL as u16,

	#[allow(missing_docs)]
	NEWNDUSEROPT = RTM_::RTM_NEWNDUSEROPT as u16,

	#[allow(missing_docs)]
	NEWADDRLABEL = RTM_::RTM_NEWADDRLABEL as u16,

	#[allow(missing_docs)]
	DELADDRLABEL = RTM_::RTM_DELADDRLABEL as u16,

	#[allow(missing_docs)]
	GETADDRLABEL = RTM_::RTM_GETADDRLABEL as u16,

	#[allow(missing_docs)]
	GETDCB = RTM_::RTM_GETDCB as u16,

	#[allow(missing_docs)]
	SETDCB = RTM_::RTM_SETDCB as u16,

	#[allow(missing_docs)]
	NEWNETCONF = RTM_::RTM_NEWNETCONF as u16,

	#[allow(missing_docs)]
	GETNETCONF = RTM_::RTM_GETNETCONF as u16,

	#[allow(missing_docs)]
	NEWMDB = RTM_::RTM_NEWMDB as u16,

	#[allow(missing_docs)]
	DELMDB = RTM_::RTM_DELMDB as u16,

	#[allow(missing_docs)]
	GETMDB = RTM_::RTM_GETMDB as u16,

	#[allow(missing_docs)]
	NEWNSID = RTM_::RTM_NEWNSID as u16,

	#[allow(missing_docs)]
	DELNSID = RTM_::RTM_DELNSID as u16,

	#[allow(missing_docs)]
	GETNSID = RTM_::RTM_GETNSID as u16,

	#[allow(missing_docs)]
	NEWSTATS = RTM_::RTM_NEWSTATS  as u16,

	#[allow(missing_docs)]
	GETSTATS = RTM_::RTM_GETSTATS  as u16,

	#[allow(missing_docs)]
	NEWCACHEREPORT = RTM_::RTM_NEWCACHEREPORT  as u16,

	#[allow(missing_docs)]
	NEWCHAIN = RTM_::RTM_NEWCHAIN  as u16,

	#[allow(missing_docs)]
	DELCHAIN = RTM_::RTM_DELCHAIN  as u16,

	#[allow(missing_docs)]
	GETCHAIN = RTM_::RTM_GETCHAIN  as u16,

	#[allow(missing_docs)]
	NEWNEXTHOP = RTM_::RTM_NEWNEXTHOP  as u16,

	#[allow(missing_docs)]
	DELNEXTHOP = RTM_::RTM_DELNEXTHOP  as u16,

	#[allow(missing_docs)]
	GETNEXTHOP = RTM_::RTM_GETNEXTHOP  as u16,

	#[allow(missing_docs)]
	NEWLINKPROP = RTM_::RTM_NEWLINKPROP  as u16,

	#[allow(missing_docs)]
	DELLINKPROP = RTM_::RTM_DELLINKPROP  as u16,

	#[allow(missing_docs)]
	GETLINKPROP = RTM_::RTM_GETLINKPROP  as u16,

	#[allow(missing_docs)]
	NEWVLAN = RTM_::RTM_NEWVLAN  as u16,

	#[allow(missing_docs)]
	DELVLAN = RTM_::RTM_DELVLAN  as u16,

	#[allow(missing_docs)]
	GETVLAN = RTM_::RTM_GETVLAN  as u16,
}

impl RouteNetlinkMessageType
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn kind(self) -> RouteNetlinkMessageKind
	{
		unsafe { transmute((self as u16) & 0b11) }
	}
}

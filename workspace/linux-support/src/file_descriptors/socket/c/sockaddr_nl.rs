// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct sockaddr_nl
{
	/// Socket address family, always `AF_NETLINK`.
	nl_family: sa_family_t,
	
	/// Always zero.
	nl_pad: u16,
	
	/// Technically, this actually Option<ProcessIdentifier> as `None` means 'let Linux kernel choose process identifier to use for port identifier'.
	nl_pid: PortIdentifier,
	
	/// Legacy multicast groups mask.
	///
	/// Instead, set this to `0` and then use `setsockopt(socket, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP, &group, size_of::<i32>())` with the enum `rtnetlink_groups` which contains constants starting `RTNLGRP_*`; these are the value of `group`.
	///
	/// Legacy groups mask used flags bits starting `RTMGRP_*` but there are far more group memberships possible then there are bits in `nl_groups`!
	nl_groups: u32,
}

impl Default for sockaddr_nl
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			nl_family: AF_NETLINK as sa_family_t,
			nl_pad: 0,
			nl_pid: PortIdentifier::current_process(),
			nl_groups: 0,
		}
	}
}

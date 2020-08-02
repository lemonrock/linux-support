// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Interface name assignment types.
///
/// Also available at `/sys/class/net/<network_interface_name>/name_assign_type`.
/// This is not readable if the value is `NET_NAME::NET_NAME_UNKNOWN` (seems to return `EINVAL`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(EnumCount)]
#[repr(u8)]
pub enum NET_NAME
{
	/// Unknown origin (not exposed to userspace).
	NET_NAME_UNKNOWN = 0,
	
	/// Enumerated by kernel.
	NET_NAME_ENUM = 1,
	
	/// Predictably named by the kernel.
	NET_NAME_PREDICTABLE = 2,
	
	/// Provided by userspace.
	NET_NAME_USER = 3,
	
	/// Renamed by userspace.
	NET_NAME_RENAMED = 4,
}

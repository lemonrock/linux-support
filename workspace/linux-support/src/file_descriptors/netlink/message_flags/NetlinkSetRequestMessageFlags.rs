// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Netlink get request message flags.
///
/// Used for `RouteNetlinkMessageType` with a kind of `RouteNetlinkMessageKind::Set`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetlinkSetRequestMessageFlags(u16);

impl NetlinkSetRequestMessageFlags
{
	#[inline(always)]
	pub const fn empty() -> Self
	{
		Self(0)
	}
}

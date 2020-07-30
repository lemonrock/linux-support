// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// If link data was sent in response to an event, represents the reason it was sent.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum IFLA_EVENT
{
	/// Not specified.
	IFLA_EVENT_NONE = 0,
	
	#[allow(missing_docs)]
	IFLA_EVENT_REBOOT = 1,
	
	#[allow(missing_docs)]
	IFLA_EVENT_FEATURES = 2,
	
	#[allow(missing_docs)]
	IFLA_EVENT_BONDING_FAILOVER = 3,
	
	#[allow(missing_docs)]
	IFLA_EVENT_NOTIFY_PEERS = 4,
	
	#[allow(missing_docs)]
	IFLA_EVENT_IGMP_RESEND = 5,
	
	#[allow(missing_docs)]
	IFLA_EVENT_BONDING_OPTIONS = 6,
}

impl Default for IFLA_EVENT
{
	#[inline(always)]
	fn default() -> Self
	{
		IFLA_EVENT::IFLA_EVENT_NONE
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// HMAC policy for SR (?Segment Routing) enabled packets.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum HmacPolicyForSrEnabledPackets
{
	#[allow(missing_docs)]
	IgnoreHmacField = -1,
	
	#[allow(missing_docs)]
	AcceptSrEnabledPacketsWithoutHmacAndValidateThoseWithHmac = 0,
	
	#[allow(missing_docs)]
	DropSrEnabledPacketsWithoutHmacAndValidateThoseWithHmac = 1,
}

impl Default for HmacPolicyForSrEnabledPackets
{
	#[inline(always)]
	fn default() -> Self
	{
		HmacPolicyForSrEnabledPackets::AcceptSrEnabledPacketsWithoutHmacAndValidateThoseWithHmac
	}
}

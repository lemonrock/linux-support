// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Route distance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum rt_scope
{
	/// Default if not better match.
	#[serde(rename = "Universe")] RT_SCOPE_UNIVERSE = 0,
	
	/// Site.
	#[serde(rename = "Site")] RT_SCOPE_SITE = 200,
	
	/// Link.
	#[serde(rename = "Link")] RT_SCOPE_LINK = 253,
	
	/// Host.
	#[serde(rename = "Host")] RT_SCOPE_HOST = 254,
	
	/// Believed to not be possible in Linux IPv6 logic.
	#[serde(rename = "Nowhere")] RT_SCOPE_NOWHERE = 255,
}

impl Default for rt_scope
{
	#[inline(always)]
	fn default() -> Self
	{
		rt_scope::RT_SCOPE_UNIVERSE
	}
}

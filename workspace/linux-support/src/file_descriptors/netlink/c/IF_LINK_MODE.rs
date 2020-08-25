// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Link modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum IF_LINK_MODE
{
	#[allow(missing_docs)]
	#[serde(rename = "Default")] IF_LINK_MODE_DEFAULT = 0,
	
	#[allow(missing_docs)]
	#[serde(rename = "Dormant")] IF_LINK_MODE_DORMANT = 1,
}

impl Default for IF_LINK_MODE
{
	#[inline(always)]
	fn default() -> Self
	{
		IF_LINK_MODE::IF_LINK_MODE_DEFAULT
	}
}

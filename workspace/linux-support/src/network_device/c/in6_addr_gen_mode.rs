// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum in6_addr_gen_mode
{
	#[serde(rename = "EUI-64")] IN6_ADDR_GEN_MODE_EUI64 = 0,
	
	#[serde(rename = "None")] IN6_ADDR_GEN_MODE_NONE = 1,
	
	#[serde(rename = "Stable Privacy")] IN6_ADDR_GEN_MODE_STABLE_PRIVACY = 2,
	
	#[serde(rename = "Random")] IN6_ADDR_GEN_MODE_RANDOM = 3,
}

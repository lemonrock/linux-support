// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive Side Scaling (RSS) function name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[derive(EnumCount)]
#[repr(u8)]
pub enum ETH_RSS_HASH
{
	/// Toepltiz.
	#[serde(rename = "Toeplitz")] ETH_RSS_HASH_TOP_BIT = 0,
	
	/// eXclusive Or (XOR).
	#[serde(rename = "XOR")] ETH_RSS_HASH_XOR_BIT = 1,
	
	/// Cyclic Redundancy Check 32 (CRC32).
	#[serde(rename = "CRC32")] ETH_RSS_HASH_CRC32_BIT = 2,
}

impl Default for ETH_RSS_HASH
{
	#[inline(always)]
	fn default() -> Self
	{
		ETH_RSS_HASH::ETH_RSS_HASH_TOP_BIT
	}
}

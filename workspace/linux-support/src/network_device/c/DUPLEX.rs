// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum DUPLEX
{
	/// Ethtool setting is `half`.
	#[serde(rename = "half")] DUPLEX_HALF = 0x00,
	
	/// Ethtool setting is `full`.
	#[serde(rename = "full")] DUPLEX_FULL = 0x01,
	
	/// Unknown.
	#[serde(rename = "unknown")] DUPLEX_UNKNOWN = 0xFF,
}

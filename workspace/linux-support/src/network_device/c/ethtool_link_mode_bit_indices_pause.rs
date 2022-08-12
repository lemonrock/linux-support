// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pause.
///
/// It is not valid for the bit `ETHTOOL_LINK_MODE_Pause_BIT` to be unset but the bit `ETHTOOL_LINK_MODE_Asym_Pause_BIT` set.
///
/// Bits are interpreted as follows:-
///
/// ```bash
/// ETHTOOL_LINK_MODE_Pause_BIT		ETHTOOL_LINK_MODE_Asym_Pause_BIT		Link Supports
/// True							True									Receive-only pausing.
/// True							False									Symmetric pausing.
/// False							True									Transmit-only pausing.
/// False							False									Pausing unsupported.
/// ```
/// ``
///
///
/// Strings are in the `ethtool_stringset::ETH_SS_LINK_MODES` string set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(EnumIter, EnumCount)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum ethtool_link_mode_bit_indices_pause
{
	/// If this bit is set, the link supports either symmetric, transmit or receive pausing.
	///
	/// String set value is `Pause`.
	#[serde(rename = "Pause")] ETHTOOL_LINK_MODE_Pause_BIT = 13,
	
	/// If this bit is set and
	///
	/// String set value is `Asym_Pause`.
	#[serde(rename = "AsymmetricPause")] ETHTOOL_LINK_MODE_Asym_Pause_BIT = 14,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// String set identifiers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(EnumCount, EnumIter)]
#[repr(u32)]
pub enum ethtool_stringset
{
	/// Self-test result names, for use with the command `ETHTOOL_TEST`.
	ETH_SS_TEST = 0,
	
	/// Statistic names, for use with the command `ETHTOOL_GSTATS`.
	ETH_SS_STATS = 1,
	
	/// Driver private flag names, for use with the commands `ETHTOOL_GPFLAGS` and the command `ETHTOOL_SPFLAGS`.
	ETH_SS_PRIV_FLAGS = 2,
	
	/// Previously used with the command `ETHTOOL_GRXNTUPLE`.
	#[deprecated]
	ETH_SS_NTUPLE_FILTERS = 3,
	
	/// Device feature names.
	ETH_SS_FEATURES = 4,
	
	/// RSS hssh function names.
	ETH_SS_RSS_HASH_FUNCS = 5,
	
	/// ?Tunable names?
	///
	/// See enum `tunable_id`.
	ETH_SS_TUNABLES = 6,
	
	/// PHY statistic names, for use with the command `ETHTOOL_GPHYSTATS`.
	ETH_SS_PHY_STATS = 7,
	
	/// PHY tunable names.
	///
	/// See enum `phy_tunable_id`.
	ETH_SS_PHY_TUNABLES = 8,
	
	/// Link mode names.
	///
	/// See union `ethtool_link_mode_bit_indices` which contains the enums `ethtool_link_mode_bit_indices_speed` and `ethtool_link_mode_bit_indices_special`.
	ETH_SS_LINK_MODES = 9,
	
	/// Driver message class names.
	///
	/// See bitflags struct `NETIF_MSG`.
	ETH_SS_MSG_CLASSES = 10,
	
	/// Wake-on-LAN modes.
	///
	/// See bitflags struct `WAKE`.
	ETH_SS_WOL_MODES = 11,
	
	/// Names for `SOF_TIMESTAMPING` flags.
	///
	/// See enum `SOF_TIMESTAMPING`.
	ETH_SS_SOF_TIMESTAMPING = 12,
	
	/// Timestamping transmit types.
	///
	/// See enum `hwtstamp_tx_types`.
	ETH_SS_TS_TX_TYPES = 13,
	
	/// Timestamping receive filters.
	///
	/// See enum `hwtstamp_rx_filters`.
	ETH_SS_TS_RX_FILTERS = 14,
}

impl ethtool_stringset
{
	#[doc(hidden)]
	const ETH_SS_COUNT: u32 = Self::ETH_SS_TS_RX_FILTERS as u32;
	
	const fn to_u64_bit(self) -> u64
	{
		1 << (self as u32 as u64)
	}
}

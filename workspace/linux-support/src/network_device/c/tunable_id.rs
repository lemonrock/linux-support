// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_PHY_TUNABLES` string set.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum tunable_id
{
	/// String set value is `Unspec`.
	#[deprecated]
	ETHTOOL_ID_UNSPEC = 0,
	
	/// String set value is `rx-copybreak`.
	///
	/// Tunable size is `tunable_type_id::ETHTOOL_TUNABLE_U32`.
	ETHTOOL_RX_COPYBREAK = 1,
	
	/// String set value is `tx-copybreak`.
	///
	/// Tunable size is `tunable_type_id::ETHTOOL_TUNABLE_U32`.
	ETHTOOL_TX_COPYBREAK = 2,
	
	/// Time in milliseconds after which the link is reported as down.
	///
	/// There are two special values:-
	///
	/// * `PFC_STORM_PREVENTION_AUTO`: `0`.
	/// * `PFC_STORM_PREVENTION_DISABLE`: `0xFFFF`.
	///
	/// String set value is `pfc-prevention-tout`.
	///
	/// Tunable size is `tunable_type_id::ETHTOOL_TUNABLE_U16`.
	ETHTOOL_PFC_PREVENTION_TOUT = 3,
}

impl tunable_id
{
	const __ETHTOOL_TUNABLE_COUNT: usize = 4;
}

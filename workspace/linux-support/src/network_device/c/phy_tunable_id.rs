// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_PHY_TUNABLES` string set.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum phy_tunable_id
{
	/// String set value is `Unspec`.
	#[deprecated]
	ETHTOOL_PHY_ID_UNSPEC = 0,
	
	/// There are two special values:-
	///
	/// * `DOWNSHIFT_DEV_DISABLE`: `0`.
	/// * `DOWNSHIFT_DEV_DEFAULT_COUNT`: `0xFF`.
	///
	/// String set value is `phy-downshift`.
	///
	/// Ethtool setting is `downshift`.
	ETHTOOL_PHY_DOWNSHIFT = 1,
	
	/// Time in milliseconds after which the link is reported as down.
	///
	/// There are two special values:-
	///
	/// * `ETHTOOL_PHY_FAST_LINK_DOWN_ON`: `0`.
	/// * `ETHTOOL_PHY_FAST_LINK_DOWN_OFF`: `0xFF`.
	///
	/// String set value is `phy-fast-link-down`.
	///
	/// Ethtool setting is `fast-link-down`.
	ETHTOOL_PHY_FAST_LINK_DOWN = 2,
	
	/// Energy Detect Power Down (EDPD) is a feature supported by some PHYs, where the PHY's RX & TX blocks are put into a low-power mode when there is no link detected (typically the cable is un-plugged).
	///
	/// For RX, only a minimal link-detection is available, and for TX the PHY wakes up to send link pulses to avoid any lock-ups in case the peer PHY may also be running in EDPD mode.
	///
	/// Some PHYs may support configuration of the wake-up interval for TX pulses, and some PHYs may support only disabling TX pulses entirely.
	/// For the latter a special value is required (`ETHTOOL_PHY_EDPD_NO_TX`) so that this can be configured from userspace (should the user want it).
	///
	/// The interval units for TX wake-up are in milliseconds, since this should cover a reasonable range of intervals:-
	///
	/// * from 1 millisecond, which does not sound like much of a power-saver;
	/// * to circa 65 seconds which is quite a lot to wait for a link to come up when plugging a cable in.
	///
	/// There are three special values:-
	///
	/// * `ETHTOOL_PHY_EDPD_DISABLE`: `0`.
	/// * `ETHTOOL_PHY_EDPD_NO_TX`: `0xFFFE`.
	/// * `ETHTOOL_PHY_EDPD_DFLT_TX_MSECS`: `0xFFFF`.
	///
	/// String set value is `phy-energy-detect-power-down`.
	///
	/// Ethtool setting is `energy-detect-power-down`.
	ETHTOOL_PHY_EDPD = 3,
}

impl phy_tunable_id
{
	const __ETHTOOL_PHY_TUNABLE_COUNT: usize = 4;
}

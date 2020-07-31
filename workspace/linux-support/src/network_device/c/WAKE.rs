// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Wake-On-Lan options.
	///
	/// Strings are in the `ethtool_stringset::ETH_SS_WOL_MODES` string set.
	pub(crate) struct WAKE: u32
	{
		/// Wake on PHY activity.
		///
		/// String set value is `phy`.
		///
		/// Ethtool setting is `p`.
		const WAKE_PHY = 1 << 0;
		
		/// Wake on unicast messages.
		///
		/// String set value is `ucast`.
		///
		/// Ethtool setting is `u`.
		const WAKE_UCAST = 1 << 1;
		
		/// Wake on multicast messages.
		///
		/// String set value is `mcast`.
		///
		/// Ethtool setting is `m`.
		const WAKE_MCAST = 1 << 2;
		
		/// Wake on broadcast messages.
		///
		/// String set value is `bcast`.
		///
		/// Ethtool setting is `b`.
		const WAKE_BCAST = 1 << 3;
		
		/// Wake on ARP.
		///
		/// String set value is `arp`.
		///
		/// Ethtool setting is `a`.
		const WAKE_ARP = 1 << 4;
		
		/// Wake on MagicPacket™.
		///
		/// String set value is `magic`.
		///
		/// Ethtool setting is `g`.
		const WAKE_MAGIC = 1 << 5;
		
		/// Enable SecureOn™ password for MagicPacket™.
		///
		/// Only meaningful if `WAKE_MAGIC` is specified; the value of `ethtool_wolinfo.sopass` is valid.
		///
		/// String set value is `magicsecure`.
		///
		/// Ethtool setting is `s`.
		const WAKE_MAGICSECURE = 1 << 6;
		
		/// Wake on filter(s).
		///
		/// String set value is `filter`.
		///
		/// Ethtool setting is `f`.
		const WAKE_FILTER = 1 << 7;
	}
}

impl WAKE
{
	#[allow(dead_code)]
	const WOL_MODE_COUNT: usize = 8;
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Wake-On-Lan options.
	///
	/// Strings are in the `ethtool_stringset::ETH_SS_WOL_MODES` string set.
	pub(crate) struct WAKE: u32
	{
		/// String set value is `phy`.
		const WAKE_PHY = 1 << 0;
		
		/// String set value is `ucast`.
		const WAKE_UCAST = 1 << 1;
		
		/// String set value is `mcast`.
		const WAKE_MCAST = 1 << 2;
		
		/// String set value is `bcast`.
		const WAKE_BCAST = 1 << 3;
		
		/// String set value is `arp`.
		const WAKE_ARP = 1 << 4;
		
		/// String set value is `magic`.
		const WAKE_MAGIC = 1 << 5;
		
		/// Only meaningful if `WAKE_MAGIC` is specified; the value of `ethtool_wolinfo.sopass` is valid.
		///
		/// String set value is `magicsecure`.
		const WAKE_MAGICSECURE = 1 << 6;
		
		/// String set value is `filter`.
		const WAKE_FILTER = 1 << 7;
	}
}

impl WAKE
{
	#[allow(dead_code)]
	const WOL_MODE_COUNT: usize = 8;
}

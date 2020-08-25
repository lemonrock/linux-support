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
		const WAKE_PHY = WakeOnLanWhen::PHY.to_bitflag();
		
		/// Wake on unicast messages.
		///
		/// String set value is `ucast`.
		///
		/// Ethtool setting is `u`.
		const WAKE_UCAST = WakeOnLanWhen::UnicastMessages.to_bitflag();
		
		/// Wake on multicast messages.
		///
		/// String set value is `mcast`.
		///
		/// Ethtool setting is `m`.
		const WAKE_MCAST = WakeOnLanWhen::MulticastMessages.to_bitflag();
		
		/// Wake on broadcast messages.
		///
		/// String set value is `bcast`.
		///
		/// Ethtool setting is `b`.
		const WAKE_BCAST = WakeOnLanWhen::BroadcastMessages.to_bitflag();
		
		/// Wake on ARP.
		///
		/// String set value is `arp`.
		///
		/// Ethtool setting is `a`.
		const WAKE_ARP = WakeOnLanWhen::AddressResolutionProtocolPackets.to_bitflag();
		
		/// Wake on MagicPacket™.
		///
		/// String set value is `magic`.
		///
		/// Ethtool setting is `g`.
		const WAKE_MAGIC = WakeOnLanWhen::MagicPacket.to_bitflag();
		
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
		const WAKE_FILTER = WakeOnLanWhen::Filters.to_bitflag();
	}
}

impl WAKE
{
	pub(crate) const WOL_MODE_COUNT: usize = 8;
	
	#[inline(always)]
	pub(crate) fn secure_on_magic_password_is_valid(self) -> bool
	{
		self.contains(WAKE::WAKE_MAGIC | WAKE::WAKE_MAGICSECURE)
	}
}

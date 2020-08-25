// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wake-on-LAN when a condition is met.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(EnumIter)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum WakeOnLanWhen
{
	/// PHY activity.
	PHY = 0,
	
	#[allow(missing_docs)]
	UnicastMessages = 1,
	
	#[allow(missing_docs)]
	MulticastMessages = 2,
	
	#[allow(missing_docs)]
	BroadcastMessages = 3,
	
	#[allow(missing_docs)]
	AddressResolutionProtocolPackets = 4,
	
	#[allow(missing_docs)]
	MagicPacket = 5,
	
	#[allow(missing_docs)]
	Filters = 7,
}

impl WakeOnLanWhen
{
	#[inline(always)]
	pub(crate) const fn to_bitflag(self) -> u32
	{
		1 << (self as u32)
	}
	
	#[inline(always)]
	pub(crate) fn from_bit_flags(bitflags: WAKE) -> HashSet<Self>
	{
		let mut hash_set = HashSet::with_capacity(WAKE::WOL_MODE_COUNT);
		
		for wake_on_lan_mode in Self::iter()
		{
			let bit = unsafe { WAKE::from_bits_unchecked(wake_on_lan_mode.to_bitflag()) };
			if bitflags.contains(bit)
			{
				hash_set.insert(wake_on_lan_mode);
			}
		}
		
		hash_set.shrink_to_fit();
		hash_set
	}
}

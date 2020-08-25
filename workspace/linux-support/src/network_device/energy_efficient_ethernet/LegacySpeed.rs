// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Legacy speed, used by Energy Efficient Ethernet.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumIter, EnumCount)]
#[repr(u32)]
pub enum LegacySpeed
{
	/// String set value is `10baseT/Half`.
	#[serde(rename = "10BASE-T Half")] ETHTOOL_LINK_MODE_10baseT_Half_BIT = 0,
	
	/// String set value is `10baseT/Full`.
	#[serde(rename = "10BASE-T")] ETHTOOL_LINK_MODE_10baseT_Full_BIT = 1,
	
	/// String set value is `100baseT/Half`.
	#[serde(rename = "100BASE-T Half")] ETHTOOL_LINK_MODE_100baseT_Half_BIT = 2,
	
	/// String set value is `100baseT/Full`.
	#[serde(rename = "100BASE-T")] ETHTOOL_LINK_MODE_100baseT_Full_BIT = 3,
	
	/// String set value is `1000baseT/Half`.
	#[serde(rename = "1000BASE-T Half")] ETHTOOL_LINK_MODE_1000baseT_Half_BIT = 4,
	
	/// String set value is `1000baseT/Full`.
	#[serde(rename = "1000BASE-T")] ETHTOOL_LINK_MODE_1000baseT_Full_BIT = 5,
	
	/// String set value is `10000baseT/Full`.
	#[serde(rename = "10GBASE-T")] ETHTOOL_LINK_MODE_10000baseT_Full_BIT = 12,
	
	/// String set value is `2500baseX/Full`.
	#[serde(rename = "2.5GBASE-X")] ETHTOOL_LINK_MODE_2500baseX_Full_BIT = 15,
	
	/// String set value is `1000baseKX/Full`.
	#[serde(rename = "1000BASE-KX")] ETHTOOL_LINK_MODE_1000baseKX_Full_BIT = 17,
	
	/// String set value is `10000baseKX4/Full`.
	#[serde(rename = "10GBASE-KX4")] ETHTOOL_LINK_MODE_10000baseKX4_Full_BIT = 18,
	
	/// String set value is `10000baseKR/Full`.
	#[serde(rename = "10GBASE-KR")] ETHTOOL_LINK_MODE_10000baseKR_Full_BIT = 19,
	
	/// String set value is `10000baseR_FEC`.
	#[serde(rename = "10GBASE-R FEC")] ETHTOOL_LINK_MODE_10000baseR_FEC_BIT = 20,
	
	/// String set value is `20000baseMLD2/Full`.
	#[serde(rename = "20GBASE-MLD2")] ETHTOOL_LINK_MODE_20000baseMLD2_Full_BIT = 21,
	
	/// String set value is `20000baseKR2/Full`.
	#[serde(rename = "20GBASE-KR2")] ETHTOOL_LINK_MODE_20000baseKR2_Full_BIT = 22,
	
	/// String set value is `40000baseKR4/Full`.
	#[serde(rename = "40GBASE-KR4")] ETHTOOL_LINK_MODE_40000baseKR4_Full_BIT = 23,
	
	/// String set value is `40000baseCR4/Full`.
	#[serde(rename = "40GBASE-CR4")] ETHTOOL_LINK_MODE_40000baseCR4_Full_BIT = 24,
	
	/// String set value is `40000baseSR4/Full`.
	#[serde(rename = "40GBASE-SR4")] ETHTOOL_LINK_MODE_40000baseSR4_Full_BIT = 25,
	
	/// String set value is `40000baseLR4/Full`.
	#[serde(rename = "40GBASE-LR4")] ETHTOOL_LINK_MODE_40000baseLR4_Full_BIT = 26,
	
	/// String set value is `56000baseKR4/Full`.
	#[serde(rename = "56GBASE-KR4")] ETHTOOL_LINK_MODE_56000baseKR4_Full_BIT = 27,
	
	/// String set value is `56000baseCR4/Full`.
	#[serde(rename = "56GBASE-CR4")] ETHTOOL_LINK_MODE_56000baseCR4_Full_BIT = 28,
	
	/// String set value is `56000baseSR4/Full`.
	#[serde(rename = "56GBASE-SR4")] ETHTOOL_LINK_MODE_56000baseSR4_Full_BIT = 29,
	
	/// String set value is `56000baseLR4/Full`.
	#[serde(rename = "56GBASE-LR4")] ETHTOOL_LINK_MODE_56000baseLR4_Full_BIT = 30,
	
	/// String set value is `25000baseCR/Full`.
	#[serde(rename = "25GBASE-CR")] ETHTOOL_LINK_MODE_25000baseCR_Full_BIT = 31,
}

impl LegacySpeed
{
	pub(crate) fn from_bit_set_word(bit_set_word: BitSetWord) -> HashSet<Self>
	{
		let mut speeds = HashSet::with_capacity(Self::COUNT);
		for speed in Self::iter()
		{
			if (bit_set_word & (1 << (speed as u32))) != 0
			{
				speeds.insert(speed);
			}
		}
		speeds.shrink_to_fit();
		speeds
	}
}

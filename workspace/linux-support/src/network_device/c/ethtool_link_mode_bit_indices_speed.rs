// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_LINK_MODES` string set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub(crate) enum ethtool_link_mode_bit_indices_speed
{
	/// String set value is `10baseT/Half`.
	#[serde(rename = "10BASE-T Half")] ETHTOOL_LINK_MODE_10baseT_Half_BIT = LegacySpeed::ETHTOOL_LINK_MODE_10baseT_Half_BIT as u32,
	
	/// String set value is `10baseT/Full`.
	#[serde(rename = "10BASE-T")] ETHTOOL_LINK_MODE_10baseT_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_10baseT_Full_BIT as u32,
	
	/// String set value is `100baseT/Half`.
	#[serde(rename = "100BASE-T Half")] ETHTOOL_LINK_MODE_100baseT_Half_BIT = LegacySpeed::ETHTOOL_LINK_MODE_100baseT_Half_BIT as u32,
	
	/// String set value is `100baseT/Full`.
	#[serde(rename = "100BASE-T")] ETHTOOL_LINK_MODE_100baseT_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_100baseT_Full_BIT as u32,
	
	/// String set value is `1000baseT/Half`.
	#[serde(rename = "1000BASE-T Half")] ETHTOOL_LINK_MODE_1000baseT_Half_BIT = LegacySpeed::ETHTOOL_LINK_MODE_1000baseT_Half_BIT as u32,
	
	/// String set value is `1000baseT/Full`.
	#[serde(rename = "1000BASE-T")] ETHTOOL_LINK_MODE_1000baseT_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_1000baseT_Full_BIT as u32,
	
	/// String set value is `10000baseT/Full`.
	#[serde(rename = "10GBASE-T")] ETHTOOL_LINK_MODE_10000baseT_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_10000baseT_Full_BIT as u32,
	
	/// String set value is `2500baseX/Full`.
	#[serde(rename = "2.5GBASE-X")] ETHTOOL_LINK_MODE_2500baseX_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_2500baseX_Full_BIT as u32,
	
	/// String set value is `1000baseKX/Full`.
	#[serde(rename = "1000BASE-KX")] ETHTOOL_LINK_MODE_1000baseKX_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_1000baseKX_Full_BIT as u32,
	
	/// String set value is `10000baseKX4/Full`.
	#[serde(rename = "10GBASE-KX4")] ETHTOOL_LINK_MODE_10000baseKX4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_10000baseKX4_Full_BIT as u32,
	
	/// String set value is `10000baseKR/Full`.
	#[serde(rename = "10GBASE-KR")] ETHTOOL_LINK_MODE_10000baseKR_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_10000baseKR_Full_BIT as u32,
	
	/// String set value is `10000baseR_FEC`.
	#[serde(rename = "10GBASE-R FEC")] ETHTOOL_LINK_MODE_10000baseR_FEC_BIT = LegacySpeed::ETHTOOL_LINK_MODE_10000baseR_FEC_BIT as u32,
	
	/// String set value is `20000baseMLD2/Full`.
	#[serde(rename = "20GBASE-MLD2")] ETHTOOL_LINK_MODE_20000baseMLD2_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_20000baseMLD2_Full_BIT as u32,
	
	/// String set value is `20000baseKR2/Full`.
	#[serde(rename = "20GBASE-KR2")] ETHTOOL_LINK_MODE_20000baseKR2_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_20000baseKR2_Full_BIT as u32,
	
	/// String set value is `40000baseKR4/Full`.
	#[serde(rename = "40GBASE-KR4")] ETHTOOL_LINK_MODE_40000baseKR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_40000baseKR4_Full_BIT as u32,
	
	/// String set value is `40000baseCR4/Full`.
	#[serde(rename = "40GBASE-CR4")] ETHTOOL_LINK_MODE_40000baseCR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_40000baseCR4_Full_BIT as u32,
	
	/// String set value is `40000baseSR4/Full`.
	#[serde(rename = "40GBASE-SR4")] ETHTOOL_LINK_MODE_40000baseSR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_40000baseSR4_Full_BIT as u32,
	
	/// String set value is `40000baseLR4/Full`.
	#[serde(rename = "40GBASE-LR4")] ETHTOOL_LINK_MODE_40000baseLR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_40000baseLR4_Full_BIT as u32,
	
	/// String set value is `56000baseKR4/Full`.
	#[serde(rename = "56GBASE-KR4")] ETHTOOL_LINK_MODE_56000baseKR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_56000baseKR4_Full_BIT as u32,
	
	/// String set value is `56000baseCR4/Full`.
	#[serde(rename = "56GBASE-CR4")] ETHTOOL_LINK_MODE_56000baseCR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_56000baseCR4_Full_BIT as u32,
	
	/// String set value is `56000baseSR4/Full`.
	#[serde(rename = "56GBASE-SR4")] ETHTOOL_LINK_MODE_56000baseSR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_56000baseSR4_Full_BIT as u32,
	
	/// String set value is `56000baseLR4/Full`.
	#[serde(rename = "56GBASE-LR4")] ETHTOOL_LINK_MODE_56000baseLR4_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_56000baseLR4_Full_BIT as u32,
	
	/// String set value is `25000baseCR/Full`.
	#[serde(rename = "25GBASE-CR")] ETHTOOL_LINK_MODE_25000baseCR_Full_BIT = LegacySpeed::ETHTOOL_LINK_MODE_25000baseCR_Full_BIT as u32,
	
	/// String set value is `25000baseKR/Full`.
	#[serde(rename = "25GBASE-KR")] ETHTOOL_LINK_MODE_25000baseKR_Full_BIT = 32,
	
	/// String set value is `25000baseSR/Full`.
	#[serde(rename = "25GBASE-SR")] ETHTOOL_LINK_MODE_25000baseSR_Full_BIT = 33,
	
	/// String set value is `50000baseCR2/Full`.
	#[serde(rename = "50GBASE-CR2")] ETHTOOL_LINK_MODE_50000baseCR2_Full_BIT = 34,
	
	/// String set value is `50000baseKR2/Full`.
	#[serde(rename = "50GBASE-KR2")] ETHTOOL_LINK_MODE_50000baseKR2_Full_BIT = 35,
	
	/// String set value is `100000baseKR4/Full`.
	#[serde(rename = "100GBASE-KR4")] ETHTOOL_LINK_MODE_100000baseKR4_Full_BIT = 36,
	
	/// String set value is `100000baseSR4/Full`.
	#[serde(rename = "100GBASE-SR4")] ETHTOOL_LINK_MODE_100000baseSR4_Full_BIT = 37,
	
	/// String set value is `100000baseCR4/Full`.
	#[serde(rename = "100GBASE-CR4")] ETHTOOL_LINK_MODE_100000baseCR4_Full_BIT = 38,
	
	/// String set value is `100000baseLR4_ER4/Full`.
	#[serde(rename = "100GBASE-LR4_ER4")] ETHTOOL_LINK_MODE_100000baseLR4_ER4_Full_BIT = 39,
	
	/// String set value is `50000baseSR2/Full`.
	#[serde(rename = "50GBASE-SR2")] ETHTOOL_LINK_MODE_50000baseSR2_Full_BIT = 40,
	
	/// String set value is `1000baseX/Full`.
	#[serde(rename = "1000BASE-X")] ETHTOOL_LINK_MODE_1000baseX_Full_BIT = 41,
	
	/// String set value is `10000baseCR/Full`.
	#[serde(rename = "10GBASE-CR")] ETHTOOL_LINK_MODE_10000baseCR_Full_BIT = 42,
	
	/// String set value is `10000baseSR/Full`.
	#[serde(rename = "10GBASE-SR")] ETHTOOL_LINK_MODE_10000baseSR_Full_BIT = 43,
	
	/// String set value is `10000baseLR/Full`.
	#[serde(rename = "10GBASE-LR")] ETHTOOL_LINK_MODE_10000baseLR_Full_BIT = 44,
	
	/// String set value is `10000baseLRM/Full`.
	#[serde(rename = "10GBASE-LRM")] ETHTOOL_LINK_MODE_10000baseLRM_Full_BIT = 45,
	
	/// String set value is `10000baseER/Full`.
	#[serde(rename = "10GBASE-ER")] ETHTOOL_LINK_MODE_10000baseER_Full_BIT = 46,
	
	/// String set value is `2500baseT/Full`.
	#[serde(rename = "2.5GBASE-T")] ETHTOOL_LINK_MODE_2500baseT_Full_BIT = 47,
	
	/// String set value is `5000baseT/Full`.
	#[serde(rename = "5GBASE-T")] ETHTOOL_LINK_MODE_5000baseT_Full_BIT = 48,
	
	/// String set value is `50000baseKR/Full`.
	#[serde(rename = "50GBASE-KR")] ETHTOOL_LINK_MODE_50000baseKR_Full_BIT = 52,
	
	/// String set value is `50000baseSR/Full`.
	#[serde(rename = "50GBASE-SR")] ETHTOOL_LINK_MODE_50000baseSR_Full_BIT = 53,
	
	/// String set value is `50000baseCR/Full`.
	#[serde(rename = "50GBASE-CR")] ETHTOOL_LINK_MODE_50000baseCR_Full_BIT = 54,
	
	/// String set value is `50000baseLR_ER_FR/Full`.
	#[serde(rename = "50GBASE-LR_ER_FR")] ETHTOOL_LINK_MODE_50000baseLR_ER_FR_Full_BIT = 55,
	
	/// String set value is `50000baseDR/Full`.
	#[serde(rename = "50GBASE-DR")] ETHTOOL_LINK_MODE_50000baseDR_Full_BIT = 56,
	
	/// String set value is `100000baseKR2/Full`.
	#[serde(rename = "100GBASE-KR2")] ETHTOOL_LINK_MODE_100000baseKR2_Full_BIT = 57,
	
	/// String set value is `100000baseSR2/Full`.
	#[serde(rename = "100GBASE-SR2")] ETHTOOL_LINK_MODE_100000baseSR2_Full_BIT = 58,
	
	/// String set value is `100000baseCR2/Full`.
	#[serde(rename = "100GBASE-CR2")] ETHTOOL_LINK_MODE_100000baseCR2_Full_BIT = 59,
	
	/// String set value is `100000baseLR2_ER2_FR2/Full`.
	#[serde(rename = "100GBASE-LR2_ER2_FR2")] ETHTOOL_LINK_MODE_100000baseLR2_ER2_FR2_Full_BIT = 60,
	
	/// String set value is `100000baseDR2/Full`.
	#[serde(rename = "100GBASE-DR2")] ETHTOOL_LINK_MODE_100000baseDR2_Full_BIT = 61,
	
	/// String set value is `200000baseKR4/Full`.
	#[serde(rename = "200GBASE-KR4")] ETHTOOL_LINK_MODE_200000baseKR4_Full_BIT = 62,
	
	/// String set value is `200000baseSR4/Full`.
	#[serde(rename = "200GBASE-SR4")] ETHTOOL_LINK_MODE_200000baseSR4_Full_BIT = 63,
	
	/// String set value is `200000baseLR4_ER4_FR4/Full`.
	#[serde(rename = "200GBASE-LR4_ER4_FR4")] ETHTOOL_LINK_MODE_200000baseLR4_ER4_FR4_Full_BIT = 64,
	
	/// String set value is `200000baseDR4/Full`.
	#[serde(rename = "200GBASE-DR4")] ETHTOOL_LINK_MODE_200000baseDR4_Full_BIT = 65,
	
	/// String set value is `200000baseCR4/Full`.
	#[serde(rename = "200GBASE-CR4")] ETHTOOL_LINK_MODE_200000baseCR4_Full_BIT = 66,
	
	/// String set value is `100baseT1/Full`.
	#[serde(rename = "100BASE-T1")] ETHTOOL_LINK_MODE_100baseT1_Full_BIT = 67,
	
	/// String set value is `1000baseT1/Full`.
	#[serde(rename = "1000BASE-T1")] ETHTOOL_LINK_MODE_1000baseT1_Full_BIT = 68,
	
	/// String set value is `400000baseKR8/Full`.
	#[serde(rename = "40GBASE-KR8")] ETHTOOL_LINK_MODE_400000baseKR8_Full_BIT = 69,
	
	/// String set value is `400000baseSR8/Full`.
	#[serde(rename = "40GBASE-SR8")] ETHTOOL_LINK_MODE_400000baseSR8_Full_BIT = 70,
	
	/// String set value is `400000baseLR8_ER8_FR8/Full`.
	#[serde(rename = "40GBASE-LR8_ER8_FR8")] ETHTOOL_LINK_MODE_400000baseLR8_ER8_FR8_Full_BIT = 71,
	
	/// String set value is `400000baseDR8/Full`.
	#[serde(rename = "40GBASE-DR8")] ETHTOOL_LINK_MODE_400000baseDR8_Full_BIT = 72,
	
	/// String set value is `400000baseCR8/Full`.
	#[serde(rename = "40GBASE-CR8")] ETHTOOL_LINK_MODE_400000baseCR8_Full_BIT = 73,
}

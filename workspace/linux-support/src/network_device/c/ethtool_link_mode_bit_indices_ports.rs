// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_LINK_MODES` string set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub(crate) enum ethtool_link_mode_bit_indices_ports
{
	/// String set value is `Autoneg`.
	ETHTOOL_LINK_MODE_Autoneg_BIT = 6,
	
	/// String set value is `TP`.
	ETHTOOL_LINK_MODE_TP_BIT = 7,
	
	/// String set value is `AUI`.
	ETHTOOL_LINK_MODE_AUI_BIT = 8,
	
	/// String set value is `MII`.
	ETHTOOL_LINK_MODE_MII_BIT = 9,
	
	/// String set value is `FIBRE`.
	ETHTOOL_LINK_MODE_FIBRE_BIT = 10,
	
	/// String set value is `BNC`.
	ETHTOOL_LINK_MODE_BNC_BIT = 11,
	
	/// String set value is `Backplane`.
	ETHTOOL_LINK_MODE_Backplane_BIT = 16,
}

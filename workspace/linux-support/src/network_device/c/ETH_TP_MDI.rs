// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// MDI or MDI-X status/control.
///
/// If one of `ETH_TP_MDI`, `ETH_TP_MDI_X`, or `ETH_TP_MDI_AUTO` is set in `ethtool_link_settings.eth_tp_mdix_ctrl` then the driver is required to renegotiate the link.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum ETH_TP_MDI
{
	/// For `ethtool_link_settings.eth_tp_mdix`: unknown.
	/// For `ethtool_link_settings.eth_tp_mdix_ctrl`: unsupported.
	ETH_TP_MDI_INVALID = 0x00,
	
	/// For `ethtool_link_settings.eth_tp_mdix`: MDI.
	/// For `ethtool_link_settings.eth_tp_mdix_ctrl`: Force MDI.
	///
	/// Ethtool setting is `off`.
	ETH_TP_MDI = 0x01,
	
	/// For `ethtool_link_settings.eth_tp_mdix`: MDI-X.
	/// For `ethtool_link_settings.eth_tp_mdix_ctrl`: Force MDI-X.
	///
	/// Ethtool setting is `on`.
	ETH_TP_MDI_X = 0x02,
	
	/// For `ethtool_link_settings.eth_tp_mdix`: Should never be valid.
	/// For `ethtool_link_settings.eth_tp_mdix_ctrl`: Auto-Select.
	///
	/// Ethtool setting is `auto`.
	ETH_TP_MDI_AUTO = 0x03,
}

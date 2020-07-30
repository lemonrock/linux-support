// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used by `phy_tunable_id::ETHTOOL_PHY_FAST_LINK_DOWN`.
///
/// Lowest time supported by the PHY.
pub(crate) const ETHTOOL_PHY_FAST_LINK_DOWN_ON: u32 = 0;

/// Used by `phy_tunable_id::ETHTOOL_PHY_FAST_LINK_DOWN`.
///
/// Off, link down detection according to standard.
pub(crate) const ETHTOOL_PHY_FAST_LINK_DOWN_OFF: u32 = 0xFF;

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Link modes.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ethtool_link_settings_link_modes
{
	/// `supported` is a bit set with each bit meaning given by `ethtool_link_mode_bit_indices` for the link modes, physical connectors and other link features for which the interface supports autonegotiation or auto-detection.
	///
	/// Read-only.
	pub(crate) supported: LinkModeBitSet,
	
	/// `advertising` is a bit set with each bit meaning given by `ethtool_link_mode_bit_indices` for the link modes, physical connectors and other link features that are advertised through autonegotiation or enabled for auto-detection.
	///
	/// Read-write.
	pub(crate) advertising: LinkModeBitSet,
	
	/// `lp_advertising` is a bit set with each bit meaning given by `ethtool_link_mode_bit_indices` for the link modes, physical connectors and other link features that the link partner advertised through autonegotiation.
	///
	/// Read-only.
	pub(crate) lp_advertising: LinkModeBitSet,
}

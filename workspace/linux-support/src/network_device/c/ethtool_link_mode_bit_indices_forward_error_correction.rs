// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Forward Error Correction (FEC).
///
/// Strings are in the `ethtool_stringset::ETH_SS_LINK_MODES` string set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub(crate) enum ethtool_link_mode_bit_indices_forward_error_correction
{
	/// String set value is `None`.
	ETHTOOL_LINK_MODE_FEC_NONE_BIT = 49,
	
	/// String set value is `RS`.
	ETHTOOL_LINK_MODE_FEC_RS_BIT = 50,
	
	/// 'BaseR'.
	///
	/// String set value is `BASER`.
	ETHTOOL_LINK_MODE_FEC_BASER_BIT = 51,
	
	/// String set value is `LLRS`.
	ETHTOOL_LINK_MODE_FEC_LLRS_BIT = 74,
}

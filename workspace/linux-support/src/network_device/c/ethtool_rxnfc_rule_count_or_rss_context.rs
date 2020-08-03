// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ethtool_rxnfc_rule_count_or_rss_context
{
	/// Number of rules to be affected.
	///
	/// For `ETHTOOL_GRXCLSRLCNT`, the number of defined rules on return.
	///
	/// For `ETHTOOL_GRXCLSRLALL`, specifies the array size of the user buffer for `rules_loc` on entry.
	/// For `ETHTOOL_GRXCLSRLALL`, specifies the number of defined rules on return.
	pub(crate) rule_cnt: u32,
	
	/// RSS context to be affected.
	pub(crate) rss_context: Option<ContextIdentifier>,
}

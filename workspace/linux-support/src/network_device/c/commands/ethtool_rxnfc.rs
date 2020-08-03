// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Commands to get or set RX flow classification rules.
///
/// The special locations dicussed below are the values:-
///
/// * `RX_CLS_LOC_ANY`.
/// * `RX_CLS_LOC_FIRST`.
/// * `RX_CLS_LOC_LAST`.
#[repr(C)]
pub(crate) struct ethtool_rxnfc
{
	/// One of:-
	///
	/// * `ETHTOOL_GRXFH` or `ETHTOOL_SRXFH`.
	/// * `ETHTOOL_GRXRINGS`.
	/// * `ETHTOOL_GRXCLSRLCNT`.
	/// * `ETHTOOL_GRXCLSRULE`.
	/// * `ETHTOOL_GRXCLSRLALL`.
	/// * `ETHTOOL_SRXCLSRLDEL`.
	/// * `ETHTOOL_SRXCLSRLINS`.
	pub(crate) cmd: u32,
	
	/// Type of flow to be affected, eg `TCP_V4_FLOW`.
	pub(crate) flow_type: u32,
	
	/// Command (`cmd`) dependent value.
	///
	/// For `ETHTOOL_GRXFH` and `ETHTOOL_SRXFH`, a bit mask indicating the fields included in the flow hash, eg `RXH_IP_SRC`.
	/// In this case, `fs`, `rule_count_or_rss_context` and `rule_locs` must not be used unless `flow_type` includes `FLOW_RSS`, in which case `rule_count_or_rss_context.rss_context` determines the RSS context to act on.
	///
	/// For `ETHTOOL_GRXRINGS`, the number of receive ring queues on return.
	///
	/// For `ETHTOOL_GRXCLSRLCNT`, if non-zero on return, then it is the size of the rule table or'd with the flag `RX_CLS_LOC_SPECIAL` if the dirver supports any special location values. If `RX_CLS_LOC_SPECIAL` is not set then the driver does not support special locaton values.
	///
	/// For `ETHTOOL_GRXCLSRLALL`, on return, the size of the rule table.
	pub(crate) data: u64,
	
	/// Flow classification rule.
	///
	/// For `ETHTOOL_GRXCLSRULE`:-
	/// * `fs.location` specifies the location of an existing rule on entry.
	/// * `fs` contains the rule on return.
	/// * `rule_count_or_rss_context.rss_context` contains the context associated with the rule if `fs.flow_type` includes the `FLOW_RSS` flag.
	///
	/// For `ETHTOOL_SRXCLSRLINS`, `fs` specifies the rule to add or upate.
	/// On entry, `fs.location` either specifies the location to use or is a special location value (the `RX_CLS_LOC_SPECIAL` flag is set).
	/// On return, `fs.location` is the actual rule location.
	/// If `fs.flow_type` includes the `FLOW_RSS` flag, `rule_count_or_rss_context.rss_context` is the rule's context.
	/// "The value from the rxfh indirection table will be added to `fs.ring_cookie` to choose which ring to deliver to".
	///
	/// For `ETHTOOL_SRXCLSRLDEL`, `fs.location` specifies the location of an exist rule on entry.
	pub(crate) fs: ethtool_rx_flow_spec,
	
	pub(crate) rule_count_or_rss_context: ethtool_rxnfc_rule_count_or_rss_context,
	
	/// Array of used rule locations.
	///
	/// Array size is in `.rule_cnt` for the command `ETHTOOL_GRXCLSRLALL`.
	///
	/// For `ETHTOOL_GRXCLSRLALL`, specifies the defined rules on return.
	pub(crate) rule_locs: __IncompleteArrayField<u32>,
}

impl EthtoolCommand for ethtool_rxnfc
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Classification rule for receive flows.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ethtool_rx_flow_spec
{
	/// Type of match to perform, eg `TCP_V4_FLOW`.
	pub(crate) flow_type: u32,
	
	/// Flow fields to match (dependent on `flow_type`).
	pub(crate) h_u: ethtool_flow_union,
	
	/// Additional flow fields to match.
	pub(crate) h_ext: ethtool_flow_ext,
	
	/// Masks for flow field bits to be matched.
	pub(crate) m_u: ethtool_flow_union,
	
	/// Masks for additional field bits to be matched.
	pub(crate) m_ext: ethtool_flow_ext,
	
	/// Receive ring queue index to deliver to, or:-
	///
	/// * `RX_CLS_FLOW_DISC` if packets should be discarded;
	/// * `RX_CLS_FLOW_WAKE` if packets should be used for Wake-on-LAN with `WAKE_FILTER`.
	pub(crate) ring_cookie: RingCookie,
	
	/// Location of rule in the table.
	///
	/// Locations must be numbered such that a flow matching multiple rules will be classified according to the first (lowest numbered) rule.
	pub(crate) location: CombinedRuleLocation,
}

impl ethtool_rx_flow_spec
{
	#[inline(always)]
	pub(crate) const fn actual_flow_type(&self) -> u32
	{
		self.flow_type & !(FLOW_EXT | FLOW_MAC_EXT | FLOW_RSS)
	}
	
	#[inline(always)]
	pub(crate) const fn has_extended_flow_type(&self) -> bool
	{
		self.flow_type & FLOW_EXT != 0
	}
	
	#[inline(always)]
	pub(crate) const fn has_extended_media_access_control_flow_type(&self) -> bool
	{
		self.flow_type & FLOW_MAC_EXT != 0
	}
	
	#[inline(always)]
	pub(crate) const fn has_receive_side_scaling_flow_type(&self) -> bool
	{
		self.flow_type & FLOW_RSS != 0
	}
}

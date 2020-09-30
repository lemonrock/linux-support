// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How rings are laid out when accessing virtual functions or offloaded queues is device specific.
///
/// To allow users to do flow steering and specify these queues the ring cookie is partitioned into a 32bit queue index with an 8 bit virtual function id.
/// This also leaves the 3 bytes for further specifiers; it is possible future devices may support more than 256 virtual functions (eg if using PCIe with ARI).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct RingCookie(pub(crate) u64);

impl TryInto<RuleAction> for RingCookie
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<RuleAction, Self::Error>
	{
		use self::RuleAction::*;
		
		let is_action_drop = self.0 == RX_CLS_FLOW_DISC;
		if is_action_drop
		{
			return Ok(Drop)
		}
		
		let is_action_wake_on_lan = self.0 == RX_CLS_FLOW_WAKE;
		if is_action_wake_on_lan
		{
			return Ok(WakeOnLan)
		}
		
		let raw_virtual_function_index = self.ethtool_get_flow_spec_ring_vf();
		let queue_identifier = QueueIdentifier::try_from(self.ethtool_get_flow_spec_ring())?;
		
		let virtual_function_index = if raw_virtual_function_index == 0
		{
			None
		}
		else
		{
			Some(VirtualFunctionIndex(raw_virtual_function_index - 1))
		};
		
		Ok(DirectToQueue { virtual_function_index, queue_identifier })
	}
}

impl RingCookie
{
	#[inline(always)]
	const fn ethtool_get_flow_spec_ring(self) -> u32
	{
		(self.0 & ETHTOOL_RX_FLOW_SPEC_RING) as u32
	}
	
	#[inline(always)]
	const fn ethtool_get_flow_spec_ring_vf(self) -> u8
	{
		((self.0 & ETHTOOL_RX_FLOW_SPEC_RING_VF) >> ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF) as u8
	}
}

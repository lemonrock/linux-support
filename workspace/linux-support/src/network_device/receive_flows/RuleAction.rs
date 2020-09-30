// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ring cookie action.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum RuleAction
{
	/// Drop (also known as Discard and `Disc`).
	Drop,
	
	/// Wake-on-LAN.
	WakeOnLan,
	
	/// Direct to queue.
	DirectToQueue
	{
		/// Virtual function 'namespacing' the queue.
		///
		/// If `None`, then this is the main (physical) function.
		virtual_function_index: Option<VirtualFunctionIndex>,
		
		/// Queue to direct to.
		queue_identifier: QueueIdentifier,
	}
}

impl Default for RuleAction
{
	#[inline(always)]
	fn default() -> Self
	{
		RuleAction::Drop
	}
}

impl Into<RingCookie> for RuleAction
{
	#[inline(always)]
	fn into(self) -> RingCookie
	{
		use self::RuleAction::*;
		
		let value = match self
		{
			Drop => RX_CLS_FLOW_DISC,
			
			WakeOnLan => RX_CLS_FLOW_WAKE,
			
			DirectToQueue { queue_identifier, virtual_function_index: None } => queue_identifier.into_u64(),
			
			DirectToQueue { queue_identifier, virtual_function_index: Some(virtual_function_index) } =>
			{
				let queue_identifier = queue_identifier.into_u64();
				let virtual_function_index_plus_one = (virtual_function_index.0 as u64) + 1;
				virtual_function_index_plus_one << ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF | queue_identifier
			}
		};
		RingCookie(value)
	}
}

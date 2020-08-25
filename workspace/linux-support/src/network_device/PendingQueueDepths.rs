// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pending queue depths.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PendingQueueDepths
{
	receive_pending_queue_depth: Option<QueueDepth>,
	
	receive_mini_pending_queue_depth: Option<QueueDepth>,
	
	receive_jumbo_pending_queue_depth: Option<QueueDepth>,
	
	transmit_pending_queue_depth: Option<QueueDepth>,
}

impl PendingQueueDepths
{
	const Unsupported: Self = Self::new(None, None, None, None);
	
	#[inline(always)]
	pub(crate) fn new(receive_pending_queue_depth: Option<QueueDepth>, receive_jumbo_pending_queue_depth: Option<QueueDepth>, receive_mini_pending_queue_depth: Option<QueueDepth>, transmit_pending_queue_depth: Option<QueueDepth>) -> Self
	{
		Self
		{
			receive_pending_queue_depth,
			receive_jumbo_pending_queue_depth,
			receive_mini_pending_queue_depth,
			transmit_pending_queue_depth,
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fill and Completion ring queue depths.
pub trait FillOrCompletionOrBothRingQueueDepths: RingQueueDepths
{
	/// Related to `Received::receive()`.
	fn fill_ring_queue_depth_or_default(&self) -> RingQueueDepth;
	
	/// Related to `Transmits::transmit()`.
	fn completion_ring_queue_depth_or_default(&self) -> RingQueueDepth;
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SharedReceiveTransmitMemoryRingQueues<'shared>
{
	user_memory: &'shared UserMemory,
	
	/// receive is `xsk_ring_cons`.
	/// transmit is `xsk_ring_prod`.
	receive_and_transmit: ReceiveOrTransmitOrBoth<XskRingQueue>,
	
	xsk_socket_file_descriptor: ExpressDataPathSocketFileDescriptor,
}

impl ReceiveTransmitMemoryRingQueues for SharedReceiveTransmitMemoryRingQueues
{
	#[inline(always)]
	fn user_memory_and_receive_transmit(&self) -> (&UserMemory, &ReceiveOrTransmitOrBoth<XskRingQueue>)
	{
		(self.user_memory, &self.receive_and_transmit)
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
#[derive(Debug)]
pub struct SharedReceiveTransmitMemoryRingQueues<'shared>
{
	user_memory: &'shared UserMemory,
	
	xdp_extended_bpf_program: &'shared RedirectMapAndAttachedProgram,
	
	receive_and_transmit: ManuallyDrop<ReceiveOrTransmitOrBoth<XskRingQueue<ConsumerXskRingQueueKind, xdp_desc>, XskRingQueue<ProducerXskRingQueueKind, xdp_desc>>>,
	
	express_data_path_socket_file_descriptor: ManuallyDrop<ExpressDataPathSocketFileDescriptor>,
	
	queue_identifier: QueueIdentifier,
}

impl Drop for SharedReceiveTransmitMemoryRingQueues<'_>
{
	fn drop(&mut self)
	{
		if self.receive_and_transmit.is_receive_or_both()
		{
			// Based on `libbpf`'s `xsk_delete_bpf_maps()`.
			let _ignored = self.xdp_extended_bpf_program.redirect_map.delete(self.queue_identifier);
		}
		
		unsafe
		{
			ManuallyDrop::drop(&mut self.express_data_path_socket_file_descriptor);
			ManuallyDrop::drop(&mut self.receive_and_transmit);
		}
	}
}

impl Deref for SharedReceiveTransmitMemoryRingQueues<'_>
{
	type Target = UserMemory;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.user_memory
	}
}

impl ReceiveTransmitMemoryRingQueues for SharedReceiveTransmitMemoryRingQueues<'_>
{
	#[inline(always)]
	fn receive_transmit(&self) -> &ReceiveOrTransmitOrBoth<XskRingQueue<ConsumerXskRingQueueKind, xdp_desc>, XskRingQueue<ProducerXskRingQueueKind, xdp_desc>>
	{
		&self.receive_and_transmit
	}
	
	/// Statistics.
	#[inline(always)]
	fn statistics(&self) -> xdp_statistics
	{
		self.express_data_path_socket_file_descriptor.statistics()
	}
	
	/// Options.
	#[inline(always)]
	fn options(&self) -> xdp_options
	{
		self.express_data_path_socket_file_descriptor.options()
	}
}

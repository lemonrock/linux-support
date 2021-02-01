// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive only.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RingQueueDepthReceiveOnly<RPC: ReceivePollCreator>(RingQueueDepth, PhantomData<RPC>);

impl<RPC: ReceivePollCreator> Supports for RingQueueDepthReceiveOnly<RPC>
{
	const SupportsReceive: bool = true;
	
	const SupportsTransmit: bool = false;
}

impl<RPC: ReceivePollCreator> RingQueueDepths for RingQueueDepthReceiveOnly<RPC>
{
}

impl<RPC: ReceivePollCreator> Receives<RingQueueDepth> for RingQueueDepthReceiveOnly<RPC>
{
	#[inline(always)]
	fn receive(&self) -> &RingQueueDepth
	{
		&self.0
	}
}

impl<RPC: ReceivePollCreator> FillOrCompletionOrBothRingQueueDepths for RingQueueDepthReceiveOnly<RPC>
{
	#[inline(always)]
	fn fill_ring_queue_depth_or_default(&self) -> RingQueueDepth
	{
		self.0
	}
	
	#[inline(always)]
	fn completion_ring_queue_depth_or_default(&self) -> RingQueueDepth
	{
		RingQueueDepth::default()
	}
}

impl<RPC: ReceivePollCreator> CreateReceiveOrTransmitOrBoth for RingQueueDepthReceiveOnly<RPC>
{
	type Arguments = RPC;
	
	type ReceiveOrTransmitOrBoth = CommonReceiveOnly<RPC::RP>;
	
	#[inline(always)]
	fn set_ring_queue_depths(&self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor)
	{
		express_data_path_socket_file_descriptor.set_xdp_socket_option_receive_ring(self.receive())
	}
	
	#[inline(always)]
	fn create_receive_or_transmit_or_both(self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, default_page_size: PageSize, memory_map_offsets: &xdp_mmap_offsets, receive_queue_identifier: QueueIdentifier, redirect_map_and_attached_program: &RedirectMapAndAttachedProgram, arguments: Self::Arguments) -> Result<Self::ReceiveOrTransmitOrBoth, ExpressDataPathSocketCreationError>
	{
		let receive_ring_queue_depth = self.0;
		let common = CommonReceiveOnly::new
		(
			ReceiveQueue::from_receive_memory_map_offsets(express_data_path_socket_file_descriptor, memory_map_offsets, receive_ring_queue_depth, default_page_size),
			arguments.create(express_data_path_socket_file_descriptor),
			receive_queue_identifier,
		);
		
		// Based on call to `enter_xsks_into_map()` in `main()` in Linux source `samples/bpf/xdpsock_user.c`.
		redirect_map_and_attached_program.insert_into_redirect_map_if_receive(receive_queue_identifier, &express_data_path_socket_file_descriptor)?;
		
		Ok(common)
	}
}

impl<RPC: ReceivePollCreator> RingQueueDepthReceiveOnly<RPC>
{
	/// Create a new instance.
	#[inline(always)]
	pub fn new(fill_or_receive_ring_queue_depth: RingQueueDepth) -> Self
	{
		Self(fill_or_receive_ring_queue_depth, PhantomData)
	}
}

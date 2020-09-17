// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and Transmit separately.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RingQueueDepthReceiveAndTransmitSeparately(RingQueueDepthReceiveOnly, RingQueueDepthTransmitOnly);

impl Supports for RingQueueDepthReceiveAndTransmitSeparately
{
	const SupportsReceive: bool = true;
	
	const SupportsTransmit: bool = true;
}

impl RingQueueDepths for RingQueueDepthReceiveAndTransmitSeparately
{
}

impl Receives for RingQueueDepthReceiveAndTransmitSeparately
{
	#[inline(always)]
	fn receive(&self) -> &RingQueueDepth
	{
		self.0.receive()
	}
}

impl Transmits for RingQueueDepthReceiveAndTransmitSeparately
{
	#[inline(always)]
	fn transmit(&self) -> &RingQueueDepth
	{
		self.1.transmit()
	}
}

impl FillOrCompletionOrBothRingQueueDepths for RingQueueDepthReceiveAndTransmitSeparately
{
	#[inline(always)]
	fn fill_ring_queue_depth_or_default(&self) -> RingQueueDepth
	{
		self.0.fill_ring_queue_depth_or_default()
	}
	
	#[inline(always)]
	fn completion_ring_queue_depth_or_default(&self) -> RingQueueDepth
	{
		self.1.completion_ring_queue_depth_or_default()
	}
}

impl<RPC: ReceivePollCreator> CreateReceiveOrTransmitOrBoth for RingQueueDepthReceiveAndTransmitSeparately
{
	type Arguments = RPC;
	
	type ReceiveOrTransmitOrBoth = CommonReceiveAndTransmitSeparately<RPC::RP>;
	
	#[inline(always)]
	fn set_ring_queue_depths(&self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor)
	{
		self.0.set_ring_queue_depths(express_data_path_socket_file_descriptor);
		self.1.set_ring_queue_depths(express_data_path_socket_file_descriptor);
	}
	
	#[inline(always)]
	fn create_receive_or_transmit_or_both(self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, defaults: &DefaultPageSizeAndHugePageSizes, memory_map_offsets: &xdp_mmap_offset, receive_queue_identifier: QueueIdentifier, redirect_map_and_attached_program: &RedirectMapAndAttachedProgram, arguments: Self::Arguments) -> Result<Self::ReceiveOrTransmitOrBoth, ExpressDataPathSocketCreationError>
	{
		Ok
		(
			CommonReceiveAndTransmitSeparately::new
			(
				self.0.create_receive_or_transmit_or_both(express_data_path_socket_file_descriptor, defaults, memory_map_offsets, receive_queue_identifier, redirect_map_and_attached_program, arguments)?,
				self.1.create_receive_or_transmit_or_both(express_data_path_socket_file_descriptor, defaults, memory_map_offsets, receive_queue_identifier, redirect_map_and_attached_program, ())?,
			)
		)
	}
}

impl RingQueueDepthReceiveAndTransmitSeparately
{
	/// Create a new instance.
	#[inline(always)]
	pub const fn new(fill_or_receive_ring_queue_depth: RingQueueDepth, completion_or_transmit_ring_queue_depth: RingQueueDepth) -> Self
	{
		Self
		(
			RingQueueDepthReceiveOnly::new(fill_or_receive_ring_queue_depth),
			RingQueueDepthTransmitOnly::new(completion_or_transmit_ring_queue_depth)
		)
	}
}

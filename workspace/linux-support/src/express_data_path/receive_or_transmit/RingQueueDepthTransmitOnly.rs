// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit only.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RingQueueDepthTransmitOnly<TSC: TransmitSendCreator>(RingQueueDepth, PhantomData<TSC>);

impl<TSC: TransmitSendCreator> Supports for RingQueueDepthTransmitOnly<TSC>
{
	const SupportsReceive: bool = false;
	
	const SupportsTransmit: bool = true;
}

impl<TSC: TransmitSendCreator> RingQueueDepths for RingQueueDepthTransmitOnly<TSC>
{
}

impl<TSC: TransmitSendCreator> Transmits<RingQueueDepth> for RingQueueDepthTransmitOnly<TSC>
{
	#[inline(always)]
	fn transmit(&self) -> &RingQueueDepth
	{
		&self.0
	}
}

impl<TSC: TransmitSendCreator> FillOrCompletionOrBothRingQueueDepths for RingQueueDepthTransmitOnly<TSC>
{
	#[inline(always)]
	fn fill_ring_queue_depth_or_default(&self) -> RingQueueDepth
	{
		RingQueueDepth::default()
	}
	
	#[inline(always)]
	fn completion_ring_queue_depth_or_default(&self) -> RingQueueDepth
	{
		self.0
	}
}

impl<TSC: TransmitSendCreator> CreateReceiveOrTransmitOrBoth for RingQueueDepthTransmitOnly<TSC>
{
	type Arguments = TSC;
	
	type ReceiveOrTransmitOrBoth = CommonTransmitOnly<TSC::TS>;
	
	#[inline(always)]
	fn set_ring_queue_depths(&self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor)
	{
		express_data_path_socket_file_descriptor.set_xdp_socket_option_transmit_ring(self.transmit())
	}
	
	#[inline(always)]
	fn create_receive_or_transmit_or_both(self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, defaults: &DefaultPageSizeAndHugePageSizes, memory_map_offsets: &xdp_mmap_offsets, _queue_identifier: QueueIdentifier, _redirect_map_and_attached_program: &RedirectMapAndAttachedProgram, arguments: Self::Arguments) -> Result<Self::ReceiveOrTransmitOrBoth, ExpressDataPathSocketCreationError>
	{
		Ok
		(
			CommonTransmitOnly::new
			(
				TransmitQueue::from_transmit_memory_map_offsets(express_data_path_socket_file_descriptor, memory_map_offsets, self.0, defaults),
				arguments.create(express_data_path_socket_file_descriptor)
			)
		)
	}
}

impl<TSC: TransmitSendCreator> RingQueueDepthTransmitOnly<TSC>
{
	/// Create a new instance.
	#[inline(always)]
	pub const fn new(completion_or_transmit_ring_queue_depth: RingQueueDepth) -> Self
	{
		Self(completion_or_transmit_ring_queue_depth, PhantomData)
	}
}

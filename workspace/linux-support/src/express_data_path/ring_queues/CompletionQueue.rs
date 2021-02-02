// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Used in conjunction with the `TransmitQueue`.
#[doc(hidden)]
pub type CompletionQueue = XskRingQueue<ConsumerXskRingQueueKind, FrameDescriptorBitfield>;

impl CompletionQueue
{
	#[inline(always)]
	pub(crate) fn from_completion_memory_map_offsets(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, completion_ring_queue_depth: RingQueueDepth) -> Self
	{
		Self::from_ring_queue_offsets(user_memory_socket_file_descriptor, memory_map_offsets.completion_ring_offsets(), completion_ring_queue_depth, XDP_UMEM_PGOFF_COMPLETION_RING)
	}
	
	#[inline(always)]
	pub(crate) fn get_completed_frame_descriptor_bitfield(&self, completion_queue_index: RingQueueIndex, relative_frame_index: RelativeFrameIndex) -> FrameDescriptorBitfield
	{
		*self.completion_adddress(completion_queue_index + relative_frame_index)
	}
	
	/// Based on `xsk_ring_cons__comp_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// What we get back is the original value of xdp_desc.addr.
	#[inline(always)]
	fn completion_adddress(&self, index: RingQueueEntryIndex) -> &FrameDescriptorBitfield
	{
		self.ring_entry(index)
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Used in conjunction with a `CompletionQueue`.
//
// Starts off empty.
//
// Is filled with 'fat pointers' (`xdp_desc`) by user space.
// The 'fat pointers' contain the size and relative address of an (Ethernet) frame in `UserMemory`.
#[doc(hidden)]
pub type TransmitQueue = XskRingQueue<ProducerXskRingQueueKind, FrameDescriptor>;

impl TransmitQueue
{
	#[inline(always)]
	pub(super) fn from_transmit_memory_map_offsets(express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, transmit_ring_queue_depth: RingQueueDepth) -> Self
	{
		Self::from_ring_queue_offsets(express_data_path_socket_file_descriptor, memory_map_offsets.transmit_ring_offsets(), transmit_ring_queue_depth, XDP_PGOFF_TX_RING)
	}
	
	#[inline(always)]
	pub(super) fn set_transmit_descriptor_from_frame(&self, transmit_queue_index: RingQueueIndex, relative_frame_index: RelativeFrameIndex, transmit_frame_descriptor_bitfield: FrameDescriptorBitfield, length_of_packet: usize)
	{
		let transmit_descriptor = self.transmit_descriptor(transmit_queue_index + relative_frame_index);
		
		FrameDescriptor::write(transmit_descriptor, transmit_frame_descriptor_bitfield, length_of_packet)
	}
	
	/// Based on `xsk_ring_prod__tx_desc()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returned pointer should be treated as uninitialized memory.
	#[inline(always)]
	fn transmit_descriptor(&self, index: RingQueueEntryIndex) -> NonNull<FrameDescriptor>
	{
		self.ring_entry_mut(index)
	}
}

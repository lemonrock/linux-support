// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used in conjunction with a `CompletionQueue`.
///
/// Starts off empty.
///
/// Is filled with 'fat pointers' (`xdp_desc`) by user space.
/// The 'fat pointers' contain the size and relative address of an (Ethernet) frame in `UserMemory`.
pub(crate) type TransmitQueue = XskRingQueue<ProducerXskRingQueueKind, FrameDescriptor>;

impl TransmitQueue
{
	#[inline(always)]
	pub(super) fn from_transmit_memory_map_offsets(express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, transmit_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets(express_data_path_socket_file_descriptor, memory_map_offsets.transmit_ring_offsets(), transmit_ring_queue_depth, defaults, XDP_PGOFF_TX_RING)
	}
	
	/// Only correct to use this method if the received frame is being forwarded without change in size and the memory pointed to does not overlap (it shouldn't).
	#[inline(always)]
	pub(super) fn set_transmit_descriptor_from_receive_descriptor<CA: ChunkAlignment>(&self, transmit_queue_index: u32, relative_frame_index: u32, receive_descriptor: &FrameDescriptor)
	{
		let index = transmit_queue_index + relative_frame_index;
		let transmit_descriptor = self.transmit_descriptor(index);
		
		FrameDescriptor::write_for_transmit_copying_from(transmit_descriptor, receive_descriptor)
	}
	
	#[inline(always)]
	pub(super) fn set_transmit_descriptor_from_aligned_frame_reference(&self, transmit_queue_index: u32, relative_frame_index: u32, aligned_frame_reference: &AlignedFrameReference, aligned_chunk_size: AlignedChunkSize, frame_headroom: FrameHeadroom)
	{
		let index = transmit_queue_index + relative_frame_index;
		let transmit_descriptor = self.transmit_descriptor(index);
		
		aligned_frame_reference.write_for_transmit_aligned(transmit_descriptor, aligned_chunk_size, frame_headroom)
	}
	
	/// Based on `xsk_ring_prod__tx_desc()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returned pointer should be treated as uninitialized memory.
	#[inline(always)]
	pub(super) fn transmit_descriptor(&self, index: u32) -> NonNull<FrameDescriptor>
	{
		self.ring_entry_mut(index)
	}
}

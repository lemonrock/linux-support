// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used in conjunction with a `FillQueue`.
///
/// Starts off empty.
///
/// Is filled with 'fat pointers' (`xdp_desc`) by the Linux kernel.
/// The 'fat pointers' contain the size and relative address of an (Ethernet) frame in `UserMemory`.
pub(crate) type ReceiveQueue = XskRingQueue<ConsumerXskRingQueueKind, FrameDescriptor>;

impl ReceiveQueue
{
	#[inline(always)]
	pub(super) fn from_receive_memory_map_offsets(express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, receive_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets(express_data_path_socket_file_descriptor, memory_map_offsets.receive_ring_offsets(), receive_ring_queue_depth, defaults, XDP_PGOFF_RX_RING)
	}
	
	/// Based on `xsk_ring_cons__rx_desc()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn get_receive_descriptor(&self, receive_queue_index: RingQueueIndex, relative_frame_index: u32) -> &FrameDescriptor
	{
		let index = receive_queue_index.add(relative_frame_index);
		self.receive_descriptor(index)
	}
	
	/// Based on `xsk_ring_cons__rx_desc()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	fn receive_descriptor(&self, index: RingQueueEntryIndex) -> &xdp_desc
	{
		self.ring_entry(index)
	}
}

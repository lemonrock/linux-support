// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used in conjunction with a `CompletionQueue`.
///
/// Starts off empty.
///
/// Is filled with 'fat pointers' (`xdp_desc`) by user space.
/// The 'fat pointers' contain the size and relative address of an (Ethernet) frame in `UserMemory`.
pub(super) type TransmitQueue = XskRingQueue<ProducerXskRingQueueKind, xdp_desc>;

impl TransmitQueue
{
	#[inline(always)]
	pub(super) fn from_transmit_memory_map_offsets(xsk_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, transmit_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets(xsk_socket_file_descriptor, memory_map_offsets.transmit_ring_offsets(), transmit_ring_queue_depth, defaults, XDP_PGOFF_TX_RING)
	}
	
	/// Based on `xsk_ring_prod__tx_desc()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returned pointer should be treated as uninitialized memory.
	#[inline(always)]
	pub(super) fn transmit_descriptor(&self, index: u32) -> NonNull<xdp_desc>
	{
		self.ring_entry_mut(index)
	}
}

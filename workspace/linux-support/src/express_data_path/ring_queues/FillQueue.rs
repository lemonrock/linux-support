// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used in conjunction with a `ReceiveQueue`.
///
/// Contains a queue of user memory frame indices 'gifted' to the Linux kernel.
///
/// Starts off full.
pub(super) type FillQueue = XskRingQueue<ProducerXskRingQueueKind, UmemDescriptor>;

impl FillQueue
{
	#[inline(always)]
	pub(super) fn from_fill_memory_map_offsets<ROTOB: ReceiveOrTransmitOrBoth<_, _>>(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, fill_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes, chunk_size: ChunkSize) -> Self
	{
		let this = Self::from_ring_queue_offsets(user_memory_socket_file_descriptor, memory_map_offsets.fill_ring_offsets(), fill_ring_queue_depth, defaults, XDP_UMEM_PGOFF_FILL_RING);
		
		// Linux documentation (`Documentation/networking/af_xdp.rst`, currently section `XDP_{RX|TX|UMEM_FILL|UMEM_COMPLETION}_RING setsockopts`) recommends not populating the fill queue if only doing transmit.
		if ROTOB::IsReceiveOrBoth
		{
			this.populate(fill_ring_queue_depth, chunk_size)
		}
		
		this
	}
	
	#[inline(always)]
	pub(super) fn set_fill_address(&self, fill_queue_index: u32, relative_frame_index: u32, relative_address_of_frame_in_user_memory: UmemDescriptor)
	{
		let index = fill_queue_index + relative_frame_index;
		unsafe { *self.fill_adddress(index).as_ptr() = relative_address_of_frame_in_user_memory }
	}
	
	/// Based on `xsk_ring_prod__fill_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returned pointer should be treated as uninitialized memory.
	#[inline(always)]
	fn fill_adddress(&self, index: u32) -> NonNull<UmemDescriptor>
	{
		self.ring_entry_mut(index)
	}
	
	/// Based on `xsk_populate_fill_ring()` in Linux source `samples/bpf/xdpsock_user.c`.
	#[inline(always)]
	fn populate(&self, fill_ring_queue_depth: RingQueueDepth, chunk_size: ChunkSize)
	{
		let number_of_frames = fill_ring_queue_depth as u32;
		let chunk_size = chunk_size as u32 as u64;
		
		let fill_queue_index = match self.reserve(number_of_frames)
		{
			None => unreachable!("FillQueue has not yet been used"),
			
			Some(fill_queue_index) => fill_queue_index
		};
		
		for relative_frame_index in 0 .. number_of_frames
		{
			self.set_fill_address(fill_queue_index, relative_frame_index, (relative_frame_index as u64) * chunk_size)
		}
		
		self.submit(number_of_frames)
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Used in conjunction with a `ReceiveQueue`.
//
// Contains a queue of user memory frame indices 'gifted' to the Linux kernel.
//
// Starts off full.
#[doc(hidden)]
pub type FillQueue = XskRingQueue<ProducerXskRingQueueKind, FrameDescriptorBitfield>;

impl FillQueue
{
	#[inline(always)]
	pub(crate) fn from_fill_memory_map_offsets<FOCOBRQD: FillOrCompletionOrBothRingQueueDepths>(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, fill_ring_queue_depth: RingQueueDepth, default_page_size: PageSize) -> Self
	{
		Self::from_ring_queue_offsets(user_memory_socket_file_descriptor, memory_map_offsets.fill_ring_offsets(), fill_ring_queue_depth, default_page_size, XDP_UMEM_PGOFF_FILL_RING)
	}
	
	/// Originally based on `xsk_populate_fill_ring()` in Linux source `samples/bpf/xdpsock_user.c`.
	#[inline(always)]
	pub(crate) fn gift_initial_frames_to_kernel_for_receive<FFQ: FreeFrameQueue>(&self, fill_ring_queue_depth: RingQueueDepth, chunk_size: FFQ::CS, free_frame_queue: &FFQ, frame_headroom: FrameHeadroom)
	{
		let number_of_frames_initially_gifted_to_the_linux_kernel: NonZeroU32 = fill_ring_queue_depth.into();
		
		let fill_queue_index = self.reserve(number_of_frames_initially_gifted_to_the_linux_kernel).unwrap();
		{
			for relative_frame_index in RelativeFrameIndex::relative_frame_indices(number_of_frames_initially_gifted_to_the_linux_kernel)
			{
				let frame_identifier = free_frame_queue.pop().unwrap();
				let fill_frame_descriptor_bitfield = chunk_size.fill_frame_descriptor_bitfield(frame_headroom, frame_identifier);
				self.set_fill_address(fill_queue_index, relative_frame_index, fill_frame_descriptor_bitfield)
			}
		}
		self.submit(number_of_frames_initially_gifted_to_the_linux_kernel)
	}
	
	#[inline(always)]
	pub(crate) fn set_fill_address_receive(&self, fill_queue_index: RingQueueIndex, fill_relative_frame_index: &mut RelativeFrameIndex, fill_frame_descriptor_bitfield: FrameDescriptorBitfield, filled_number_of_frames: &mut u32)
	{
		self.set_fill_address(fill_queue_index, *fill_relative_frame_index, fill_frame_descriptor_bitfield);
		fill_relative_frame_index.next();
		*filled_number_of_frames += 1;
	}
	
	/// At this point in time, only the `orig_addr` part of the `fill_address_frame_descriptor_bitfield` is used by the Linux kernel.
	#[inline(always)]
	pub(crate) fn set_fill_address(&self, fill_queue_index: RingQueueIndex, relative_frame_index: RelativeFrameIndex, fill_frame_descriptor_bitfield: FrameDescriptorBitfield)
	{
		unsafe { * self.fill_address(fill_queue_index + relative_frame_index).as_ptr() = fill_frame_descriptor_bitfield }
	}
	
	/// Based on `xsk_ring_prod__fill_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returned pointer should be treated as uninitialized memory.
	#[inline(always)]
	fn fill_address(&self, index: RingQueueEntryIndex) -> NonNull<FrameDescriptorBitfield>
	{
		self.ring_entry_mut(index)
	}
}

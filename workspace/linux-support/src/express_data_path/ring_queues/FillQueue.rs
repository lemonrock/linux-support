// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used in conjunction with a `ReceiveQueue`.
///
/// Contains a queue of user memory frame indices 'gifted' to the Linux kernel.
///
/// Starts off full.
pub(crate) type FillQueue = XskRingQueue<ProducerXskRingQueueKind, UserMemoryDescriptor>;

impl FillQueue
{
	#[inline(always)]
	pub(crate) fn from_fill_memory_map_offsets<FOCOBRQD: FillOrCompletionOrBothRingQueueDepths>(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, fill_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes, chunk_size: AlignedChunkSize) -> (Self, Option<NonZeroU32>)
	{
		let this = Self::from_ring_queue_offsets(user_memory_socket_file_descriptor, memory_map_offsets.fill_ring_offsets(), fill_ring_queue_depth, defaults, XDP_UMEM_PGOFF_FILL_RING);
		
		// Linux documentation (`Documentation/networking/af_xdp.rst`, currently section `XDP_{RX|TX|UMEM_FILL|UMEM_COMPLETION}_RING setsockopts`) recommends not populating the fill queue if only doing transmit.
		let number_of_frames_initially_gifted_to_the_linux_kernel = if FOCOBRQD::SupportsReceive
		{
			xxxx;
			// TODO: populate for non-aligned chunk sizes.
			
			Some(this.gift_initial_frames_to_kernel_for_receive(fill_ring_queue_depth, chunk_size))
		}
		else
		{
			None
		};
		
		(this, number_of_frames_initially_gifted_to_the_linux_kernel)
	}
	
	#[inline(always)]
	pub(crate) fn set_fill_user_memory_descriptor_of_frame_in_user_memory(&self, fill_queue_index: u32, relative_frame_index: u32, fill_address_user_memory_descriptor: UserMemoryDescriptor)
	{
		let index = fill_queue_index + relative_frame_index;
		unsafe { *self.fill_user_memory_descriptor(index).as_ptr() = fill_address_user_memory_descriptor }
	}
	
	/// Based on `xsk_ring_prod__fill_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returned pointer should be treated as uninitialized memory.
	#[inline(always)]
	fn fill_user_memory_descriptor(&self, index: u32) -> NonNull<UserMemoryDescriptor>
	{
		self.ring_entry_mut(index)
	}
	
	/// Based on `xsk_populate_fill_ring()` in Linux source `samples/bpf/xdpsock_user.c`.
	#[inline(always)]
	fn gift_initial_frames_to_kernel_for_receive(&self, fill_ring_queue_depth: RingQueueDepth, aligned_chunk_size: AlignedChunkSize, is_aligned: bool) -> NonZeroU32
	{
		let number_of_frames_initially_gifted_to_the_linux_kernel: NonZeroU32 = fill_ring_queue_depth.into();
		
		let fill_queue_index = self.reserve(number_of_frames_initially_gifted_to_the_linux_kernel).unwrap();
		{
			for absolute_frame_index in 0 .. number_of_frames_initially_gifted_to_the_linux_kernel.get()
			{
				let fill_address_user_memory_descriptor = if is_aligned
				{
					let frame_number = AlignedFrameNumber::from(absolute_frame_index);
					frame_number.to_user_memory_descriptor(aligned_chunk_size)
				}
				else
				{
					 XXXXXX;
				};
				
				let relative_frame_index = absolute_frame_index;
				self.set_fill_user_memory_descriptor_of_frame_in_user_memory(fill_queue_index, relative_frame_index, fill_address_user_memory_descriptor)
			}
		}
		self.submit(number_of_frames_initially_gifted_to_the_linux_kernel);
		
		number_of_frames_initially_gifted_to_the_linux_kernel
	}
	
	#[inline(always)]
	pub(crate) fn first_frame_not_initially_gifted_to_the_linux_kernel(number_of_frames_initially_gifted_to_the_linux_kernel: NonZeroU32) -> AlignedFrameNumber
	{
		AlignedFrameNumber(number_of_frames_initially_gifted_to_the_linux_kernel.get())
	}
}

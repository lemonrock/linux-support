// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug)]
pub struct UserMemory<FFQ: FreeFrameQueue>
{
	fill_queue: FillQueue,
	
	/// `xsk_ring_cons`.
	///
	/// Forms a pairing with `TransmitQueue`.
	completion_queue: CompletionQueue,
	
	user_memory_area: ManuallyDrop<UserMemoryArea>,
	
	user_memory_socket_file_descriptor: ManuallyDrop<ExpressDataPathSocketFileDescriptor>,
	
	chunk_size: FFQ::CS,
	
	number_of_chunks: NonZeroU32,
	
	frame_headroom: FrameHeadroom,
	
	free_frame_queue: FFQ,
}

impl<FFQ: FreeFrameQueue> Drop for UserMemory<FFQ>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			ManuallyDrop::drop(&mut self.user_memory_socket_file_descriptor);
			ManuallyDrop::drop(&mut self.user_memory_area);
		};
	}
}

impl<FFQ: FreeFrameQueue> UserMemory<FFQ>
{
	#[inline(always)]
	pub(crate) fn number_of_frames(&self, frames: &[FrameReference<FFQ::CS>]) -> NonZeroU32
	{
		let number_of_frames = frames.len();
		debug_assert_ne!(number_of_frames, 0);
		debug_assert!(number_of_frames <= (u32::MAX as usize));
		
		let number_of_frames = number_of_frames as u32;
		debug_assert!(number_of_frames <= self.number_of_chunks.get());
		
		unsafe { NonZeroU32::new_unchecked(number_of_frames) }
	}
	
	#[inline(always)]
	pub(crate) fn user_memory_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.user_memory_socket_file_descriptor
	}
	
	#[inline(always)]
	pub(crate) fn received_xdp_headroom_our_frame_headroom_ethernet_packet_minimum_tailroom_length(&self, received_descriptor: &FrameDescriptor) -> (FrameDescriptorBitfield, &[u8], &mut [u8], &mut [u8], usize)
	{
		let received_relative_addresses_and_offsets = FFQ::CS::received_relative_addresses_and_offsets(received_descriptor, self.frame_headroom);
		
		let user_memory_area = self.user_memory_area.deref();
		(
			FFQ::CS::fill_frame_descriptor_bitfield_if_constructed_from_received_frame_descriptor(&received_relative_addresses_and_offsets),
			received_relative_addresses_and_offsets.xdp_headroom(user_memory_area),
			received_relative_addresses_and_offsets.our_frame_headroom_mut(user_memory_area),
			received_relative_addresses_and_offsets.ethernet_packet_mut(user_memory_area),
			received_relative_addresses_and_offsets.minimum_tailroom_length(self.chunk_size),
		)
	}
	
	#[inline(always)]
	pub(crate) fn frame_to_transmit_our_frame_headroom_ethernet_packet(&self, frame_identifier: <<FFQ as FreeFrameQueue>::CS as ChunkSize>::FrameIdentifier) -> (&mut [u8], &mut [u8])
	{
		let mut relative_addresses_and_offsets = self.chunk_size.transmit_relative_addesses_and_offsets(self.frame_headroom, frame_identifier, 0);
		relative_addresses_and_offsets.length_of_packet = relative_addresses_and_offsets.minimum_tailroom_length(self.chunk_size);
		
		let user_memory_area = self.user_memory_area.deref();
		(
			relative_addresses_and_offsets.our_frame_headroom_mut(user_memory_area),
			relative_addresses_and_offsets.ethernet_packet_mut(user_memory_area),
		)
	}
	
	#[inline(always)]
	pub(crate) fn push_free_frame_from_receive(&self, received_frame_descriptor_bitfield: FrameDescriptorBitfield)
	{
		let newly_freed_frame_identifier = self.chunk_size.received_frame_identifier(received_frame_descriptor_bitfield);
		self.free_frame_queue.push(newly_freed_frame_identifier)
	}
	
	#[inline(always)]
	pub(crate) fn push_free_frame_from_completion(&self, completed_frame_descriptor_bitfield: FrameDescriptorBitfield)
	{
		let newly_freed_frame_identifier = self.chunk_size.completed_frame_identifier(completed_frame_descriptor_bitfield);
		self.free_frame_queue.push(newly_freed_frame_identifier);
	}
	
	#[inline(always)]
	pub(crate) fn pop_free_frame(&self) -> Option<<<FFQ as FreeFrameQueue>::CS as ChunkSize>::FrameIdentifier>
	{
		self.free_frame_queue.pop()
	}
	
	#[inline(always)]
	pub(crate) fn frame_identifier_to_fill_frame_descriptor_bitfield(&self, frame_identifier: <<FFQ as FreeFrameQueue>::CS as ChunkSize>::FrameIdentifier) -> FrameDescriptorBitfield
	{
		self.chunk_size.fill_frame_descriptor_bitfield(self.frame_headroom, frame_identifier)
	}
	
	#[inline(always)]
	pub(crate) fn frame_identifier_to_transmit_frame_descriptor_bitfield(&self, frame_identifier: <<FFQ as FreeFrameQueue>::CS as ChunkSize>::FrameIdentifier) -> FrameDescriptorBitfield
	{
		self.chunk_size.transmit_frame_descriptor_bitfield(self.frame_headroom, frame_identifier)
	}
	
	/// Based on `libbpf`'s `xsk_umem__create_v0_0_4()` (also known as `xsk_umem__create()`) in Linux source `tools/lib/bpf/xsk.c` and also `main()` and `xsk_configure_umem()` in Linux source `samples/bpf/xdp_sockuser.c`.
	///
	/// If flags contains `XdpUmemRegFlags::UnalignedChunks`, then `huge_memory_page_size` can not be `None`.
	/// `number_of_chunks` might be 4096.
	///
	/// If using an AlignedChunkSize, `number_of_chunks` must be an exact multiple of the number that would fit in a page.
	/// Thus we round it up if necessary.
	fn new<FOCOBRQD: FillOrCompletionOrBothRingQueueDepths>(number_of_chunks: NonZeroU32, chunk_size: FFQ::CS, frame_headroom: FrameHeadroom, maximum_transmission_unit_payload_size: MaximumTransmissionUnitPayloadSize, fill_or_completion_or_both_ring_queue_depths: FOCOBRQD, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		FFQ::CS::validate_user_memory(huge_memory_page_size);
		
		let fill_ring_queue_depth = fill_or_completion_or_both_ring_queue_depths.fill_ring_queue_depth_or_default();
		let completion_ring_queue_depth = fill_or_completion_or_both_ring_queue_depths.completion_ring_queue_depth_or_default();
		
		// Internally, Linux drops received packets that don't fit into a chunk.
		// See the usage of `xsk_umem_get_rx_frame_size()` in Linux source.
		if chunk_size.compare_to_frame_sizes(frame_headroom, maximum_transmission_unit_payload_size) == Ordering::Less
		{
			let chunk_size: u64 = chunk_size.into();
			return Err(ChunkSizeDoesNotAccommodateFrameHeadroomAndMaximumTransmissionUnitIncludingFrameCheckSequenceSoLinuxWouldDropPackets { xdp_packet_headroom: XDP_PACKET_HEADROOM, frame_headroom, chunk_size, maximum_transmission_unit_payload_size } )
		}
		
		let number_of_chunks = chunk_size.round_up_number_of_chunks_to_a_multiple_that_fits_exactly_into_multiple_pages(number_of_chunks);
		let user_memory_area = UserMemoryArea::new(number_of_chunks, chunk_size, huge_memory_page_size, defaults)?;
		let free_frame_queue = FFQ::new(number_of_chunks, user_memory_area.deref());
		
		let user_memory_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new().map_err(CouldNotCreateUserMemorySocketFileDescriptor)?;
		let configuration = xdp_umem_reg::new(&user_memory_area, chunk_size, frame_headroom, FFQ::CS::RegistrationFlags);
		user_memory_socket_file_descriptor.register_user_space_memory(&configuration, fill_ring_queue_depth, completion_ring_queue_depth);
		let memory_map_offsets = user_memory_socket_file_descriptor.get_memory_map_offsets();
		
		let fill_queue = XskRingQueue::from_fill_memory_map_offsets::<FOCOBRQD>(&user_memory_socket_file_descriptor, &memory_map_offsets, fill_ring_queue_depth, defaults);
		
		// Linux documentation (`Documentation/networking/af_xdp.rst`, currently section `XDP_{RX|TX|UMEM_FILL|UMEM_COMPLETION}_RING setsockopts`) recommends not populating the fill queue if only doing transmit.
		if FOCOBRQD::SupportsReceive
		{
			fill_queue.gift_initial_frames_to_kernel_for_receive(fill_ring_queue_depth, chunk_size, &free_frame_queue, frame_headroom)
		}
		
		Ok
		(
			Self
			{
				fill_queue,
				completion_queue: XskRingQueue::from_completion_memory_map_offsets(&user_memory_socket_file_descriptor, &memory_map_offsets, completion_ring_queue_depth, defaults),
				user_memory_area: ManuallyDrop::new(user_memory_area),
				user_memory_socket_file_descriptor: ManuallyDrop::new(user_memory_socket_file_descriptor),
				chunk_size,
				number_of_chunks,
				frame_headroom,
				free_frame_queue,
			}
		)
	}
}

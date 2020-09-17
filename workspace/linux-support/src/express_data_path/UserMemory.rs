// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug)]
pub struct UserMemory<CA: ChunkAlignment>
{
	fill_queue: FillQueue,
	
	/// `xsk_ring_cons`.
	///
	/// Forms a pairing with `TransmitQueue`.
	completion_queue: CompletionQueue,
	
	user_memory_area: ManuallyDrop<UserMemoryArea>,
	
	user_memory_socket_file_descriptor: ManuallyDrop<ExpressDataPathSocketFileDescriptor>,
	
	chunk_size: ChunkSize,
	
	chunk_alignment: CA,
	
	number_of_frames: NonZeroU32,
	
	frame_headroom: FrameHeadroom,
}

impl<CA: ChunkAlignment> Drop for UserMemory<CA>
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

impl<CA: ChunkAlignment> UserMemory<CA>
{
	#[inline(always)]
	pub(crate) fn number_of_frames(&self, frames: &[FrameReference]) -> u32
	{
		let number_of_frames = frames.len();
		debug_assert!(number_of_frames <= (u32::MAX as usize));
		
		let number_of_frames = number_of_frames as u32;
		debug_assert!(number_of_frames <= self.number_of_frames.get());
		
		number_of_frames
	}
	
	#[inline(always)]
	pub(crate) fn user_memory_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.user_memory_socket_file_descriptor
	}
	
	#[inline(always)]
	pub(crate) fn chunk_size(&self) -> ChunkSize
	{
		self.chunk_size
	}
	
	/// The network packet should start with an `ether_header` struct, eg see the function `swap_mac_addresses()` in Linux source `samples/bpf/xdpsock_user.c`.
	///
	/// ***WARNING***: The frame is only valid until the underlying `descriptor` is released; the lifetime `'a` is overly long.
	///
	/// Frames in user memory do not include a trailing ethernet frame check sequeunces (FCS).
	#[inline(always)]
	pub(crate) fn frame_from_descriptor<'a>(&'a self, descriptor: &'a xdp_desc) -> (&'a mut [u8], &'a mut [u8])
	{
		self.user_memory_area.frame(self.frame_headroom, descriptor.user_memory_area_relative_address::<CA>(self.chunk_size), descriptor.length())
	}
	
	/// The (Ethernet) frame (packet) should start with an `ether_header` struct, eg see the function `swap_mac_addresses()` in Linux source `samples/bpf/xdpsock_user.c`.
	#[inline(always)]
	pub(crate) fn frame_from_frame_number(&self, frame_number: FrameNumber, chunk_size: ChunkSize) -> (&mut [u8], &mut [u8])
	{
		self.user_memory_area.frame(self.frame_headroom, frame_number.to_user_memory_area_relative_address(chunk_size), chunk_size as u32 as usize)
	}
	
	/// The network packet should start with an `ether_header` struct, eg see the function `swap_mac_addresses()` in Linux source `samples/bpf/xdpsock_user.c`.
	#[inline(always)]
	pub(crate) fn frame_from_frame_reference(&self, frame_reference: &FrameReference, chunk_size: ChunkSize) -> (&mut [u8], &mut [u8])
	{
		self.user_memory_area.frame(self.frame_headroom, frame_reference.to_user_memory_area_relative_address(chunk_size), frame_reference.frame_length())
	}
	
	/// Based on `libbpf`'s `xsk_umem__create_v0_0_4()` (also known as `xsk_umem__create()`) in Linux source `tools/lib/bpf/xsk.c` and also `main()` and `xsk_configure_umem()` in Linux source `samples/bpf/xdp_sockuser.c`.
	///
	/// If flags contains `XdpUmemRegFlags::UnalignedChunks`, then `huge_memory_page_size` can not be `None`.
	/// `number_of_frames` might be 4096.
	fn new(number_of_frames: NonZeroU32, chunk_size: ChunkSize, frame_headroom: FrameHeadroom, network_interface_maximum_transmission_unit_including_frame_check_sequence: MaximumTransmissionUnit, chunk_alignment: CA, fill_or_completion_or_both_ring_queue_depths: impl FillOrCompletionOrBothRingQueueDepths, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let fill_ring_queue_depth = fill_or_completion_or_both_ring_queue_depths.fill_queue_ring_queue_depth_or_default();
		let completion_ring_queue_depth = fill_or_completion_or_both_ring_queue_depths.completion_queue_depth_or_default();
		
		let flags = if CA::IsUnaligned
		{
			debug_assert!(huge_memory_page_size.is_some(), "When using XdpUmemRegFlagsUnalignedChunks in `flags`, Huge Pages must be used");
			XdpUmemFlags::UnalignedChunks
		}
		else
		{
			XdpUmemFlags::empty()
		};
		
		assert!((XDP_PACKET_HEADROOM + frame_headroom.0) < (chunk_size as u32), "Mirrors check in `xdp_umem_reg() in Linux source`");
		
		// Internally, Linux drops received packets that don't fit into a chunk.
		// See the usage of `xsk_umem_get_rx_frame_size()` in Linux source.
		if (XDP_PACKET_HEADROOM + frame_headroom.0 + network_interface_maximum_transmission_unit_including_frame_check_sequence.0) > (chunk_size as u32)
		{
			return Err(ChunkSizeDoesNotAccommodateFrameHeadroomAndMaximumTransmissionUnitIncludingFrameCheckSequence { xdp_packet_headroom: XDP_PACKET_HEADROOM, frame_headroom, network_interface_maximum_transmission_unit_including_frame_check_sequence} )
		}
		
		
		assert!((XDP_PACKET_HEADROOM + frame_headroom.0 + network_interface_maximum_transmission_unit_including_frame_check_sequence.0) <= (chunk_size as u32), "XDP_PACKET_HEADROOM `{:?}` + frame_headroom `{:?}` + network_interface_maximum_transmission_unit `{:?}` will not fit in chunk_size `{}`", XDP_PACKET_HEADROOM, frame_headroom, network_interface_maximum_transmission_unit_including_frame_check_sequence, chunk_size);
		
		let user_memory_area = UserMemoryArea::new(number_of_frames, chunk_size, huge_memory_page_size, defaults)?;
		
		let user_memory_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new().map_err(CouldNotCreateUserMemorySocketFileDescriptor)?;
		let configuration = xdp_umem_reg::new(&user_memory_area, chunk_size, frame_headroom, flags);
		user_memory_socket_file_descriptor.register_user_space_memory(&configuration, fill_ring_queue_depth, completion_ring_queue_depth);
		let memory_map_offsets = user_memory_socket_file_descriptor.get_memory_map_offsets();
		
		Ok
		(
			Self
			{
				fill_queue: XskRingQueue::from_fill_memory_map_offsets::<FOCOBRQD>(&user_memory_socket_file_descriptor, &memory_map_offsets, fill_ring_queue_depth, defaults, chunk_size),
				completion_queue: XskRingQueue::from_completion_memory_map_offsets(&user_memory_socket_file_descriptor, &memory_map_offsets, completion_ring_queue_depth, defaults),
				user_memory_area: ManuallyDrop::new(user_memory_area),
				user_memory_socket_file_descriptor: ManuallyDrop::new(user_memory_socket_file_descriptor),
				chunk_size,
				chunk_alignment,
				number_of_frames,
				frame_headroom,
			}
		)
	}
}

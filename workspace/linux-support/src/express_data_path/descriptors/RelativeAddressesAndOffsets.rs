// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A chunk (frame) is laid out as follows:-
///
/// ```bash
/// |xdp_headroom|our_frame_headroom|ethernet packet|
///
/// xdp_headroom can be further subdivided:-
///
/// |xdp_frame|available for use|
///
/// xdp_headroom is fixed in size as XDP_PACKET_HEADROOM, 256 bytes.
/// ```
///
/// The following is already known:-
///
/// ```bash
/// pool.addrs is the start of user memory.
/// pool.addrs_cnt is actually the length (size) of user memory.
/// pool.headroom is the FrameHeadroom we specify at creation.
/// pool.chunk_mask = AlignedChunkSize.mask().
/// xdp_buff_xsk.xdp.data is an absolute address.
/// xdp_buff_xsk.xdp.data_hard_start is an absolute address.
/// xdp_buff_xsk.xdp.data_meta is an absolute address.
/// ```
///
/// The following for receive is worked out from:-
/// 
/// *  `__xp_alloc()` in Linux source `net/xdp/xsk_buff_pool.c`;
/// *  `xp_alloc()` in Linux source `net/xdp/xsk_buff_pool.c`.
/// * `__xsk_rcv()` in Linux source `net/xdp/xsk.c`.
/// * `__xsk_rcv_zc()` in Linux source `net/xdp/xsk.c`.
/// * `xdp_update_frame_from_buff()` in Linux source `includes/net/xdp.h`.
/// * `xp_aligned_extract_addr()` in Linux source `includes/net/xsk_buff_pool.h`.
/// * `xp_check_aligned()` in Linux source `includes/net/xdp_sock_drv.h`.
/// * `xp_check_unaligned()` in Linux source `includes/net/xdp_sock_drv.h`.
/// * `xp_get_handle()` in Linux source `net/xdp/xsk.c`.
/// * `xp_unaligned_extract_addr()` in Linux source `includes/net/xsk_buff_pool.h`.
/// * `xp_unaligned_extract_offset()` in Linux source `includes/net/xsk_buff_pool.h`.
/// * `xsk_buff_alloc()` in Linux source `includes/net/xdp_sock_drv.h` (which calls `xp_alloc()`);
/// * `xsk_umem__extract_addr()` in Linux source `tools/lib/bpf/xsk.h`.
/// * `xsk_umem__extract_offset()` in Linux source `tools/lib/bpf/xsk.h`.
/// * `xskq_prod_reserve_desc()` in Linux source `net/xdp/xsk_queue.h`.
///
/// ```bash
/// filled_address = new_address_from_fill_queue();
///
/// let user_memory_descriptor = if aligned
/// {
/// 	// xp_aligned_extract_addr(filled_address), viz
/// 	// filled_address & pool.chunk_mask  viz
/// 	filled_address & !(chunk_size - 1)
///
/// 	// ie make user_memory_descriptor start from start of a chunk.
/// }
/// else
/// {
/// 	// user_memory_descriptor + chunk_size must not cross a non-contiguous page; the only way to ensure this is to mmap from a contiguous region of physical memory.
/// 	// xp_unaligned_extract_addr(filled_address), viz
/// 	filled_address & XSK_UNALIGNED_BUF_ADDR_MASK
///
/// 	// ie extract arbitrary relative user_memory_descriptor.
/// };
/// xdp_buff_xsk.orig_addr = user_memory_descriptor
/// xdp_buff_xsk.xdp.data_hard_start = pool.addrs + user_memory_descriptor + pool.headroom
/// xdp_buff_xsk.xdp.data = xdp_buff_xsk.xdp.data_hard_start + XDP_PACKET_HEADROOM
/// xdp_buff_xsk.xdp.data_meta = xdp_buff_xsk.xdp.data
///
/// xdp_headroom_length = xdp_buff_xsk.xdp.data - xdp_buff_xsk.xdp.data_hard_start;
/// xdp_headroom_available_to_program = xdp_headroom_length - size_of::<xdp_frame>();
/// xdp_meta_data_length = xdp_buff_xsk.xdp.data - xdp_buff_xsk.xdp.data_meta;
/// xdp_meta_data_length can be zero.
/// 
/// offset = xdp_buff_xsk.xdp.data - xdp_buff_xsk.xdp.data_hard_start + pool.headroom
///
/// if aligned
/// {
/// 	receive_descriptor.addr = xdp_buff_xsk.orig_addr + offset
/// }
/// else
/// {
///		receive_descriptor.addr = xdp_buff_xsk.orig_addr + (offset << XSK_UNALIGNED_BUF_OFFSET_SHIFT)
/// }
/// ```
///
/// Thus
/// 
/// * If aligned:-
/// 	* `start_of_packet = desc.addr`.
/// 	* `length_of_packet = desc.len`.
/// 	* `length_of_our_frame_headroom = FrameHeadroom`.
/// 	* `start_of_our_frame_headroom = start_of_packet - length_of_our_frame_headroom`.
/// 	* `end_of_xdp_headroom = start_of_our_frame_headroom`.
/// 	* `xdp_headroom_length` which should be equal to `XDP_PACKET_HEADROOM`.
/// 	* `start_of_xdp_headroom = start_of_our_frame_headroom - xdp_headroom_length`.
/// 	* `start_of_xdp_headroom_after_xdp_frame = start_of_xdp_headroom + size_of::<xdp_frame>()`.
/// 	* `length_of_xdp_headroom_after_xdp_frame = xdp_headroom_length - size_of::<xdp_frame>()`.
/// 	* `orig_addr = end_of_xdp_headroom - xdp_headroom_length`.
/// 	* `offset = start_of_packet - orig_addr`.
///
/// * If unaligned:-
/// 	* `orig_addr = desc.addr & XSK_UNALIGNED_BUF_ADDR_MASK`.
/// 	* `offset = desc.addr >> XSK_UNALIGNED_BUF_OFFSET_SHIFT`.
/// 	* `start_of_packet = orig_addr + offset`.
/// 	* `length_of_packet = desc.len`.
/// 	* `length_of_our_frame_headroom = FrameHeadroom`.
/// 	* `start_of_our_frame_headroom = start_of_packet - length_of_our_frame_headroom`.
/// 	* `end_of_xdp_headroom = start_of_our_frame_headroom`.
/// 	* `xdp_headroom_length = offset - length_of_our_frame_headroom` which should be equal to `XDP_PACKET_HEADROOM`.
/// 	* `start_of_xdp_headroom = start_of_our_frame_headroom - xdp_headroom_length`.
/// 	* `start_of_xdp_headroom_after_xdp_frame = start_of_xdp_headroom + size_of::<xdp_frame>()`.
/// 	* `length_of_xdp_headroom_after_xdp_frame = xdp_headroom_length - size_of::<xdp_frame>()`.
pub struct RelativeAddressesAndOffsets
{
	pub start_of_packet: u64,
	
	/// Zero for completed frames, which is technically incorrect.
	///
	/// It is not possible to dehydrate this value.
	pub length_of_packet: usize,
	
	/// Sames as `orig_addr` for transmitted frames.
	pub start_of_our_frame_headroom: u64,
	
	pub length_of_our_frame_headroom: usize,
	
	/// Sames as `orig_addr` for transmitted frames and completed frames.
	pub end_of_xdp_headroom: u64,
	
	/// Zero for transmitted frames and completed frames.
	pub xdp_headroom_length: usize,
	
	/// Sames as `orig_addr` for transmitted frames and completed frames.
	pub start_of_xdp_headroom: u64,
	
	pub orig_addr: u64,
	
	pub offset: usize,
	
	// pub start_of_xdp_headroom_after_xdp_frame: u64,
	
	// pub length_of_xdp_headroom_after_xdp_frame: usize,
}

impl RelativeAddressesAndOffsets
{
	/// Length should be `XDP_FRAME_HEADROOM` but zero if created for a frame being transmitted by us.
	///
	/// The first part of this slice is occupied by a `xdp_frame`.
	#[inline(always)]
	pub(crate) fn xdp_headroom(&self, user_memory_area: &UserMemoryArea) -> &[u8]
	{
		user_memory_area.slice(self.start_of_xdp_headroom, self.xdp_headroom_length)
	}
	
	#[inline(always)]
	pub(crate) fn our_frame_headroom(&self, user_memory_area: &UserMemoryArea) -> &[u8]
	{
		user_memory_area.slice(self.start_of_our_frame_headroom, self.length_of_our_frame_headroom)
	}
	
	#[inline(always)]
	pub(crate) fn our_frame_headroom_mut(&self, user_memory_area: &UserMemoryArea) -> &mut [u8]
	{
		user_memory_area.slice_mut(self.start_of_our_frame_headroom, self.length_of_our_frame_headroom)
	}
	
	#[inline(always)]
	pub(crate) fn ethernet_packet(&self, user_memory_area: &UserMemoryArea) -> &[u8]
	{
		user_memory_area.slice(self.start_of_packet, self.length_of_packet)
	}
	
	#[inline(always)]
	pub(crate) fn ethernet_packet_mut(&self, user_memory_area: &UserMemoryArea) -> &mut [u8]
	{
		user_memory_area.slice_mut(self.start_of_packet, self.length_of_packet)
	}
	
	const TransmittedXdpHeadroomLength: usize = 0;
	
	/// `xdp_headroom_length` will be zero.
	/// `start_of_xdp_headroom` will be equal to `orig_addr`.
	/// `end_of_xdp_headroom` will be equal to `orig_addr`.
	/// `start_of_our_frame_headroom` will be equal to `orig_addr`.
	#[inline(always)]
	pub(crate) fn for_transmitted_frame_descriptor_if_aligned(orig_addr: u64, frame_headroom: FrameHeadroom, length_of_packet: usize) -> Self
	{
		let start_of_xdp_headroom = orig_addr;
		let xdp_headroom_length: usize = Self::TransmittedXdpHeadroomLength;
		let end_of_xdp_headroom = start_of_xdp_headroom + xdp_headroom_length;
		let start_of_our_frame_headroom = end_of_xdp_headroom;
		let length_of_our_frame_headroom = frame_headroom.into();
		let start_of_packet = start_of_our_frame_headroom + length_of_our_frame_headroom;
		let offset = xdp_headroom_length + length_of_our_frame_headroom;
		
		Self
		{
			start_of_packet,
			length_of_packet,
			start_of_our_frame_headroom,
			length_of_our_frame_headroom,
			end_of_xdp_headroom,
			xdp_headroom_length,
			start_of_xdp_headroom,
			orig_addr,
			offset,
			//start_of_xdp_headroom_after_xdp_frame: start_of_xdp_headroom,
			//length_of_xdp_headroom_after_xdp_frame: 0,
		}
	}
	
	#[inline(always)]
	pub(crate) const fn from_completed_frame_descriptor_if_aligned(frame_descriptor_bitfield: FrameDescriptorBitfield, frame_headroom: FrameHeadroom) -> Self
	{
		let start_of_packet = frame_descriptor_bitfield.start_of_packet_if_aligned();
		let length_of_packet = 0;
		let length_of_our_frame_headroom = frame_headroom.into();
		let start_of_our_frame_headroom = start_of_packet - length_of_our_frame_headroom;
		let end_of_xdp_headroom = start_of_our_frame_headroom;
		let xdp_headroom_length = Self::TransmittedXdpHeadroomLength;
		let start_of_xdp_headroom = end_of_xdp_headroom - xdp_headroom_length;
		let orig_addr = start_of_xdp_headroom;
		let offset = xdp_headroom_length + length_of_our_frame_headroom;
		
		Self
		{
			start_of_packet,
			length_of_packet,
			start_of_our_frame_headroom,
			length_of_our_frame_headroom,
			end_of_xdp_headroom,
			xdp_headroom_length,
			start_of_xdp_headroom,
			orig_addr,
			offset,
			//start_of_xdp_headroom_after_xdp_frame: start_of_xdp_headroom,
			//length_of_xdp_headroom_after_xdp_frame: 0,
		}
	}
	
	#[inline(always)]
	pub(crate) const fn from_completed_frame_descriptor_if_unaligned(frame_descriptor_bitfield: FrameDescriptorBitfield, frame_headroom: FrameHeadroom) -> Self
	{
		let start_of_packet = frame_descriptor_bitfield.start_of_packet_if_unaligned();
		let length_of_packet = 0;
		let length_of_our_frame_headroom = frame_headroom.into();
		let start_of_our_frame_headroom = start_of_packet - length_of_our_frame_headroom;
		let end_of_xdp_headroom = start_of_our_frame_headroom;
		let xdp_headroom_length = Self::TransmittedXdpHeadroomLength;
		let start_of_xdp_headroom = end_of_xdp_headroom - xdp_headroom_length;
		let orig_addr = start_of_xdp_headroom;
		debug_assert_eq!(orig_addr, frame_descriptor_bitfield.orig_addr_if_unaligned());
		let offset = xdp_headroom_length + length_of_our_frame_headroom;
		debug_assert_eq!(offset, frame_descriptor_bitfield.offset_if_unaligned());
		
		Self
		{
			start_of_packet,
			length_of_packet,
			start_of_our_frame_headroom,
			length_of_our_frame_headroom,
			end_of_xdp_headroom,
			xdp_headroom_length,
			start_of_xdp_headroom,
			orig_addr,
			offset,
			//start_of_xdp_headroom_after_xdp_frame: start_of_xdp_headroom,
			//length_of_xdp_headroom_after_xdp_frame: 0,
		}
	}
	
	#[inline(always)]
	pub(crate) const fn from_received_frame_descriptor_if_aligned(frame_descriptor_bitfield: FrameDescriptorBitfield, length_of_packet: u32, frame_headroom: FrameHeadroom) -> Self
	{
		let start_of_packet = frame_descriptor_bitfield.start_of_packet_if_aligned();
		let length_of_packet = length_of_packet as usize;
		let length_of_our_frame_headroom = frame_headroom.into();
		let start_of_our_frame_headroom = start_of_packet - length_of_our_frame_headroom;
		let end_of_xdp_headroom = start_of_our_frame_headroom;
		let xdp_headroom_length = XDP_PACKET_HEADROOM;
		let start_of_xdp_headroom = start_of_our_frame_headroom - xdp_headroom_length;
		let orig_addr = end_of_xdp_headroom - xdp_headroom_length;
		let offset = (start_of_packet - orig_addr) as usize;
		// let start_of_xdp_headroom_after_xdp_frame = start_of_xdp_headroom + (size_of::<xdp_frame>() as u64);
		// let length_of_xdp_headroom_after_xdp_frame = xdp_headroom_length - size_of::<xdp_frame>();
		
		Self
		{
			start_of_packet,
			length_of_packet,
			start_of_our_frame_headroom,
			length_of_our_frame_headroom,
			end_of_xdp_headroom,
			xdp_headroom_length,
			start_of_xdp_headroom,
			orig_addr,
			offset,
			//start_of_xdp_headroom_after_xdp_frame,
			//length_of_xdp_headroom_after_xdp_frame,
		}
	}
	
	#[inline(always)]
	pub(crate) const fn from_received_frame_descriptor_if_unaligned(frame_descriptor_bitfield: FrameDescriptorBitfield, length_of_packet: u32, frame_headroom: FrameHeadroom) -> Self
	{
		let orig_addr = frame_descriptor_bitfield.orig_addr_if_unaligned();
		let offset = frame_descriptor_bitfield.offset_if_unaligned() as usize;
		let start_of_packet = orig_addr + offset;
		let length_of_packet = length_of_packet as usize;
		let length_of_our_frame_headroom = frame_headroom.into();
		let start_of_our_frame_headroom = start_of_packet - length_of_our_frame_headroom;
		let end_of_xdp_headroom = start_of_our_frame_headroom;
		let xdp_headroom_length = offset - length_of_our_frame_headroom;
		debug_assert_eq!(xdp_headroom_length, XDP_PACKET_HEADROOM);
		let start_of_xdp_headroom = start_of_our_frame_headroom - xdp_headroom_length;
		// let start_of_xdp_headroom_after_xdp_frame = start_of_xdp_headroom + (size_of::<xdp_frame>() as u64);
		// let length_of_xdp_headroom_after_xdp_frame = xdp_headroom_length - size_of::<xdp_frame>();
		
		Self
		{
			start_of_packet,
			length_of_packet,
			start_of_our_frame_headroom,
			length_of_our_frame_headroom,
			end_of_xdp_headroom,
			xdp_headroom_length,
			start_of_xdp_headroom,
			orig_addr,
			offset,
			//start_of_xdp_headroom_after_xdp_frame,
			//length_of_xdp_headroom_after_xdp_frame,
		}
	}
}

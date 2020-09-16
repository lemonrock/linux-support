// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Rx/Tx descriptor.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct xdp_desc
{
	/// This is the `addr` in the functions `xsk_umem__extract_addr()` and `xsk_umem__add_offset_to_addr()` in the Linux source `tools/lib/bpf/xsk.h`.
	///
	/// If using aligned chunks, then this is an address; Linux masks it so that it is a multiple of `ChunkSize`.
	/// If using unaligned chunks, this is a bit field:-
	///
	/// * The top 16 bits contain an offset.
	/// * The bottom 48 bits contain an address.
	addr: u64,
	
	len: u32,
	
	options: u32,
}

impl Descriptor for xdp_desc
{
}

impl xdp_desc
{
	/// Based on `xsk_umem__add_offset_to_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// The return value of this function is the `addr` to `xsk_umem__get_data()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn user_memory_area_relative_address<CA: ChunkAlignment>(&self, chunk_size: ChunkSize) -> UserMemoryAreaRelativeAddress
	{
		CA::user_memory_area_relative_address(chunk_size, self)
	}
	
	#[inline(always)]
	pub(super) fn length(&self) -> usize
	{
		self.len as usize
	}
	
	#[inline(always)]
	pub(super) fn write_frame_reference<CA: ChunkAlignment>(this: NonNull<Self>, frame_reference: &FrameReference, user_memory: &UserMemory<CA>)
	{
		Self::write(this, frame_reference.to_user_memory_area_relative_address(user_memory.chunk_size()), frame_reference.frame_length.into(), 0)
	}
	
	#[inline(always)]
	fn write(this: NonNull<Self>, frame_address: UserMemoryAreaRelativeAddress, frame_length: u32, options: u32)
	{
		debug_assert_eq!(options, 0, "Options must always be zero");
		
		unsafe
		{
			let this = this.as_mut();
			write(&mut this.addr, frame_address);
			write(&mut this.len, frame_length);
			write(&mut this.options, options);
		}
	}
	
	/// Based on `xsk_umem__extract_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Based on `xp_aligned_extract_addr()` in Linux source `net/xdp/xsk_buff_pool.h`, which masks with `chunk_size`.
	/// HOWEVR, `xsk_buff_raw_get_data()` which calls `xp_raw_get_data()` on the transmit path DOES NOT mask with the `chunk_size`.
	#[inline(always)]
	pub(super) fn extract_address_if_aligned(&self, _chunk_size: ChunkSize) -> u64
	{
		self.addr
	}
	
	#[inline(always)]
	pub(super) fn extract_address_if_aligned_using_chunk_mask(&self, chunk_size: ChunkSize) -> u64
	{
		self.addr & chunk_size.mask()
	}
	
	/// Based on `xsk_umem__extract_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Identical to `xp_unaligned_extract_addr()` in Linux source `net/xdp/xsk_buff_pool.h`.
	#[inline(always)]
	pub(super) fn extract_address_if_unaligned(&self) -> u64
	{
		self.addr & XSK_UNALIGNED_BUF_ADDR_MASK
	}
	
	/// Based on `xsk_umem__extract_offset()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Identical to `xp_unaligned_extract_offset()` in Linux source `net/xdp/xsk_buff_pool.h`.
	#[inline(always)]
	pub(super) fn extract_offset_if_unaligned(&self) -> u64
	{
		self.addr >> XSK_UNALIGNED_BUF_OFFSET_SHIFT
	}
}

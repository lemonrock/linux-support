// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct FrameDescriptorBitfield(u64);

impl Descriptor for FrameDescriptorBitfield
{
}

impl Into<u64> for FrameDescriptorBitfield
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl FrameDescriptorBitfield
{
	#[inline(always)]
	pub(crate) fn for_aligned(start_of_packet: UserMemoryAreaRelativeAddress) -> Self
	{
		Self(start_of_packet)
	}
	
	#[inline(always)]
	pub(crate) fn for_aligned_from_orig_addr_and_frame_headroom(orig_addr: UserMemoryAreaRelativeAddress, frame_headroom: FrameHeadroom) -> Self
	{
		Self::for_aligned(RelativeAddressesAndOffsets::start_of_packet_for_fill_queue_if_aligned(orig_addr, frame_headroom))
	}
	
	#[inline(always)]
	pub(crate) fn for_unaligned(orig_addr: UserMemoryAreaRelativeAddress, offset: usize) -> Self
	{
		debug_assert_eq!(orig_addr, orig_addr & XSK_UNALIGNED_BUF_ADDR_MASK);
		
		Self(orig_addr | ((offset as u64) << XSK_UNALIGNED_BUF_OFFSET_SHIFT))
	}
	
	#[inline(always)]
	pub(crate) const fn start_of_packet_if_aligned(self) -> UserMemoryAreaRelativeAddress
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) const fn start_of_packet_if_unaligned(self) -> UserMemoryAreaRelativeAddress
	{
		self.orig_addr_if_unaligned() + self.offset_if_unaligned()
	}
	
	#[inline(always)]
	pub(crate) const fn orig_addr_if_aligned(self, aligned_chunk_size: AlignedChunkSize) -> UserMemoryAreaRelativeAddress
	{
		self.0 & aligned_chunk_size.mask()
	}
	
	#[inline(always)]
	pub(crate) const fn orig_addr_if_unaligned(self) -> UserMemoryAreaRelativeAddress
	{
		self.0 & XSK_UNALIGNED_BUF_ADDR_MASK
	}
	
	#[inline(always)]
	pub(crate) const fn offset_if_aligned(self, aligned_chunk_size: AlignedChunkSize) -> usize
	{
		let orig_addr = self.orig_addr_if_aligned(aligned_chunk_size);
		debug_assert!(orig_addr <= self.0);
		(self.0 - orig_addr) as usize
	}
	
	#[inline(always)]
	pub(crate) const fn offset_if_unaligned(self) -> usize
	{
		(self.0 >> XSK_UNALIGNED_BUF_OFFSET_SHIFT) as usize
	}
}

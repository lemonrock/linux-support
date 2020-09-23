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
		Self(start_of_packet.into_u64())
	}
	
	#[inline(always)]
	pub(crate) fn for_aligned_from_orig_addr_and_frame_headroom(orig_addr: UserMemoryAreaRelativeAddress, frame_headroom: FrameHeadroom) -> Self
	{
		Self::for_aligned(RelativeAddressesAndOffsets::start_of_packet_for_fill_queue_if_aligned(orig_addr, frame_headroom))
	}
	
	#[inline(always)]
	pub(crate) fn for_unaligned(orig_addr: UserMemoryAreaRelativeAddress, offset: usize) -> Self
	{
		let orig_addr = Self::debug_assert_no_offset(orig_addr.into_u64());
		
		Self(Self::encode_offset(offset) | orig_addr)
	}
	
	#[inline(always)]
	pub(crate) fn start_of_packet_if_aligned(self) -> UserMemoryAreaRelativeAddress
	{
		let start_of_packet = Self::debug_assert_no_offset(self.0);
		
		UserMemoryAreaRelativeAddress::from_u64(start_of_packet)
	}
	
	#[inline(always)]
	fn debug_assert_no_offset(value: u64) -> u64
	{
		debug_assert_eq!(value, Self::extract_address(value));
		value
	}
	
	#[inline(always)]
	pub(crate) fn start_of_packet_if_unaligned(self) -> UserMemoryAreaRelativeAddress
	{
		self.orig_addr_if_unaligned() + self.offset_if_unaligned()
	}
	
	#[inline(always)]
	pub(crate) fn orig_addr_if_aligned(self, aligned_chunk_size: AlignedChunkSize) -> UserMemoryAreaRelativeAddress
	{
		let start_of_packet = Self::debug_assert_no_offset(self.0);
		
		UserMemoryAreaRelativeAddress::from_u64(start_of_packet & aligned_chunk_size.mask_u64())
	}
	
	#[inline(always)]
	pub(crate) const fn orig_addr_if_unaligned(self) -> UserMemoryAreaRelativeAddress
	{
		UserMemoryAreaRelativeAddress::from_u64(Self::extract_address(self.0))
	}
	
	#[inline(always)]
	pub(crate) fn offset_if_aligned(self, aligned_chunk_size: AlignedChunkSize) -> usize
	{
		let start_of_packet = self.start_of_packet_if_aligned();
		
		let orig_addr = self.orig_addr_if_aligned(aligned_chunk_size);
		
		start_of_packet - orig_addr
	}
	
	#[inline(always)]
	pub(crate) fn offset_if_unaligned(self) -> usize
	{
		Self::extract_offset(self.0)
	}
	
	const XSK_UNALIGNED_BUF_OFFSET_SHIFT: u64 = 48;
	
	#[inline(always)]
	fn encode_offset(offset: usize) -> u64
	{
		(Self::validate_offset(offset) as u64) << Self::XSK_UNALIGNED_BUF_OFFSET_SHIFT
	}
	
	#[inline(always)]
	fn extract_offset(value: u64) -> usize
	{
		let offset = value >> Self::XSK_UNALIGNED_BUF_OFFSET_SHIFT;
		debug_assert!(offset <= (usize::MAX as u64));
		
		Self::validate_offset(offset as usize)
	}
	
	#[inline(always)]
	const fn extract_address(value: u64) -> u64
	{
		const XSK_UNALIGNED_BUF_ADDR_MASK: u64 = (1 << FrameDescriptorBitfield::XSK_UNALIGNED_BUF_OFFSET_SHIFT) - 1;
		
		value & XSK_UNALIGNED_BUF_ADDR_MASK
	}
	
	#[inline(always)]
	fn validate_offset(offset: usize) -> usize
	{
		const BitsInAByte: u64 = 8;
		const U64SizeInBits: u64 = (size_of::<u64>() as u64) * BitsInAByte;
		const OffsetSizeInBits: u64 = U64SizeInBits - FrameDescriptorBitfield::XSK_UNALIGNED_BUF_OFFSET_SHIFT;
		const ExclusiveMaximum: usize = (1 << OffsetSizeInBits) as usize;
		
		debug_assert!(offset < ExclusiveMaximum);
		offset
	}
}

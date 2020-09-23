// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A frame number; a relative location in user memory of an (Ethernet) frame (packets).
///
/// Frames in user memory do not include a trailing ethernet frame check sequeunces (FCS).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AlignedFrameNumber(u32);

impl From<u32> for AlignedFrameNumber
{
	#[inline(always)]
	fn from(absolute_frame_index: u32) -> Self
	{
		Self(absolute_frame_index)
	}
}

impl Add<u32> for AlignedFrameNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u32) -> Self::Output
	{
		debug_assert!(self.0.checked_add(rhs).is_some());
		
		Self(self.0 + rhs)
	}
}

unsafe impl Step for AlignedFrameNumber
{
	fn steps_between(start: &Self, end: &Self) -> Option<usize>
	{
		u32::steps_between(&start.0, &end.0)
	}
	
	fn forward_checked(start: Self, count: usize) -> Option<Self>
	{
		u32::forward_checked(start.0, count).map(Self)
	}
	
	fn backward_checked(start: Self, count: usize) -> Option<Self>
	{
		u32::backward_checked(start.0, count).map(Self)
	}
}

impl AlignedFrameNumber
{
	pub(crate) const InclusiveMinimum: Self = Self(0);
	
	#[inline(always)]
	pub(crate) fn to_fill_frame_descriptor_bitfield_if_aligned(self, aligned_chunk_size: AlignedChunkSize, frame_headroom: FrameHeadroom) -> FrameDescriptorBitfield
	{
		FrameDescriptorBitfield::for_aligned_from_orig_addr_and_frame_headroom(self.orig_addr_if_aligned(aligned_chunk_size), frame_headroom)
	}
	
	#[inline(always)]
	pub(crate) fn from_received_descriptor_if_aligned(aligned_chunk_size: AlignedChunkSize, received_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self
	{
		// Strictly speaking, `received_frame_descriptor_bitfield.orig_addr_if_aligned(self) / aligned_chunk_size.to_u64()` but aligned frames do not cross an aligned_chunk_size multiple.
		let absolute_frame_index = received_frame_descriptor_bitfield.start_of_packet_if_aligned() / aligned_chunk_size;
		debug_assert!(absolute_frame_index <= (u32::MAX as u64));
		Self(absolute_frame_index as u32)
	}
	
	#[inline(always)]
	pub(crate) fn transmit_frame_descriptor_bitfield_if_aligned(self, aligned_chunk_size: AlignedChunkSize, frame_headroom: FrameHeadroom) -> FrameDescriptorBitfield
	{
		let orig_addr = self.orig_addr_if_aligned(aligned_chunk_size);
		FrameDescriptorBitfield::for_aligned_from_orig_addr_and_frame_headroom(orig_addr, frame_headroom)
	}
	
	#[inline(always)]
	pub(crate) fn transmit_relative_addesses_and_offsets(self, aligned_chunk_size: AlignedChunkSize, frame_headroom: FrameHeadroom, length_of_packet: usize) -> RelativeAddressesAndOffsets
	{
		RelativeAddressesAndOffsets::for_transmitted_frame_descriptor(self.orig_addr_if_aligned(aligned_chunk_size), frame_headroom, length_of_packet)
	}
	
	#[inline(always)]
	pub(crate) fn from_completed_descriptor_if_aligned(aligned_chunk_size: AlignedChunkSize, completed_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self
	{
		// Strictly speaking, `completed_frame_descriptor_bitfield.orig_addr_if_aligned(self) / aligned_chunk_size.to_u64()` but aligned frames do not cross an aligned_chunk_size multiple.
		let absolute_frame_index = completed_frame_descriptor_bitfield.start_of_packet_if_aligned() / aligned_chunk_size;
		debug_assert!(absolute_frame_index <= (u32::MAX as u64));
		Self(absolute_frame_index as u32)
	}
	
	#[inline(always)]
	pub(crate) fn from_relative_addresses_and_offsets_if_aligned(relative_addresss_and_offsets: RelativeAddressesAndOffsets, aligned_chunk_size: AlignedChunkSize) -> Self
	{
		let absolute_frame_index = relative_addresss_and_offsets.orig_addr / aligned_chunk_size;
		debug_assert!(absolute_frame_index <= (u32::MAX as u64));
		Self(absolute_frame_index as u32)
	}
	
	#[inline(always)]
	pub(crate) const fn orig_addr_if_aligned(self, aligned_chunk_size: AlignedChunkSize) -> UserMemoryAreaRelativeAddress
	{
		UserMemoryAreaRelativeAddress::from_u64(self.to_u64() * aligned_chunk_size.into_u64())
	}
	
	#[inline(always)]
	pub(crate) const fn to_u64(self) -> u64
	{
		self.0 as u64
	}
}

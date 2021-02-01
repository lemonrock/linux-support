// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * Must be greater than or equal to `XDP_UMEM_MIN_CHUNK_SIZE` (`2048`).
/// * Must be less than or equal to `PAGE_SIZE` (`4096` on most systems).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnalignedChunkSize(NonZeroU16);

impl Into<u32> for UnalignedChunkSize
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let value: NonZeroU32 = self.into();
		value.get()
	}
}

impl Into<usize> for UnalignedChunkSize
{
	#[inline(always)]
	fn into(self) -> usize
	{
		let value: NonZeroU32 = self.into();
		value.get() as usize
	}
}

impl Into<u64> for UnalignedChunkSize
{
	#[inline(always)]
	fn into(self) -> u64
	{
		let value: NonZeroU32 = self.into();
		value.get() as u64
	}
}

impl Into<NonZeroU32> for UnalignedChunkSize
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		let value = self.0.get();
		new_non_zero_u32(value as u32)
	}
}

impl TryFrom<NonZeroU16> for UnalignedChunkSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if value < Self::InclusiveMinimum.0
		{
			Err(ParseNumberError::TooSmall)
		}
		else if value > Self::inclusive_maximum().0
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl TryFrom<u16> for UnalignedChunkSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(new_non_zero_u16(value))
		}
	}
}

impl Default for UnalignedChunkSize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::inclusive_maximum()
	}
}

impl ChunkSize for UnalignedChunkSize
{
	type FrameIdentifier = UserMemoryAreaRelativeAddress;
	
	const IsUnaligned: bool = true;
	
	const RegistrationFlags: XdpUmemRegFlags = XdpUmemRegFlags::UnalignedChunks;
	
	#[inline(always)]
	fn round_up_number_of_chunks_to_a_multiple_that_fits_exactly_into_multiple_pages(self, number_of_chunks: NonZeroU32) -> NonZeroU32
	{
		number_of_chunks
	}
	
	#[doc(hidden)]
	fn validate_user_memory(user_memory_area_page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings)
	{
		debug_assert!(user_memory_area_page_size_or_huge_page_size_settings.is_using_huge_pages(), "When using XdpUmemRegFlagsUnalignedChunks in `flags`, Huge Pages must be used");
	}
	
	#[inline(always)]
	fn received_relative_addresses_and_offsets(received_descriptor: &FrameDescriptor, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets
	{
		received_descriptor.received_relative_addresses_and_offsets_if_unaligned(frame_headroom)
	}
	
	#[inline(always)]
	fn fill_frame_descriptor_bitfield_if_constructed_from_received_frame_descriptor(relative_addresses_and_offsets: &RelativeAddressesAndOffsets) -> FrameDescriptorBitfield
	{
		relative_addresses_and_offsets.fill_frame_descriptor_bitfield_if_constructed_from_received_frame_descriptor_if_unaligned()
	}
	
	#[inline(always)]
	fn from_completed_frame_descriptor(completed_frame_descriptor_bitfield: FrameDescriptorBitfield, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets
	{
		RelativeAddressesAndOffsets::from_completed_frame_descriptor_if_unaligned(completed_frame_descriptor_bitfield, frame_headroom)
	}
	
	#[inline(always)]
	fn received_frame_identifier(self, received_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self::FrameIdentifier
	{
		received_frame_descriptor_bitfield.orig_addr_if_unaligned()
	}
	
	#[inline(always)]
	fn completed_frame_identifier(self, completed_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self::FrameIdentifier
	{
		completed_frame_descriptor_bitfield.orig_addr_if_unaligned()
	}
	
	#[inline(always)]
	fn fill_frame_descriptor_bitfield(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier) -> FrameDescriptorBitfield
	{
		let relative_addesses_and_offsets = RelativeAddressesAndOffsets::for_transmitted_frame_descriptor(frame_identifier, frame_headroom, 0);
		relative_addesses_and_offsets.fill_frame_descriptor_bitfield_if_unaligned()
	}
	
	#[inline(always)]
	fn transmit_frame_descriptor_bitfield(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier) -> FrameDescriptorBitfield
	{
		let relative_addesses_and_offsets = self.transmit_relative_addesses_and_offsets(frame_headroom, frame_identifier, 0);
		relative_addesses_and_offsets.transmit_frame_descriptor_bitfield_if_unaligned()
	}
	
	#[inline(always)]
	fn transmit_relative_addesses_and_offsets(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier, length_of_packet: usize) -> RelativeAddressesAndOffsets
	{
		RelativeAddressesAndOffsets::for_transmitted_frame_descriptor(frame_identifier, frame_headroom, length_of_packet)
	}
}

impl UnalignedChunkSize
{
	/// `XDP_UMEM_MIN_CHUNK_SIZE`.
	pub const InclusiveMinimum: Self = Self::new_unchecked(2048);
	
	/// Inclusive maximum (page size).
	#[inline(always)]
	#[cfg(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64"))]
	pub const fn inclusive_maximum() -> Self
	{
		Self::new_unchecked(PageSize::current() as u16)
	}
	
	/// Inclusive maximum (page size).
	#[inline(always)]
	#[cfg(not(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64")))]
	pub fn inclusive_maximum() -> Self
	{
		Self::new_unchecked(PageSize::current() as u16)
	}
	
	/// To `u32`.
	#[inline(always)]
	pub const fn to_u32(self) -> u32
	{
		self.0.get() as u32
	}
	
	#[inline(always)]
	const fn new_unchecked(value: u16) -> Self
	{
		Self(new_non_zero_u16(value))
	}
}

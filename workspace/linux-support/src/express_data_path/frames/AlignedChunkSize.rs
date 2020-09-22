// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Frame size in bytes.
///
/// Represents the size of elements in user memory used for storing (Ethernet) frames (packets).
///
/// In DPDK, contains the header, packet and trailer.
///
/// It is the maximum size of `len(ethernet_packet) + frame_headroom`.
///
/// Called `chunk_size` in some Linux documentation and code.
///
/// * Must be a power of two.
/// * Must be greater than or equal to `XDP_UMEM_MIN_CHUNK_SIZE` (`2048`).
/// * Must be less than or equal to `PAGE_SIZE` (`4096` on most systems; aarch 16Kb and 64Kb page sizes are not supported here as they cannot be statically determined to be in effect).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AlignedChunkSize
{
	#[allow(missing_docs)]
	_2048 = 2048,
	
	/// This is the default used by `libbpf` as `XSK_UMEM__DEFAULT_FRAME_SIZE`.
	_4096 = 4096,

	/// On Sparc64.
	#[cfg(target_arch = "sparc64")]
	_8192 = 8192,
}

impl Default for AlignedChunkSize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::_4096
	}
}

impl Into<NonZeroU32> for AlignedChunkSize
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		unsafe { transmute(self )}
	}
}

impl ChunkSize for AlignedChunkSize
{
	type FrameIdentifier = AlignedFrameNumber;
	
	const IsUnaligned: bool = false;
	
	const RegistrationFlags: XdpUmemRegFlags = XdpUmemRegFlags::empty();
	
	#[inline(always)]
	fn round_up_number_of_chunks(self, number_of_chunks: NonZeroU32) -> NonZeroU32
	{
		let chunks_per_page =
		{
			let page_size = PageSize::current().size_in_bytes().get() as u32;
			let chunk_size = self.into().get();
			
			debug_assert!(page_size >= chunk_size);
			debug_assert_eq!(page_size % chunk_size, 0);
			
			page_size / chunk_size
		};
		unsafe { NonZeroU32::new_unchecked((number_of_chunks.get() + chunks_per_page - 1) / chunks_per_page) }
	}
	
	#[doc(hidden)]
	fn validate_user_memory(_huge_memory_page_size: Option<Option<HugePageSize>>)
	{
	}
	
	#[inline(always)]
	fn received_relative_addresses_and_offsets(received_descriptor: &FrameDescriptor, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets
	{
		receive_descriptor.received_relative_addresses_and_offsets_if_aligned(frame_headroom)
	}
	
	#[inline(always)]
	fn fill_frame_descriptor_bitfield_if_constructed_from_received_frame_descriptor(relative_addresses_and_offsets: &RelativeAddressesAndOffsets) -> FrameDescriptorBitfield
	{
		relative_addresses_and_offsets.fill_frame_descriptor_bitfield_if_constructed_from_received_frame_descriptor_if_aligned()
	}
	
	#[inline(always)]
	fn from_completed_frame_descriptor(completed_frame_descriptor_bitfield: FrameDescriptorBitfield, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets
	{
		RelativeAddressesAndOffsets::from_completed_frame_descriptor_if_aligned(frame_descriptor_bitfield, frame_headroom)
	}
	
	#[inline(always)]
	fn received_frame_identifier(self, received_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self::FrameIdentifier
	{
		AlignedFrameNumber::from_received_descriptor_if_aligned(self, received_frame_descriptor_bitfield)
	}
	
	#[inline(always)]
	fn completed_frame_identifier(self, completed_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self::FrameIdentifier
	{
		AlignedFrameNumber::from_completed_descriptor_if_aligned(self, completed_frame_descriptor_bitfield)
	}
	
	#[inline(always)]
	fn fill_frame_descriptor_bitfield(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier) -> FrameDescriptorBitfield
	{
		frame_identifier.to_fill_frame_descriptor_bitfield_if_aligned(self.chunk_size, self.frame_headroom)
	}
	
	#[inline(always)]
	fn transmit_frame_descriptor_bitfield(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier) -> FrameDescriptorBitfield
	{
		frame_identifier.transmit_frame_descriptor_bitfield_if_aligned(self, frame_headroom)
	}
	
	#[inline(always)]
	fn transmit_relative_addesses_and_offsets(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier, length_of_packet: usize) -> RelativeAddressesAndOffsets
	{
		frame_identifier.transmit_relative_addesses_and_offsets(self, frame_headroom, length_of_packet)
	}
}

impl AlignedChunkSize
{
	#[inline(always)]
	pub(crate) const fn mask(self) -> u64
	{
		!((self as u32 as u64) - 1)
	}
	
	#[inline(always)]
	pub(crate) const fn to_u32(self) -> u32
	{
		self as u32
	}
	
	#[inline(always)]
	pub(crate) const fn to_u64(self) -> u64
	{
		self.to_u32() as u64
	}
}

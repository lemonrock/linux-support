// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Chunk size.
pub trait ChunkSize: Default + Debug + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Into<NonZeroU32> + Into<u32> + Into<usize> + Into<u64>
{
	/// A frame identifier, such as an `AlignedFrameNumber`.
	type FrameIdentifier: Copy;
	
	#[doc(hidden)]
	const IsUnaligned: bool;
	
	#[doc(hidden)]
	const RegistrationFlags: XdpUmemRegFlags;
	
	#[doc(hidden)]
	#[inline(always)]
	fn user_memory_area_length(self, number_of_chunks: NonZeroU32) -> NonZeroU64
	{
		let number_of_chunks = number_of_chunks.get() as u64;
		let chunk_size: u64 = self.into();
		new_non_zero_u64(number_of_chunks * chunk_size)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn compare_to_frame_sizes(self, frame_headroom: FrameHeadroom, maximum_transmission_unit_payload_size: MaximumTransmissionUnitPayloadSize) -> Ordering
	{
		let value: usize = self.into();
		
		let minimum_required_frame_size = frame_headroom.with_xdp_packet_headroom_before_frame_headroom() + maximum_transmission_unit_payload_size.frame_size_including_trailing_frame_check_sequence();
		
		value.cmp(&minimum_required_frame_size)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn calculate_maximum_transmission_unit_payload_size(self, frame_headroom: FrameHeadroom) -> Result<MaximumTransmissionUnitPayloadSize, ParseNumberError>
	{
		let overhead = frame_headroom.with_xdp_packet_headroom_before_frame_headroom() + MaximumTransmissionUnitPayloadSize::EthernetFrameOverheadIncludingTrailingFrameCheckSequence;
		let value: usize = self.into();
		
		let maximum_length_of_packet = value.checked_sub(overhead).ok_or(ParseNumberError::TooShortWithMinusSign)?;
		
		MaximumTransmissionUnitPayloadSize::try_from(maximum_length_of_packet)
	}
	
	#[doc(hidden)]
	fn round_up_number_of_chunks_to_a_multiple_that_fits_exactly_into_multiple_pages(self, number_of_chunks: NonZeroU32) -> NonZeroU32;
	
	#[doc(hidden)]
	fn validate_user_memory(user_memory_area_page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings);
	
	#[doc(hidden)]
	fn received_relative_addresses_and_offsets(received_descriptor: &FrameDescriptor, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets;
	
	#[doc(hidden)]
	fn fill_frame_descriptor_bitfield_if_constructed_from_received_frame_descriptor(relative_addresses_and_offsets: &RelativeAddressesAndOffsets) -> FrameDescriptorBitfield;
	
	#[doc(hidden)]
	fn from_completed_frame_descriptor(completed_frame_descriptor_bitfield: FrameDescriptorBitfield, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets;
	
	#[doc(hidden)]
	fn received_frame_identifier(self, received_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self::FrameIdentifier;
	
	#[doc(hidden)]
	fn completed_frame_identifier(self, completed_frame_descriptor_bitfield: FrameDescriptorBitfield) -> Self::FrameIdentifier;
	
	#[doc(hidden)]
	fn fill_frame_descriptor_bitfield(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier) -> FrameDescriptorBitfield;
	
	#[doc(hidden)]
	fn transmit_frame_descriptor_bitfield(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier) -> FrameDescriptorBitfield;
	
	#[doc(hidden)]
	fn transmit_relative_addresses_and_offsets(self, frame_headroom: FrameHeadroom, frame_identifier: Self::FrameIdentifier, length_of_packet: usize) -> RelativeAddressesAndOffsets;
}

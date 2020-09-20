// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A frame reference (effectively, a fat pointer).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlignedFrameReference
{
	/// Frame number.
	pub aligned_frame_number: AlignedFrameNumber,
	
	/// This is the length of data in an Ethernet packet.
	///
	/// ?It excludes the trailing Frame Check Sequence (FCS)?
	pub length_of_packet: usize,
}

impl AlignedFrameReference
{
	#[inline(always)]
	pub(crate) fn write_for_transmit_aligned(&self, transmit_descriptor: NonNull<FrameDescriptor>, aligned_chunk_size: AlignedChunkSize, frame_headroom: FrameHeadroom)
	{
		let orig_addr = self.aligned_frame_number.to_user_memory_descriptor(aligned_chunk_size).into();
		let relative_address_and_offsets = RelativeAddressesAndOffsets::for_transmitted_frame_descriptor_if_aligned(orig_addr, frame_headroom, self.length_of_packet);
		
		FrameDescriptor::write_for_transmit_aligned(transmit_descriptor, relative_address_and_offsets.start_of_packet, relative_address_and_offsets.length_of_packet)
	}
}

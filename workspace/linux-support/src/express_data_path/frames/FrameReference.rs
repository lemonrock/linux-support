// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A frame reference (effectively, a fat pointer).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameReference<CS: ChunkSize>
{
	/// Frame identifier, either `AlignedFrameNumber` or `UserMemoryAreaRelativeAddress`.
	frame_identifier: CS::FrameIdentifier,
	
	/// This is the length of data in an Ethernet packet.
	///
	/// ?It includes the trailing Frame Check Sequence (FCS)?
	pub(crate) length_of_packet: usize,
}

impl<FFQ: FreeFrameQueue> FrameReference<FFQ::CS>
{
	/// Must write a frame descriptor that works with `xsk_buff_raw_get_data()` in Linux source.
	/// That function requires `desc.addr == start_of_packet`.
	#[inline(always)]
	pub(crate) fn transmit_frame_descriptor_bitfield(&self, user_memory: &UserMemory<FFQ>) -> FrameDescriptorBitfield
	{
		user_memory.frame_identifier_to_transmit_frame_descriptor_bitfield(self.frame_identifier)
	}
}

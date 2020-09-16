// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A frame reference (effectively, a fat pointer).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameReference
{
	/// Relative address.
	pub frame_number: FrameNumber,
	
	/// This is the length of data in an Ethernet packet.
	///
	/// It excludes the trailing Frame Check Sequence (FCS).
	///
	/// Not sure how this relates to a non-zero `FrameHeadroom` or unaligned frames.
	///
	/// This has a minimum value.
	pub frame_length: FrameLength,
}

impl FrameReference
{
	#[inline(always)]
	pub fn frame<CA: ChunkAlignment>(&self, user_memory: &UserMemory<CA>, chunk_size: ChunkSize) -> (&mut [u8], &mut [u8])
	{
		user_memory.frame_from_frame_reference(self, chunk_size)
	}
	
	/// `chunk_size` is the the size of a memory array element in user memory.
	///
	/// It is the maximum `frame_length` can be.
	#[inline(always)]
	pub(crate) fn to_user_memory_area_relative_address(&self, chunk_size: ChunkSize) -> UserMemoryAreaRelativeAddress
	{
		debug_assert!(self.frame_length as u32 <= chunk_size as u32, "`frame_length` is larger than `chunk_size`");
		
		self.frame_number.to_user_memory_area_relative_address(chunk_size)
	}
}

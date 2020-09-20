// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// UMEM (User space memory) descriptor.
///
/// Used for `FillQueue` and `CompletionQueue`.
///
/// This is a relative memory address.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UserMemoryDescriptor(u64);

impl From<u64> for UserMemoryDescriptor
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		Self(value)
	}
}

impl Into<u64> for UserMemoryDescriptor
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Add<u64> for UserMemoryDescriptor
{
	type Output = u64;
	
	#[inline(always)]
	fn add(self, value: u64) -> Self::Output
	{
		self.0 + value
	}
}

impl Descriptor for UserMemoryDescriptor
{
}

impl UserMemoryDescriptor
{
	/// Raw address, before any frame headroom.
	#[inline(always)]
	pub(crate) fn absolute_address_of_frame(self, user_memory_area: &UserMemoryArea) -> NonNull<u8>
	{
		user_memory_area.virtual_address().add(self.0).into()
	}
	
	/// To an `AlignedFrameNumber`; only correct if aligned chunks are being used.
	#[inline(always)]
	pub const fn to_aligned_frame_number(self, aligned_chunk_size: AlignedChunkSize) -> AlignedFrameNumber
	{
		let absolute_frame_index = self.0 / aligned_chunk_size.to_u64();
		AlignedFrameNumber::from(absolute_frame_index as u32)
	}
	
	/// From an `AlignedFrameNumber`; only correct if aligned chunks are being used.
	#[inline(always)]
	pub const fn from_aligned_frame_number(aligned_frame_number: AlignedFrameNumber, aligned_chunk_size: AlignedChunkSize) -> Self
	{
		aligned_frame_number.to_user_memory_descriptor(aligned_chunk_size)
	}
}

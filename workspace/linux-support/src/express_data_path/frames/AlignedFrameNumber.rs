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

impl Add for AlignedFrameNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u32) -> Self::Output
	{
		debug_assert(self.0.checked_add(rhs).is_some());
		
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
	/// To an `UserMemoryDescriptor`; only correct if aligned chunks are being used.
	#[inline(always)]
	pub const fn to_user_memory_descriptor(self, aligned_chunk_size: AlignedChunkSize) -> UserMemoryDescriptor
	{
		UserMemoryDescriptor::from(self.to_u64() * aligned_chunk_size.to_u64())
	}
	
	/// From an `UserMemoryDescriptor`; only correct if aligned chunks are being used.
	#[inline(always)]
	pub const fn from_user_memory_descriptor(user_memory_descriptor: UserMemoryDescriptor, aligned_chunk_size: AlignedChunkSize) -> Self
	{
		user_memory_descriptor.to_aligned_frame_number(aligned_chunk_size)
	}
	
	#[inline(always)]
	pub(crate) const fn to_u64(self) -> u64
	{
		self.0 as u64
	}
}

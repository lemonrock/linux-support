// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A frame number; a relative location in user memory of an (Ethernet) frame (packets).
///
/// Frames in user memory do not include a trailing ethernet frame check sequeunces (FCS).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FrameNumber(u32);

impl Add for FrameNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u32) -> Self::Output
	{
		debug_assert(self.0.checked_add(rhs).is_some());
		
		Self(self.0 + rhs)
	}
}

unsafe impl Step for FrameNumber
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

impl FrameNumber
{
	/// To an `UserMemoryAreaRelativeAddress`; only correct if unaligned chunks are not allowed.
	#[inline(always)]
	pub const fn to_user_memory_area_relative_address(self, chunk_size: ChunkSize) -> UserMemoryAreaRelativeAddress
	{
		(self.0 as u64) * (chunk_size as u32 as u64)
	}
}

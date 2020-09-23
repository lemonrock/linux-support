// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct RelativeFrameIndex(u32);

impl RelativeFrameIndex
{
	pub(crate) const Zero: Self = Self(0);
	
	#[inline(always)]
	pub(crate) fn relative_frame_indices(number_of_frames: NonZeroU32) -> impl Iterator<Item=Self>
	{
		(0 .. (number_of_frames.get())).map(|index| Self(index))
	}
	
	#[inline(always)]
	pub(crate) const fn into_u32(self) -> u32
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) const fn into_usize(self) -> usize
	{
		self.0 as usize
	}
	
	#[inline(always)]
	pub(crate) fn next(&mut self)
	{
		self.0 += 1
	}
}

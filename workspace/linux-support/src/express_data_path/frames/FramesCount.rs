// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct FramesCount(Cell<u64>);

impl FramesCount
{
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		Self(Cell::new(0))
	}
	
	#[inline(always)]
	pub(crate) fn current(&self) -> u64
	{
		self.0.get()
	}
	
	#[inline(always)]
	pub(crate) fn increment(&self, number_of_frames: NonZeroU32)
	{
		self.0.set(self.current() + number_of_frames as u64)
	}
}

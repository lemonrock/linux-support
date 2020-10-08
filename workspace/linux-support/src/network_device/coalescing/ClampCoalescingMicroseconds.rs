// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A tuple of minimum, preferred and actual.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ClampCoalescingMicroseconds
{
	/// Inclusive minimum microseconds.
	///
	/// Can be zero.
	pub inclusive_minimum_microseconds: u32,
	
	/// Preferred microseconds.
	///
	/// Can be zero.
	pub preferred_microseconds: u32,
	
	/// Inclusive maximum microseconds.
	///
	/// Can be, but probably shouldn't be, zero.
	pub inclusive_maximum_microseconds: u32,
}

impl ClampCoalescingMicroseconds
{
	#[inline(always)]
	pub const fn new(inclusive_minimum_microseconds: u32, preferred_microseconds: u32, inclusive_maximum_microseconds: u32) -> Self
	{
		Self
		{
			inclusive_minimum_microseconds,
			preferred_microseconds,
			inclusive_maximum_microseconds
		}
	}
	
	#[inline(always)]
	fn clamp(self) -> Option<NonZeroU32>
	{
		unsafe { transmute(max(self.inclusive_minimum_microseconds, min(self.preferred_microseconds, self.inclusive_maximum_microseconds))) }
	}
}

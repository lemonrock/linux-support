// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Weight.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Weight(pub u16);

impl Weight
{
	pub(crate) const Unassigned: Self = Self(0);
	
	#[inline(always)]
	const fn is_weightless(self) -> bool
	{
		self.0 == 0
	}
	
	#[inline(always)]
	const fn into_non_zero_u16(self) -> NonZeroU16
	{
		debug_assert_ne!(self.0, 0);
		unsafe { NonZeroU16::new_unchecked(self.0) }
	}
	
	#[inline(always)]
	fn zero_or_not_compare(&self, rhs: &Self) -> Ordering
	{
		let left = self.0;
		let right = rhs.0;
		
		use self::Ordering::*;
		match (left, right)
		{
			(0, 0) => Equal,
			(0, _) => Less,
			(_, 0) => Greater,
			(_, _) => Equal,
		}
	}
}

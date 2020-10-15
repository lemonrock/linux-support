// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait UnsafeCastMut: UnsafeCast
{
	#[inline(always)]
	fn as_usize_pointer_mut(&mut self) -> usize
	{
		self as *mut Self as *mut () as usize
	}

	#[inline(always)]
	fn unsafe_cast_mut<To>(&mut self) -> &mut To
	{
		unsafe { &mut * (self.as_usize_pointer_mut() as *mut To) }
	}

	#[inline(always)]
	fn unsafe_cast_mut_non_null<To>(&mut self) -> NonNull<To>
	{
		unsafe { NonNull::new_unchecked(self.as_usize_pointer_mut() as *mut To) }
	}

	#[inline(always)]
	fn unsafe_cast_slice_mut<To>(&mut self, length: usize) -> &mut [To]
	{
		unsafe { from_raw_parts_mut(self.unsafe_cast_mut::<To>(), length) }
	}
}

impl<T: UnsafeCast> UnsafeCastMut for T
{
}

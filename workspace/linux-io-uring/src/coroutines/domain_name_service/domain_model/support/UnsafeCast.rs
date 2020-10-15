// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait UnsafeCast
{
	#[inline(always)]
	fn as_usize_pointer(&self) -> usize
	{
		self as *const Self as *const () as usize
	}

	#[inline(always)]
	fn unsafe_cast<To>(&self) -> &To
	{
		unsafe { & * (self.as_usize_pointer() as *const To) }
	}

	#[inline(always)]
	fn unsafe_cast_slice<To>(&self, length: usize) -> &[To]
	{
		unsafe { from_raw_parts(self.unsafe_cast::<To>(), length) }
	}
}

impl<T: ?Sized> UnsafeCast for T
{
}

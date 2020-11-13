// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fully inlined, completely unverified code to allocate as zeroed.
#[inline(always)]
pub fn unsafe_zeroed<T>() -> T
{
	unsafe { assert_zero_valid::<T>(); }
	
	let mut u = MaybeUninit::<T>::uninit();
	unsafe
	{
		u.as_mut_ptr().write_bytes(0u8, 1);
		u.assume_init()
	}
}

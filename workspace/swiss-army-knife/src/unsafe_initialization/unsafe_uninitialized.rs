// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fully inlined, completely unverified code to allocate as uninitialized and remove deprecation warnings (and risk of `std::mem::uninitialized()`) function disappearing from Rust).
#[inline(always)]
pub fn unsafe_uninitialized<T>() -> T
{
	unsafe { assert_uninit_valid::<T>(); }
	let u = MaybeUninit::<T>::uninit();
	unsafe { u.assume_init() }
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This function is missing from Rust's intrinsics.
#[inline(always)]
pub unsafe fn _mm512_cvtsi512_si32(a: __m512i) -> i32
{
	let b: __v16si = transmute(a);
	b.get_unchecked_value_safe(0)
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[inline(always)]
pub unsafe fn _mm_reduce_add_epu32(a: __m128i) -> u32
{
	let upper_64_bits = _mm_unpackhi_epi64(a, a);
	let sum_64 = _mm_add_epi32(upper_64_bits, a);
	
	// Swap the lower two elements (lane 0, `w`, goes to lane 1; lane 1, `x`, goes to lane 0).
	let upper_32_bits = _mm_shuffle_epi32(sum_64, _MM_SHUFFLE(2, 3, 0, 1));
	let sum_32 = _mm_add_epi32(sum_64, upper_32_bits);
	
	_mm_cvtsi128_si32(sum_32) as u32
}

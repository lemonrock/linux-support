// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[inline(always)]
pub unsafe fn _mm_reduce_add_epu8(a: __m128i) -> u32
{
	let horizontal_sum_16_bit_integers = _mm_sad_epu8(a, _mm_setzero_si128());
	(_mm_extract_epi16(horizontal_sum_16_bit_integers, 0) + _mm_extract_epi16(horizontal_sum_16_bit_integers, 4)) as u32
}

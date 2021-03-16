// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[cfg(target_feature = "sse2")]
#[inline(always)]
pub unsafe fn _mm256_reduce_add_epu32(a: __m256i) -> u32
{
	let lower_128_bits = _mm256_castsi256_si128(a);
	let upper_128_bits = _mm256_extracti128_si256(a, 1);
	let sum_128 = _mm_add_epi32(lower_128_bits,upper_128_bits);
	
	return _mm_reduce_add_epu32(sum_128);
}

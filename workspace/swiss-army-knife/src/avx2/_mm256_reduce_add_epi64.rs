// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[inline(always)]
pub unsafe fn _mm256_reduce_add_epi64(a: __m256i) -> i64
{
	let shifted = _mm256_shuffle_epi32(a, 0b00_00_11_10);
	let sum_adjacent_pairs = _mm256_add_epi64(a, shifted);
	let high_128_bits = _mm256_extracti128_si256(sum_adjacent_pairs, 1);
	let low_128_bits = _mm256_castsi256_si128(sum_adjacent_pairs);
	let sum = _mm_add_epi64(low_128_bits, high_128_bits);
	_mm_cvtsi128_si64(sum)
}

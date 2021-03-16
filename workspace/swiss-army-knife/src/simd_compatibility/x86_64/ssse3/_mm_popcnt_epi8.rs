// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Based on <https://github.com/WojciechMula/sse-popcount/blob/master/popcnt-avx2-lookup.cpp>.
///
/// Only efficient if `lookup_table` and `nibble_mask` are inlined and hoisted out of any loop by the compiler.
#[inline(always)]
pub unsafe fn _mm_popcnt_epi8(a: __m128i) -> __m128i
{
	// This is a look up table (LUT) of nibbles.
	let lookup_table = _mm_setr_epi8
	(
		0, 1, 1, 2,
		1, 2, 2, 3,
		1, 2, 2, 3,
		2, 3, 3, 4,
	);
	
	let nibble_mask = _mm_set1_epi8(0x0F);
	
	let low_nibble = _mm_and_si128(a, nibble_mask);
	let low_nibble_population_count = _mm_shuffle_epi8(lookup_table, low_nibble);
	
	let high_nibble = _mm_and_si128(_mm_srli_epi16(a, 4), nibble_mask);
	let high_nibble_population_count = _mm_shuffle_epi8(lookup_table, high_nibble);
	
	_mm_add_epi8(low_nibble_population_count, high_nibble_population_count)
}

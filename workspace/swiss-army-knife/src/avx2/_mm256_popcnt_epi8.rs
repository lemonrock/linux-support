// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Based on <https://github.com/WojciechMula/sse-popcount/blob/master/popcnt-avx2-lookup.cpp>.
#[inline(always)]
pub unsafe fn _mm256_popcnt_epi8(a: __m256i) -> __m256i
{
	// This is a table of nibbles.
	//
	// It is divided into two 128-bit lanes of identical look up values as `_mm256_shuffle_epi8()` operates on 128-bit lanes.
	let lookup_table = _mm256_setr_epi8
	(
		0, 1, 1, 2,
		1, 2, 2, 3,
		1, 2, 2, 3,
		2, 3, 3, 4,
		
		0, 1, 1, 2,
		1, 2, 2, 3,
		1, 2, 2, 3,
		2, 3, 3, 4,
	);
	
	let nibble_mask = _mm256_set1_epi8(0x0F);
	
	let low_nibble = _mm256_and_si256(a, nibble_mask);
	let low_nibble_population_count = _mm256_shuffle_epi8(lookup_table, low_nibble);
	
	let high_nibble = _mm256_and_si256(_mm256_srli_epi16(a, 4), nibble_mask);
	let high_nibble_population_count = _mm256_shuffle_epi8(lookup_table, high_nibble);
	
	_mm256_add_epi8(low_nibble_population_count, high_nibble_population_count)
}

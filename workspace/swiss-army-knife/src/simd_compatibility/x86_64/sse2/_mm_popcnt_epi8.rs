// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Based on <https://stackoverflow.com/questions/17354971/fast-counting-the-number-of-set-bits-in-m128i-register/17355341#17355341>.
///
/// Only efficient if `popcount_mask1` and `popcount_mask2` are inlined and hoisted out of any loop by the compiler.
#[inline(always)]
pub unsafe fn _mm_popcnt_epi8(a: __m128i) -> __m128i
{
	let popcount_mask1 = _mm_set1_epi8(0x77);
	let popcount_mask2 = _mm_set1_epi8(0x0F);
	
	let n = _mm_srli_epi64(a, 1);
	let n = _mm_and_si128(popcount_mask1, n);
	let x = _mm_sub_epi8(a, n);
	let n = _mm_srli_epi64(n, 1);
	let n = _mm_and_si128(popcount_mask1, n);
	let x = _mm_sub_epi8(x, n);
	let n = _mm_srli_epi64(n, 1);
	let n = _mm_and_si128(popcount_mask1, n);
	let x = _mm_sub_epi8(x, n);
	let x = _mm_add_epi8(x, _mm_srli_epi16(x, 4));
	let x = _mm_and_si128(popcount_mask2, x);
	x
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[cfg(all(target_feature = "avx2", target_feature = "sse2"))]
#[inline(always)]
pub unsafe fn _mm512_reduce_add_epu32(a: __m512i) -> u32
{
	let lower_256_bits = _mm512_castsi512_si256(a);
	let upper_256_bits = _mm512_extracti64x4_epi64(a, 1);
	let sum_256 = _mm256_add_epi32(lower_256_bits, upper_256_bits);
	
	_mm256_reduce_add_epu32(sum_256)
}

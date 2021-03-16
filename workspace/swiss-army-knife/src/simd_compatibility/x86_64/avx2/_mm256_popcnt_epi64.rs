// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Based on <https://github.com/WojciechMula/sse-popcount/blob/master/popcnt-avx2-lookup.cpp>.
#[inline(always)]
pub unsafe fn _mm256_popcnt_epi64(a: __m256i) -> __m256i
{
	_mm256_sad_epu8(_mm256_popcnt_epi8(a), _mm256_setzero_si256())
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Shuffles.
#[inline(always)]
pub unsafe fn _mm_shuffle_epi64_constant<const imm8: i32>(a: __m128i, b: __m128i) -> __m128i
{
	_mm_castpd_si128(_mm_shuffle_pd(_mm_castsi128_pd(a), _mm_castsi128_pd(b), imm8))
}

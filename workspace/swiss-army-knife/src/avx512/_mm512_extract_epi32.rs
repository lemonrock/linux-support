// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F but present in similar form in AVX.
///
/// Inefficient.
#[inline(always)]
pub unsafe fn _mm512_extract_epi32(a: __m512i, index: i32) -> i32
{
	_mm512_cvtsi512_si32(_mm512_alignr_epi32(a, a, index))
}

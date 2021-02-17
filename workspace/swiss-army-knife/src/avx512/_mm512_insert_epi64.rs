// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F but present in similar form in AVX.
#[inline(always)]
pub unsafe fn _mm512_insert_epi64(a: __m512i, i: i64, index: i32) -> __m512i
{
	_mm512_mask_set1_epi64(a, 1 << index, i)
}

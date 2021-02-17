// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F but present in similar form in AVX.
///
/// Can only be provided if the `avx512bw` target feature is enabled.
#[cfg(target_feature = "avx512bw")]
#[inline(always)]
pub unsafe fn _mm512_insert_epi8(a: __m512i, i: i8, index: i32) -> __m512i
{
	std::arch::x86_64::_mm512_mask_set1_epi8(a, 1 << index, i)
}

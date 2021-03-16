// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F.
///
/// Inefficient.
#[cfg(target_feature = "avx512bw")]
#[inline(always)]
pub unsafe fn _mm512_reduce_add_epu8(a: __m512i) -> u64
{
	_mm512_reduce_add_epi64(_mm512_sad_epu8(a, _mm512_setzero_si512())) as u64
}

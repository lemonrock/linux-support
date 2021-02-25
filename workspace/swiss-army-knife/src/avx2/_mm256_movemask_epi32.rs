// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is efficient and compiles down to the same assembler as `_mm256_movemask_ps()`.
///
/// Returns a bit set with bits 0 to 7 being set if a 32-bit lane has a significant bit set.
#[inline(always)]
pub unsafe fn _mm256_movemask_epi32(a: __m256i) -> i32
{
	_mm256_movemask_ps(_mm256_castsi256_ps(a))
}

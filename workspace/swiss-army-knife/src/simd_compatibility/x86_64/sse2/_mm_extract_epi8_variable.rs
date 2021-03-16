// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
///
/// It is useful if `imm8` is not constant.
#[inline(always)]
pub unsafe fn _mm_extract_epi8_variable(a: __m128i, imm8: i32) -> i8
{
	let index = _mm_cvtsi32_si128(imm8);
	let val = _mm_shuffle_epi8(a, index);
	_mm_cvtsi128_si32(val) as i8
}

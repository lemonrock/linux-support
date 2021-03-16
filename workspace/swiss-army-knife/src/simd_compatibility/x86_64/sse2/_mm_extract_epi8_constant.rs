// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[inline(always)]
pub unsafe fn _mm_extract_epi8_constant<const imm8: i32>(a: __m128i) -> i8
{
	let value_u16 = _mm_extract_epi16(a, imm8 / 2) as u16;
	let shifted = (value_u16 >> ((imm8 % 2) * 8)) as u8;
	shifted as i8
}

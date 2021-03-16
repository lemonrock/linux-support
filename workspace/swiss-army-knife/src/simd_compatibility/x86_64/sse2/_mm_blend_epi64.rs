// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is efficient and compiles down to the same assembler as `_mm_blend_epi32()` but with a different immediate value used for the control word.
///
/// `imm8` is a control value between `0` and `7` inclusive; if a bit is set, then the equivalent lane from `b` is copied otherwise the lane from `a` is copied.
#[inline(always)]
pub unsafe fn _mm_blend_epi64(a: __m128i, b: __m128i, imm8: i32) -> __m128i
{
	debug_assert!(imm8 >= 0, "imm8 must be zero or positive, not `{}`", imm8);
	debug_assert!(imm8 <= 0b1111, "imm8 must be less than or equal to 15, not `{}`", imm8);
	
	macro_rules! _mm_blend_epi64_const
	{
		($imm8: expr) =>
		{
			_mm_castpd_si128(_mm_blend_pd(_mm_castsi128_pd(a), _mm_castsi128_pd(b), $imm8))
		}
	}
	
	match imm8
	{
		0 => _mm_blend_epi64_const!(0),
		1 => _mm_blend_epi64_const!(1),
		2 => _mm_blend_epi64_const!(2),
		3 => _mm_blend_epi64_const!(3),
		4 => _mm_blend_epi64_const!(4),
		5 => _mm_blend_epi64_const!(5),
		6 => _mm_blend_epi64_const!(6),
		7 => _mm_blend_epi64_const!(7),
		_ => unreachable_code_const("Invalid value"),
	}
}

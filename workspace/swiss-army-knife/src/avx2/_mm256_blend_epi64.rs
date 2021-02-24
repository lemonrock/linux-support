// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is efficient and compiles down to the same assembler as `_mm256_blend_epi32()` but with a different immediate value used for the control word.
///
/// `imm8` is a control value between `0` and `15` inclusive; if a bit is set, then the equivalent lane from `b` is copied otherwise the lane from `a` is copied.
#[inline(always)]
pub unsafe fn _mm256_blend_epi64(a: __m256i, b: __m256i, imm8: i32) -> __m256i
{
	debug_assert!(imm8 >= 0, "imm8 must be zero or positive, not `{}`", imm8);
	debug_assert!(imm8 <= 0b1111, "imm8 must be less than or equal to 15, not `{}`", imm8);
	
	macro_rules! _mm256_blend_epi64_const
	{
		($imm8: expr) =>
		{
			_mm256_castpd_si256(_mm256_blend_pd(_mm256_castsi256_pd(a), _mm256_castsi256_pd(b), $imm8))
		}
	}
	
	match imm8
	{
		0 => _mm256_blend_epi64_const!(0),
		1 => _mm256_blend_epi64_const!(1),
		2 => _mm256_blend_epi64_const!(2),
		3 => _mm256_blend_epi64_const!(3),
		4 => _mm256_blend_epi64_const!(4),
		5 => _mm256_blend_epi64_const!(5),
		6 => _mm256_blend_epi64_const!(6),
		7 => _mm256_blend_epi64_const!(7),
		8 => _mm256_blend_epi64_const!(8),
		9 => _mm256_blend_epi64_const!(9),
		10 => _mm256_blend_epi64_const!(10),
		11 => _mm256_blend_epi64_const!(11),
		12 => _mm256_blend_epi64_const!(12),
		13 => _mm256_blend_epi64_const!(13),
		14 => _mm256_blend_epi64_const!(14),
		15 => _mm256_blend_epi64_const!(15),
		_ => unreachable_code_const("Invalid value"),
	}
	
	
}

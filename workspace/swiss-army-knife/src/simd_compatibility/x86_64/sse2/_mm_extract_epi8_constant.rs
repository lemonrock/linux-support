// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is inefficient and should be avoided inside loops.
#[inline(always)]
pub unsafe fn _mm_extract_epi8_constant<const imm8: i32>(a: __m128i) -> i8
{
	macro_rules! _mm_extract_epi16_compiler_work_around
	{
		($imm8: expr) =>
		{
			{
				const imm8_by_2: i32 = $imm8 / 2;
				_mm_extract_epi16::<imm8_by_2>(a)
			}
		}
	}
	
	let value = match imm8
	{
		0 => _mm_extract_epi16_compiler_work_around!(0),
		1 => _mm_extract_epi16_compiler_work_around!(1),
		2 => _mm_extract_epi16_compiler_work_around!(2),
		3 => _mm_extract_epi16_compiler_work_around!(3),
		4 => _mm_extract_epi16_compiler_work_around!(4),
		5 => _mm_extract_epi16_compiler_work_around!(5),
		6 => _mm_extract_epi16_compiler_work_around!(6),
		7 => _mm_extract_epi16_compiler_work_around!(7),
		_ => unreachable_code_const("Only values 0 to 7 inclusive are supported")
	};
	
	let value_u16 = value as u16;
	let shifted = (value_u16 >> ((imm8 % 2) * 8)) as u8;
	shifted as i8
}

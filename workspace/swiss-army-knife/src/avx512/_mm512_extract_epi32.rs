// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F but present in similar form in AVX.
///
/// Inefficient.
#[inline(always)]
pub unsafe fn _mm512_extract_epi32(a: __m512i, index: i32) -> i32
{
	macro_rules! _mm512_extract_epi32_constified_index
	{
		($a: ident, $index: expr) =>
		{
			{
				let a = $a;
				const index: i32 = $index;
				_mm512_alignr_epi32(a, a, index)
			}
		}
	}
	
	let x = match index & 0b1111
	{
		0 => _mm512_extract_epi32_constified_index!(a, 0),
		1 => _mm512_extract_epi32_constified_index!(a, 1),
		2 => _mm512_extract_epi32_constified_index!(a, 2),
		3 => _mm512_extract_epi32_constified_index!(a, 3),
		4 => _mm512_extract_epi32_constified_index!(a, 4),
		5 => _mm512_extract_epi32_constified_index!(a, 5),
		6 => _mm512_extract_epi32_constified_index!(a, 6),
		7 => _mm512_extract_epi32_constified_index!(a, 7),
		8 => _mm512_extract_epi32_constified_index!(a, 8),
		9 => _mm512_extract_epi32_constified_index!(a, 9),
		10 => _mm512_extract_epi32_constified_index!(a, 10),
		11 => _mm512_extract_epi32_constified_index!(a, 11),
		12 => _mm512_extract_epi32_constified_index!(a, 12),
		13 => _mm512_extract_epi32_constified_index!(a, 13),
		14 => _mm512_extract_epi32_constified_index!(a, 14),
		15 => _mm512_extract_epi32_constified_index!(a, 15),
		_ => unreachable_code_const("Masked lower 4 bits before match"),
	};
	
	_mm512_cvtsi512_si32(x)
}

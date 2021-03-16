// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F but present in similar form in AVX.
///
/// Inefficient.
///
/// Even though the return type is `i32`, the value fits in an `u16`.
#[inline(always)]
pub unsafe fn _mm512_extract_epi16<const index: i32>(a: __m512i) -> i32
{
	macro_rules! _mm512_extract_epi32_const_index
	{
		($index_divided_by_2: expr) =>
		{
			_mm512_extract_epi32::<$index_divided_by_2>(a)
		}
	}
	
	// Rust does not allow:-
	// ```
	// const Epi32Index: i32 = index / 2;
	// _mm512_extract_epi32::<Epi32Index>(a)
	// ```
	//
	// Hence this horror.
	let result = match index
	{
		0 ..= 1 => _mm512_extract_epi32_const_index!(0),
		2 ..= 3 => _mm512_extract_epi32_const_index!(1),
		4 ..= 5 => _mm512_extract_epi32_const_index!(2),
		6 ..= 7 => _mm512_extract_epi32_const_index!(3),
		8 ..= 9 => _mm512_extract_epi32_const_index!(4),
		10 ..= 11 => _mm512_extract_epi32_const_index!(5),
		12 ..= 13 => _mm512_extract_epi32_const_index!(6),
		14 ..= 15 => _mm512_extract_epi32_const_index!(7),
		16 ..= 17 => _mm512_extract_epi32_const_index!(8),
		18 ..= 19 => _mm512_extract_epi32_const_index!(9),
		20 ..= 21 => _mm512_extract_epi32_const_index!(10),
		22 ..= 23 => _mm512_extract_epi32_const_index!(11),
		24 ..= 25 => _mm512_extract_epi32_const_index!(12),
		26 ..= 27 => _mm512_extract_epi32_const_index!(13),
		28 ..= 29 => _mm512_extract_epi32_const_index!(14),
		30 ..= 31 => _mm512_extract_epi32_const_index!(15),
		_ => panic!("index {} not supported", index)
	};
	
	let shift = if (index % 2) == 0
	{
		0
	}
	else
	{
		16
	};
	
	(result >> shift) & 0xFFFF
}

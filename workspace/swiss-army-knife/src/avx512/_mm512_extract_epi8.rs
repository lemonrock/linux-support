// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Provides a function missing in AVX512F but present in similar form in AVX.
///
/// Inefficient.
///
/// Even though the return type is `i32`, the value fits in an `u8`.
/// `index` is useful between `0` and `63` inclusive.
#[inline(always)]
pub unsafe fn _mm512_extract_epi8<const index: i32>(a: __m512i) -> i32
{
	macro_rules! _mm512_extract_epi32_const_index
	{
		($index_divided_by_4: expr) =>
		{
			_mm512_extract_epi32::<$index_divided_by_4>(a)
		}
	}
	
	// Rust does not allow:-
	// ```
	// const Epi32Index: i32 = index / 4;
	// _mm512_extract_epi32::<Epi32Index>(a)
	// ```
	//
	// Hence this horror.
	let result = match index
	{
		0 ..= 3 => _mm512_extract_epi32_const_index!(0),
		4 ..= 7 => _mm512_extract_epi32_const_index!(1),
		8 ..= 11 => _mm512_extract_epi32_const_index!(2),
		12 ..= 15 => _mm512_extract_epi32_const_index!(3),
		16 ..= 19 => _mm512_extract_epi32_const_index!(4),
		20 ..= 23 => _mm512_extract_epi32_const_index!(5),
		24 ..= 27 => _mm512_extract_epi32_const_index!(6),
		28 ..= 31 => _mm512_extract_epi32_const_index!(7),
		32 ..= 35 => _mm512_extract_epi32_const_index!(8),
		36 ..= 39 => _mm512_extract_epi32_const_index!(9),
		40 ..= 43 => _mm512_extract_epi32_const_index!(10),
		44 ..= 47 => _mm512_extract_epi32_const_index!(11),
		48 ..= 51 => _mm512_extract_epi32_const_index!(12),
		52 ..= 55 => _mm512_extract_epi32_const_index!(13),
		56 ..= 59 => _mm512_extract_epi32_const_index!(14),
		60 ..= 63 => _mm512_extract_epi32_const_index!(15),
		_ => panic!("index {} not supported", index)
	};
	
	let shift = if (index % 4) == 0
	{
		0
	}
	else
	{
		16
	};
	
	(result >> shift) & 0xFF
}

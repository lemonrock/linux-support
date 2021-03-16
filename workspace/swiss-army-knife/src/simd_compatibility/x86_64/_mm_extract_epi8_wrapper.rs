// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wrapper function for `_mm_extract_epi8` to support constant and non-constant forms in Rust and our compatibility module.
#[cfg(target_feature = "sse4.1")]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_extract_epi8_wrapper<const imm8: i32>(a: std::arch::x86_64::__m128i) -> i32
{
	std::arch::x86_64::_mm_extract_epi8(a, imm8)
}

#[cfg(all(target_feature = "sse2", not(target_feature = "sse4.1")))]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_extract_epi8_wrapper<const imm8: i32>(a: std::arch::x86_64::__m128i) -> i32
{
	sse2::_mm_extract_epi8_constant::<imm8>(a)
}

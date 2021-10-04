// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



use super::*;
use crate::get_unchecked::GetUnchecked;
use cfg_if::cfg_if;
#[cfg(all(target_arch = "x86_64", target_feature = "ssse3"))] use std::arch::x86_64::__m128i as BytesVector128;
#[cfg(all(target_arch = "x86_64", target_feature = "ssse3"))] use std::arch::x86_64::_mm_set_epi8 as set_bytes_vector_128;
#[cfg(all(target_arch = "x86_64", target_feature = "ssse3"))] use std::arch::x86_64::_mm_shuffle_epi8 as shuffle_bytes_vector_128;
#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))] use std::arch::x86_64::__m256i as BytesVector256;
#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))] use std::arch::x86_64::_mm256_set_epi8 as set_bytes_vector_256;
#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))] use std::arch::x86_64::_mm256_shuffle_epi8 as shuffle_bytes_vector_256;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86_64::__m512i as BytesVector512;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86_64::_mm512_set_epi8 as set_bytes_vector_512;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86_64::_mm512_shuffle_epi8 as shuffle_bytes_vector_512;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86_64::_mm512_loadu_epi8 as load_unaligned_bytes_vector_512;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86_64::_mm512_storeu_epi8 as store_unaligned_bytes_vector_512;
#[cfg(all(target_arch = "x86", target_feature = "ssse3"))] use std::arch::x86::__m128i as BytesVector128;
#[cfg(all(target_arch = "x86", target_feature = "ssse3"))] use std::arch::x86::_mm_set_epi8 as set_bytes_vector_128;
#[cfg(all(target_arch = "x86", target_feature = "ssse3"))] use std::arch::x86::_mm_shuffle_epi8 as shuffle_bytes_vector_128;
#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))] use std::arch::x86::__m256i as BytesVector256;
#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))] use std::arch::x86::_mm256_set_epi8 as set_bytes_vector_256;
#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))] use std::arch::x86::_mm256_shuffle_epi8 as shuffle_bytes_vector_256;
#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86::__m512i as BytesVector512;
#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86::_mm512_set_epi8 as set_bytes_vector_512;
#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86::_mm512_shuffle_epi8 as shuffle_bytes_vector_512;
#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86::_mm512_loadu_epi8 as load_unaligned_bytes_vector_512;
#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))] use std::arch::x86::_mm512_storeu_epi8 as store_unaligned_bytes_vector_512;
// x86_64
cfg_if!
{
	// This is true for all Intel CPUs except the legacy Xeon Phi Knights Mill and Knights Landing.
	if #[cfg(all(target_arch = "x86_64", target_feature = "ssse3", target_feature = "avx512f", target_feature = "avx512bw", target_feature = "avx512vl"))]
	{
		use std::arch::x86_64::_mm_loadu_epi8 as load_unaligned_bytes_vector_128;
		use std::arch::x86_64::_mm_storeu_epi8 as store_unaligned_bytes_vector_128;
	}
	else if #[cfg(all(target_arch = "x86_64", target_feature = "ssse3"))]
	{
		use std::arch::x86_64::_mm_lddqu_si128 as load_unaligned_bytes_vector_128;
		use std::arch::x86_64::_mm_storeu_si128 as store_unaligned_bytes_vector_128;
	}
}
cfg_if!
{
	// This is true for all Intel CPUs except the legacy Xeon Phi Knights Mill and Knights Landing.
	if #[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2", target_feature = "avx512f", target_feature = "avx512bw", target_feature = "avx512vl"))]
	{
		use std::arch::x86_64::_mm256_loadu_epi8 as load_unaligned_bytes_vector_256;
		use std::arch::x86_64::_mm256_storeu_epi8 as store_unaligned_bytes_vector_256;
	}
	else if #[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
	{
		use std::arch::x86_64::_mm256_lddqu_si256 as load_unaligned_bytes_vector_256;
		use std::arch::x86_64::_mm256_storeu_si256 as store_unaligned_bytes_vector_256;
	}
}
// x86; some older Intel Core Solo and Atom Lincroft CPUs may implement SSSE3 in a 32-bit mode; the other AVX, etc combinations are solely for completeness.
cfg_if!
{
	// This is true for all Intel CPUs except the legacy Xeon Phi Knights Mill and Knights Landing.
	if #[cfg(all(target_arch = "x86", target_feature = "ssse3", target_feature = "avx512f", target_feature = "avx512bw", target_feature = "avx512vl"))]
	{
		use std::arch::x86::_mm_loadu_epi8 as load_unaligned_bytes_vector_128;
		use std::arch::x86::_mm_storeu_epi8 as store_unaligned_bytes_vector_128;
	}
	else if #[cfg(all(target_arch = "x86", target_feature = "ssse3"))]
	{
		use std::arch::x86::_mm_lddqu_si128 as load_unaligned_bytes_vector_128;
		use std::arch::x86::_mm_storeu_si128 as store_unaligned_bytes_vector_128;
	}
}
cfg_if!
{
	// This is true for all Intel CPUs except the legacy Xeon Phi Knights Mill and Knights Landing.
	if #[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2", target_feature = "avx512f", target_feature = "avx512bw", target_feature = "avx512vl"))]
	{
		use std::arch::x86::_mm256_loadu_epi8 as load_unaligned_bytes_vector_256;
		use std::arch::x86::_mm256_storeu_epi8 as store_unaligned_bytes_vector_256;
	}
	else if #[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
	{
		use std::arch::x86::_mm256_lddqu_si256 as load_unaligned_bytes_vector_256;
		use std::arch::x86::_mm256_storeu_si256 as store_unaligned_bytes_vector_256;
	}
}


include!("BytesVector.rs");
include!("ByteSwapUnalignedMemory.rs");
include!("BytesVector128Size.rs");
include!("BytesVector256Size.rs");
include!("BytesVector512Size.rs");
include!("load_movbe_16.rs");
include!("load_movbe_32.rs");
include!("load_movbe_64.rs");
include!("Unaligned.rs");
include!("Unaligned16.rs");
include!("Unaligned32.rs");
include!("Unaligned64.rs");

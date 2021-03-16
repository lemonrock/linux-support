// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::arch::x86_64::_mm256_add_epi32;
use std::arch::x86_64::__m512i;
use std::arch::x86_64::_mm512_castsi512_si256;
use std::arch::x86_64::_mm512_extracti64x4_epi64;
#[cfg(target_feature = "avx512bw")] use std::arch::x86_64::__mmask64;
#[cfg(target_feature = "avx512dq")] use std::arch::x86_64::__mmask8;
use std::arch::x86_64::_mm512_alignr_epi32;
#[cfg(target_feature = "avx512bw")] use std::arch::x86_64::_mm512_mask_set1_epi16;
use std::arch::x86_64::_mm512_mask_set1_epi32;
use std::arch::x86_64::_mm512_mask_set1_epi64;
use std::arch::x86_64::_mm512_reduce_add_epi64;
#[cfg(target_feature = "avx512bw")] use std::arch::x86_64::_mm512_sad_epu8;
use std::arch::x86_64::_mm512_setzero_si512;
#[cfg(target_feature = "avx512dq")] use std::mem::transmute;
use crate::get_unchecked::GetUnchecked;
#[cfg(all(target_feature = "avx2", target_feature = "sse2"))] use super::avx2::_mm256_reduce_add_epu32;


include!("__v16si.rs");
include!("_kshiftli_mask8.rs");
include!("_kshiftri_mask8.rs");
include!("_ktestc_mask8_u8.rs");
include!("_ktestz_mask8_u8.rs");
include!("_load_mask8.rs");
include!("_mm512_cvtsi512_si32.rs");
include!("_mm512_extract_epi8.rs");
include!("_mm512_extract_epi16.rs");
include!("_mm512_extract_epi32.rs");
include!("_mm512_insert_epi8.rs");
include!("_mm512_insert_epi16.rs");
include!("_mm512_insert_epi32.rs");
include!("_mm512_insert_epi64.rs");
include!("_mm512_maskz_popcnt_epi8_load_unaligned.rs");
include!("_mm512_popcnt_epi8_load_unaligned.rs");
include!("_mm512_popcnt_epi16_load_unaligned.rs");
include!("_mm512_popcnt_epi32_load_unaligned.rs");
include!("_mm512_popcnt_epi64_load_unaligned.rs");
include!("_mm512_reduce_add_epu8.rs");
include!("_mm512_reduce_add_epu32.rs");

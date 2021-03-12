// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[cfg(target_feature = "sse2")] use crate::sse2::_mm_reduce_add_epu32;
#[cfg(target_feature = "sse2")] use crate::sse2::_mm_reduce_add_epu8;
#[cfg(target_feature = "sse2")] use std::arch::x86_64::_mm_add_epi32;
#[cfg(target_feature = "sse2")] use std::arch::x86_64::_mm_add_epi64;
use crate::unreachable_code_const;
use std::arch::x86_64::__m256i;
use std::arch::x86_64::_mm256_add_epi64;
use std::arch::x86_64::_mm256_add_epi8;
use std::arch::x86_64::_mm256_and_si256;
use std::arch::x86_64::_mm256_blend_pd;
use std::arch::x86_64::_mm256_castpd_si256;
use std::arch::x86_64::_mm256_castsi256_pd;
use std::arch::x86_64::_mm256_castsi256_ps;
use std::arch::x86_64::_mm256_castsi256_si128;
use std::arch::x86_64::_mm256_extracti128_si256;
use std::arch::x86_64::_mm256_movemask_pd;
use std::arch::x86_64::_mm256_movemask_ps;
use std::arch::x86_64::_mm256_or_si256;
use std::arch::x86_64::_mm256_permute4x64_epi64;
use std::arch::x86_64::_mm256_sad_epu8;
use std::arch::x86_64::_mm256_set1_epi8;
use std::arch::x86_64::_mm256_setr_epi8;
use std::arch::x86_64::_mm256_setzero_si256;
use std::arch::x86_64::_mm256_shuffle_epi32;
use std::arch::x86_64::_mm256_shuffle_epi8;
use std::arch::x86_64::_mm256_slli_epi64;
use std::arch::x86_64::_mm256_srli_epi16;
use std::arch::x86_64::_mm256_srli_epi64;
use std::arch::x86_64::_mm_cvtsi128_si64;


include!("shuffle_and_blend_with_zeros.rs");


include!("_mm256_blend_epi64.rs");
include!("_mm256_lane_crossing_left_shift_upto_64_bits.rs");
include!("_mm256_lane_crossing_right_shift_upto_64_bits.rs");
include!("_mm256_movemask_epi32.rs");
include!("_mm256_movemask_epi64.rs");
include!("_mm256_popcnt_epi8.rs");
include!("_mm256_popcnt_epi64.rs");
include!("_mm256_reduce_add_epi64.rs");
include!("_mm256_reduce_add_epu8.rs");
include!("_mm256_reduce_add_epu32.rs");
include!("_mm256_reduce_add_epu64.rs");
include!("U64LaneIndex.rs");

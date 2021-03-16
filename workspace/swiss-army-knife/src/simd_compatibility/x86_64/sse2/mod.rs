// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::MaximumLaneCrossingShift;
use crate::unreachable_code_const;
use std::arch::x86_64::_MM_SHUFFLE;
use std::arch::x86_64::__m128i;
use std::arch::x86_64::_mm_add_epi32;
use std::arch::x86_64::_mm_add_epi8;
use std::arch::x86_64::_mm_and_si128;
use std::arch::x86_64::_mm_blend_pd;
use std::arch::x86_64::_mm_castpd_si128;
use std::arch::x86_64::_mm_castsi128_pd;
use std::arch::x86_64::_mm_cvtsi128_si32;
use std::arch::x86_64::_mm_cvtsi32_si128;
use std::arch::x86_64::_mm_extract_epi16;
use std::arch::x86_64::_mm_or_si128;
use std::arch::x86_64::_mm_sad_epu8;
use std::arch::x86_64::_mm_set1_epi8;
use std::arch::x86_64::_mm_setzero_si128;
use std::arch::x86_64::_mm_shuffle_epi32;
use std::arch::x86_64::_mm_shuffle_epi8;
use std::arch::x86_64::_mm_shuffle_pd;
use std::arch::x86_64::_mm_slli_epi64;
use std::arch::x86_64::_mm_srli_epi16;
use std::arch::x86_64::_mm_srli_epi64;
use std::arch::x86_64::_mm_sub_epi8;
use std::arch::x86_64::_mm_unpackhi_epi64;


include!("_mm_blend_epi64.rs");
include!("_mm_extract_epi8_constant.rs");
include!("_mm_extract_epi8_variable.rs");
include!("_mm_lane_crossing_left_shift_upto_64_bits.rs");
include!("_mm_lane_crossing_right_shift_upto_64_bits.rs");
include!("_mm_popcnt_epi8.rs");
include!("_mm_reduce_add_epu8.rs");
include!("_mm_reduce_add_epu32.rs");
include!("_mm_shuffle_epi64_constant.rs");

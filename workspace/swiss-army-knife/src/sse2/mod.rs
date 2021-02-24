// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::arch::x86_64::{_MM_SHUFFLE, _mm_sad_epu8, _mm_setzero_si128, _mm_extract_epi16};
use std::arch::x86_64::__m128i;
use std::arch::x86_64::_mm_add_epi32;
use std::arch::x86_64::_mm_cvtsi128_si32;
use std::arch::x86_64::_mm_shuffle_epi32;
use std::arch::x86_64::_mm_unpackhi_epi64;


include!("_mm_reduce_add_epu8.rs");
include!("_mm_reduce_add_epu32.rs");

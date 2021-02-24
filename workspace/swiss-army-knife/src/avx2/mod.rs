// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::arch::x86_64::__m256i;
#[cfg(target_feature = "sse2")] use std::arch::x86_64::_mm_add_epi32;
use std::arch::x86_64::_mm256_castsi256_si128;
use std::arch::x86_64::_mm256_extracti128_si256;
#[cfg(target_feature = "sse2")] use crate::sse2::_mm_reduce_add_epu8;
#[cfg(target_feature = "sse2")] use crate::sse2::_mm_reduce_add_epu32;


include!("_mm256_reduce_add_epu8.rs");
include!("_mm256_reduce_add_epu32.rs");

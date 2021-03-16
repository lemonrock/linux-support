// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::arch::x86_64::__m128i;
use std::arch::x86_64::_mm_add_epi8;
use std::arch::x86_64::_mm_and_si128;
use std::arch::x86_64::_mm_set1_epi8;
use std::arch::x86_64::_mm_setr_epi8;
use std::arch::x86_64::_mm_shuffle_epi8;
use std::arch::x86_64::_mm_srli_epi16;


include!("_mm_popcnt_epi8.rs");

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[cfg(all(target_feature = "avx512f", target_feature = "avx512bitalg", target_feature = "avx512vl"))] pub use std::arch::x86_64::_mm256_popcnt_epi8;
#[cfg(all(target_feature = "avx2", not(all(target_feature = "avx512f", target_feature = "avx512bitalg", target_feature = "avx512vl"))))] pub use avx2::_mm256_popcnt_epi8;

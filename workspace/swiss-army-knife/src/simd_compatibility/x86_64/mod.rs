// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// AVX2 support, including functions one might assume to be present as intrinsics provided by Intel but aren't.
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub mod avx2;

/// AVX512 support, including functions one might assume to be present as intrinsics provided by Intel but aren't.
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
pub mod avx512;


/// SSE2 support, including functions one might assume to be present as intrinsics provided by Intel but aren't.
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub mod sse2;


/// SSSE3 support, including functions one might assume to be present as intrinsics provided by Intel but aren't.
#[cfg(all(target_arch = "x86_64", target_feature = "ssse3"))]
pub mod ssse3;


include!("_mm256_popcnt_epi8.rs");
include!("_mm_extract_epi8_wrapper.rs");
include!("_mm_popcnt_epi8.rs");
include!("BitsPerU64Lane.rs");
include!("MaximumLaneCrossingShift.rs");

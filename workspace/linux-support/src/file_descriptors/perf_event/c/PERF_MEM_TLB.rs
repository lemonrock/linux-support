// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_NA: u32 = 1;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_HIT: u32 = 2;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_MISS: u32 = 4;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_L1: u32 = 8;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_L2: u32 = 16;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_WK: u32 = 32;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_OS: u32 = 64;
#[allow(dead_code)] pub(crate) const PERF_MEM_TLB_SHIFT: u32 = 26;

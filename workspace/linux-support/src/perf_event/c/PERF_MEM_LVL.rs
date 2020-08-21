// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_NA: u32 = 1;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_HIT: u32 = 2;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_MISS: u32 = 4;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_L1: u32 = 8;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_LFB: u32 = 16;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_L2: u32 = 32;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_L3: u32 = 64;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_LOC_RAM: u32 = 128;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_REM_RAM1: u32 = 256;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_REM_RAM2: u32 = 512;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_REM_CCE1: u32 = 1024;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_REM_CCE2: u32 = 2048;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_IO: u32 = 4096;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_UNC: u32 = 8192;
#[allow(dead_code)] pub(crate) const PERF_MEM_LVL_SHIFT: u32 = 5;

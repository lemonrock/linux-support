// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const PERF_MEM_LVL_NA: u32 = 1;
pub(crate) const PERF_MEM_LVL_HIT: u32 = 2;
pub(crate) const PERF_MEM_LVL_MISS: u32 = 4;
pub(crate) const PERF_MEM_LVL_L1: u32 = 8;
pub(crate) const PERF_MEM_LVL_LFB: u32 = 16;
pub(crate) const PERF_MEM_LVL_L2: u32 = 32;
pub(crate) const PERF_MEM_LVL_L3: u32 = 64;
pub(crate) const PERF_MEM_LVL_LOC_RAM: u32 = 128;
pub(crate) const PERF_MEM_LVL_REM_RAM1: u32 = 256;
pub(crate) const PERF_MEM_LVL_REM_RAM2: u32 = 512;
pub(crate) const PERF_MEM_LVL_REM_CCE1: u32 = 1024;
pub(crate) const PERF_MEM_LVL_REM_CCE2: u32 = 2048;
pub(crate) const PERF_MEM_LVL_IO: u32 = 4096;
pub(crate) const PERF_MEM_LVL_UNC: u32 = 8192;
pub(crate) const PERF_MEM_LVL_SHIFT: u32 = 5;

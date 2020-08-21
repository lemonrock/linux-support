// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_CPUMODE_MASK: u32 = 7;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_CPUMODE_UNKNOWN: u32 = 0;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_KERNEL: u32 = 1;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_USER: u32 = 2;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_HYPERVISOR: u32 = 3;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_GUEST_KERNEL: u32 = 4;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_GUEST_USER: u32 = 5;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: u32 = 4096;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_MMAP_DATA: u32 = 8192;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_COMM_EXEC: u32 = 8192;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_FORK_EXEC: u32 = 8192;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_SWITCH_OUT: u32 = 8192;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_EXACT_IP: u32 = 16384;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u32 = 16384;
#[allow(dead_code)] pub(crate) const PERF_RECORD_MISC_EXT_RESERVED: u32 = 32768;

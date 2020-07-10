// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const PERF_RECORD_MISC_CPUMODE_MASK: u32 = 7;
pub(crate) const PERF_RECORD_MISC_CPUMODE_UNKNOWN: u32 = 0;
pub(crate) const PERF_RECORD_MISC_KERNEL: u32 = 1;
pub(crate) const PERF_RECORD_MISC_USER: u32 = 2;
pub(crate) const PERF_RECORD_MISC_HYPERVISOR: u32 = 3;
pub(crate) const PERF_RECORD_MISC_GUEST_KERNEL: u32 = 4;
pub(crate) const PERF_RECORD_MISC_GUEST_USER: u32 = 5;
pub(crate) const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: u32 = 4096;
pub(crate) const PERF_RECORD_MISC_MMAP_DATA: u32 = 8192;
pub(crate) const PERF_RECORD_MISC_COMM_EXEC: u32 = 8192;
pub(crate) const PERF_RECORD_MISC_FORK_EXEC: u32 = 8192;
pub(crate) const PERF_RECORD_MISC_SWITCH_OUT: u32 = 8192;
pub(crate) const PERF_RECORD_MISC_EXACT_IP: u32 = 16384;
pub(crate) const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u32 = 16384;
pub(crate) const PERF_RECORD_MISC_EXT_RESERVED: u32 = 32768;

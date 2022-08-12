// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum perf_hw_cache_id
{
	PERF_COUNT_HW_CACHE_L1D = 0,
	
	PERF_COUNT_HW_CACHE_L1I = 1,
	
	PERF_COUNT_HW_CACHE_LL = 2,
	
	PERF_COUNT_HW_CACHE_DTLB = 3,
	
	PERF_COUNT_HW_CACHE_ITLB = 4,
	
	PERF_COUNT_HW_CACHE_BPU = 5,
	
	PERF_COUNT_HW_CACHE_NODE = 6,
}

impl perf_hw_cache_id
{
	#[allow(dead_code)]
	pub(crate) const PERF_COUNT_HW_CACHE_MAX: u32 = 6;
}

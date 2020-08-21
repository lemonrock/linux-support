// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum perf_bpf_event_type
{
	PERF_BPF_EVENT_UNKNOWN = 0,
	PERF_BPF_EVENT_PROG_LOAD = 1,
	PERF_BPF_EVENT_PROG_UNLOAD = 2,
}

impl perf_bpf_event_type
{
	#[allow(dead_code)]
	pub(crate) const PERF_BPF_EVENT_MAX: u32 = 3;
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum perf_event_read_format
{
	PERF_FORMAT_TOTAL_TIME_ENABLED = 1,
	PERF_FORMAT_TOTAL_TIME_RUNNING = 2,
	PERF_FORMAT_ID = 4,
	PERF_FORMAT_GROUP = 8,
	PERF_FORMAT_MAX = 16,
}

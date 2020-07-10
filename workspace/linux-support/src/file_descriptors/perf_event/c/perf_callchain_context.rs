// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum perf_callchain_context
{
	PERF_CONTEXT_HV = 18446744073709551584,
	PERF_CONTEXT_KERNEL = 18446744073709551488,
	PERF_CONTEXT_USER = 18446744073709551104,
	PERF_CONTEXT_GUEST = 18446744073709549568,
	PERF_CONTEXT_GUEST_KERNEL = 18446744073709549440,
	PERF_CONTEXT_GUEST_USER = 18446744073709549056,
	PERF_CONTEXT_MAX = 18446744073709547521,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum _bindgen_ty_1
{
	PERF_BR_UNKNOWN = 0,
	PERF_BR_COND = 1,
	PERF_BR_UNCOND = 2,
	PERF_BR_IND = 3,
	PERF_BR_CALL = 4,
	PERF_BR_IND_CALL = 5,
	PERF_BR_RET = 6,
	PERF_BR_SYSCALL = 7,
	PERF_BR_SYSRET = 8,
	PERF_BR_COND_CALL = 9,
	PERF_BR_COND_RET = 10,
}

impl _bindgen_ty_1
{
	#[allow(dead_code)]
	pub(crate) const PERF_BR_MAX: u32 = 1;
}

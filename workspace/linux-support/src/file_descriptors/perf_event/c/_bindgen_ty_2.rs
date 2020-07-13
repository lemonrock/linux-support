// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum _bindgen_ty_2
{
	PERF_TXN_ELISION = 1,
	PERF_TXN_TRANSACTION = 2,
	PERF_TXN_SYNC = 4,
	PERF_TXN_ASYNC = 8,
	PERF_TXN_RETRY = 16,
	PERF_TXN_CONFLICT = 32,
	PERF_TXN_CAPACITY_WRITE = 64,
	PERF_TXN_CAPACITY_READ = 128,
}

impl _bindgen_ty_2
{
	#[allow(dead_code)]
	pub(crate) const PERF_TXN_MAX: u64 = 256;

	#[allow(dead_code)]
	pub(crate) const PERF_TXN_ABORT_MASK: u64 = 18446744069414584320;
}

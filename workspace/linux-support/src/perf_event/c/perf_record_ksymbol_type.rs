// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum perf_record_ksymbol_type
{
	PERF_RECORD_KSYMBOL_TYPE_UNKNOWN = 0,
	PERF_RECORD_KSYMBOL_TYPE_BPF = 1,
}

impl perf_record_ksymbol_type
{
	#[allow(dead_code)]
	pub(crate) const PERF_RECORD_KSYMBOL_TYPE_MAX: u32 = 2;
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct __kernel_timespec
{
	/// Seconds.
	tv_sec: __kernel_time64_t,

	/// Nanoseconds (but 1 less than the value in a second).
	tv_nsec: c_longlong,
}

impl From<Duration> for __kernel_timespec
{
	fn from(value: Duration) -> Self
	{
		Self
		{
			tv_sec: value.as_secs() as i64,
			tv_nsec: value.subsec_nanos() as i64,
		}
	}
}

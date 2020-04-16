// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How to update a timestamp.
///
/// Defaults to `DoNotChange`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimestampUpdate
{
	/// Do not change.
	DoNotChange,

	/// Now.
	Now,

	/// At a specific time.
	At
	{
		/// Can not exceed `std::i64::MAX as u64`.
		absolute_seconds_since_unix_epoch: u64,

		/// Relative to `absolute_seconds_since_unix_epoch`.
		/// Must be less than 1,000,000,000.
		relative_nanoseconds: u32,
	},
}

impl Default for TimestampUpdate
{
	#[inline(always)]
	fn default() -> Self
	{
		TimestampUpdate::DoNotChange
	}
}

impl TimestampUpdate
{
	#[allow(deprecated)]
	#[inline(always)]
	fn to_timespec(&self) -> timespec
	{
		use self::TimestampUpdate::*;

		match self
		{
			&DoNotChange => timespec { tv_sec: unsafe { uninitialized() }, tv_nsec: UTIME_OMIT },
			&Now => timespec { tv_sec: unsafe { uninitialized() }, tv_nsec: UTIME_NOW },
			&At { absolute_seconds_since_unix_epoch, relative_nanoseconds } => timespec { tv_sec: absolute_seconds_since_unix_epoch.try_into().expect("We can not support times greater than std::i64::MAX"), tv_nsec: relative_nanoseconds as i64 },
		}
	}
}

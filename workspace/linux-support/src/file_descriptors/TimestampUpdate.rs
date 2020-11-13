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
	///
	/// Construct using `TimestampUpdate::from(SystemTime)`.
	At
	{
		/// Can not exceed `i64::MAX as u64`.
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

impl From<SystemTime> for TimestampUpdate
{
	#[inline(always)]
	fn from(value: SystemTime) -> Self
	{
		let duration_since_the_unix_epoch = value.duration_since(UNIX_EPOCH).unwrap();
		TimestampUpdate::At
		{
			absolute_seconds_since_unix_epoch: duration_since_the_unix_epoch.as_secs(),
			relative_nanoseconds: duration_since_the_unix_epoch.subsec_nanos(),
		}
	}
}

impl From<Option<SystemTime>> for TimestampUpdate
{
	#[inline(always)]
	fn from(value: Option<SystemTime>) -> Self
	{
		match value
		{
			None => TimestampUpdate::DoNotChange,
			Some(value) => Self::from(value)
		}
	}
}

impl Into<Option<SystemTime>> for TimestampUpdate
{
	#[inline(always)]
	fn into(self) -> Option<SystemTime>
	{
		use self::TimestampUpdate::*;

		match self
		{
			DoNotChange => None,
			Now => Some(SystemTime::now()),
			At { absolute_seconds_since_unix_epoch, relative_nanoseconds } => Some(UNIX_EPOCH + Duration::new(absolute_seconds_since_unix_epoch, relative_nanoseconds)),
		}
	}
}

impl TimestampUpdate
{
	/// Converts `TimestampUpdate::Now` to `TimestampUpdate::At`.
	#[inline(always)]
	pub fn resolve_now(&mut self)
	{
		if unlikely!(*self != TimestampUpdate::Now)
		{
			return
		}

		*self = SystemTime::now().into()
	}

	#[inline(always)]
	fn to_timespec(&self) -> timespec
	{
		use self::TimestampUpdate::*;

		match self
		{
			&DoNotChange => timespec { tv_sec: unsafe_uninitialized(), tv_nsec: UTIME_OMIT },
			&Now => timespec { tv_sec: unsafe_uninitialized(), tv_nsec: UTIME_NOW },
			&At { absolute_seconds_since_unix_epoch, relative_nanoseconds } => timespec { tv_sec: absolute_seconds_since_unix_epoch.try_into().expect("We can not support times greater than i64::MAX"), tv_nsec: relative_nanoseconds as i64 },
		}
	}
}

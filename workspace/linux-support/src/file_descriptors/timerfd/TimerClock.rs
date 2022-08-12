// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Choice of clock for a timer file descriptor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum TimerClock
{
	/// A settable system-wide real-time clock.
	RealTime = CLOCK_REALTIME,

	/// A unsettable monotonically increasing clock that measures time from some unspecified point in the past that does not change after system startup.
	Monotonic = CLOCK_MONOTONIC,

	/// Like `Monotonic`, this is a monotonically increasing clock.
	///
	/// However, whereas the `Monotonic` clock does not measure the time while a system is suspended, the `BootTime` clock does include the time during which the system is suspended.
	/// This is useful for applications that need to be suspend-aware.
	/// `RealTime` is not suitable for such applications, since that clock is affected by discontinuous changes to the system clock.
	///
	/// Since Linux 3.15.
	BootTime = CLOCK_BOOTTIME,

	/// This clock is like `RealTime`, but will wake the system if it is suspended.
	///
	/// The caller must have the `CAP_WAKE_ALARM` capability in order to set a timer against this clock.
	///
	/// Since Linux 3.11.
	RealTimeAlarm = CLOCK_REALTIME_ALARM,

	/// This clock is like `BootTime`, but will wake the system if it is suspended.
	///
	/// The caller must have the `CAP_WAKE_ALARM` capability in order to set a timer against this clock.
	///
	/// Since Linux 3.11.
	BootTimeAlarm = CLOCK_BOOTTIME_ALARM,
}

impl Default for TimerClock
{
	#[inline(always)]
	fn default() -> Self
	{
		TimerClock::Monotonic
	}
}

// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// The clock `clockid` is one of the constants `CLOCK_REALTIME`, `CLOCK_MONOTONIC`, `CLOCK_BOOTTIME`, `CLOCK_REALTIME_ALARM` or `CLOCK_BOOTTIME_ALARM`.
	///
	/// The current value of the clock `clockid` can be retrieved using `clock_gettime()`.
	///
	/// `flags` is either zero or a bitwise or of the constants starting `TFD_*`.
	///
	/// On success,` timerfd_create()` returns a new file descriptor (it is not known if this can be negative).
	/// On error, `-1` is returned and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EINVAL`: The `clockid` argument is neither `CLOCK_MONOTONIC` nor `CLOCK_REALTIME` (?conflicts with earlier documentation)?
	/// * `EINVAL`: `flags` is invalid; or, in Linux 2.6.26 or earlier, `flags` is nonzero.
	/// * `EMFILE`: The per-process limit on the number of open file descriptors has been reached.
	/// * `ENFILE`: The system-wide limit on the total number of open files has been reached.
	/// * `ENODEV`: Could not mount (internal) anonymous inode device.
	/// * `ENOMEM`: There was insufficient kernel memory to create the timer.
	pub(crate) fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
}

/// A settable system-wide real-time clock.
pub(crate) const CLOCK_REALTIME: c_int = 0;

/// A nonsettable monotonically increasing clock that measures time from some unspecified point in the past that does not change after system startup.
pub(crate) const CLOCK_MONOTONIC: c_int = 1;

/// Like `CLOCK_MONOTONIC`, this is a monotonically increasing clock.
///
/// However, whereas the `CLOCK_MONOTONIC` clock does not measure the time while a system is suspended, the `CLOCK_BOOTTIME` clock does include the time during which the system is suspended.
/// This is useful for applications that need to be suspend-aware.
/// `CLOCK_REALTIME` is not suitable for such applications, since that clock is affected by discontinuous changes to the system clock.
///
/// Since Linux 3.15.
pub(crate) const CLOCK_BOOTTIME: c_int = 7;

/// This clock is like `CLOCK_REALTIME`, but will wake the system if it is suspended.
///
/// The caller must have the `CAP_WAKE_ALARM` capability in order to set a timer against this clock.
///
/// Since Linux 3.11.
pub(crate) const CLOCK_REALTIME_ALARM: c_int = 8;

/// This clock is like `CLOCK_BOOTTIME`, but will wake the system if it is suspended.
///
/// The caller must have the `CAP_WAKE_ALARM` capability in order to set a timer against this clock.
///
/// Since Linux 3.11.
pub(crate) const CLOCK_BOOTTIME_ALARM: c_int = 9;

/// Sets the `O_NONBLOCK` file status flag on the newly opened timerfd file description.
///
/// Since Linux 2.6.27.
pub(crate) const TFD_NONBLOCK: c_int = O_NONBLOCK;

/// Set the close-on-exec (`FD_CLOEXEC`) flag on the newly opened timerfd file description.
///
/// Since Linux 2.6.27.
pub(crate) const TFD_CLOEXEC: c_int = O_CLOEXEC;


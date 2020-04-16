// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `timerfd_settime()` arms (starts) or disarms (stops) the timer referred to by the file descriptor `fd`.
	///
	/// The new_value argument specifies the initial expiration and interval for the timer.
	/// The `itimerspec` structure used for this argument contains two fields, each of which is in turn a structure of type `timespec`.
	///
	/// `new_value.it_value` specifies the initial expiration of the timer, in seconds and nanoseconds.
	///
	/// * Setting either field of `new_value.it_value` to a nonzero value arms the timer.
	/// * Setting both fields of `new_value.it_value` to zero disarms the timer.
	///
	/// Setting one or both fields of `new_value.it_interval` to nonzero values specifies the period, in seconds and nanoseconds, for repeated timer expirations after the initial expiration.
	/// If both fields of `new_value.it_interval` are zero, the timer expires just once, at the time specified by `new_value.it_value`.
	///
	/// By default, the initial expiration time specified in new_value is interpreted relative to the current time on the timer's clock at the time of the call (ie, `new_value.it_value` specifies a time relative to the current value of the clock specified by `clockid`).
	/// An absolute timeout can be selected via the `flags` argument.
	///
	/// The `flags` argument is a bit mask that can include the flags `TFD_TIMER_ABSTIME` and `TFD_TIMER_CANCEL_ON_SET`.
	/// On success, `timerfd_settime()` returns `0`.
	/// On error, `-1` is returned and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	/// * `EBADF`: `fd` is not a valid file descriptor.
	/// * `EFAULT`: `new_value` or `old_value` is not a valid pointer.
	/// * `EINVAL`: `fd` is not a valid timerfd file descriptor.
	/// * `EINVAL`: `new_value` is not properly initialized (one of the fields of type `tv_nsec` falls outside the range zero to 999,999,999).
	/// * `EINVAL`: `flags` is invalid.
	pub(crate) fn timerfd_settime(fd: RawFd, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;
}


/// Interpret `new_value.it_value` as an absolute value on the timer's clock.
///
/// The timer will expire when the value of the timer's clock reaches the value specified in `new_value.it_value`.
pub(crate) const TFD_TIMER_ABSTIME: c_int = 1;

/// If this flag is specified along with `TFD_TIMER_ABSTIME` and the clock for this timer is `CLOCK_REALTIME` or `CLOCK_REALTIME_ALARM`, then mark this timer as cancelable if the realtime clock undergoes a discontinuous change (`settimeofday()`, `clock_settime()`, or similar).
///
/// When such changes occur, a current or future `read()` from the file descriptor will fail with the error `ECANCELED` (sic).
pub(crate) const TFD_TIMER_CANCEL_ON_SET: c_int = 1 << 1;

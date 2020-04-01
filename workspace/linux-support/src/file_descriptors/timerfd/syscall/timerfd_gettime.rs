// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `timerfd_gettime()` returns, in `curr_value`, an `itimerspec` structure that contains the current setting of the timer referred to by the file descriptor `fd`.
 	///
	/// The `it_value` field returns the amount of time until the timer will next expire.
	/// If both fields of this structure are zero, then the timer is currently disarmed.
	/// This field always contains a relative value, regardless of whether the `TFD_TIMER_ABSTIME` flag was specified when setting the timer.
	///
	/// The `it_interval` field returns the interval of the timer.
	/// If both fields of this structure are zero, then the timer is set to expire just once, at the time specified by `curr_value.it_value`.
	///
	/// On success, `timerfd_gettime()` returns `0`.
	/// On error, `-1` is returned and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	/// * `EBADF`: `fd` is not a valid file descriptor.
	/// * `EFAULT`: `curr_value` is not a valid pointer.
	/// * `EINVAL`: `fd` is not a valid timerfd file descriptor.
	pub(crate) fn timerfd_gettime(fd: RawFd, curr_value: *mut itimerspec) -> c_int;
}

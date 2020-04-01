// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// The `epoll_wait() `call waits for events on the `epoll` instance referred to by the file descriptor `epfd`.
	/// The memory area pointed to by `events` will contain the events that will be available for the caller.
	///
	/// Up to `maxevents` are returned by `epoll_wait()`.
	///
	/// The `maxevents` argument must be greater than zero.
	///
	/// The `timeout` argument specifies the number of milliseconds that `epoll_wait()` will block.
	/// Time is measured against the `CLOCK_MONOTONIC` clock.
	/// The call will block until either:-
	/// * a file descriptor delivers an event;
	/// * the call is interrupted by a signal handler; or
	/// * the timeout expires
	///
	/// Note that the timeout interval will be rounded up to the system clock granularity, and kernel scheduling delays mean that the blocking interval may overrun by a small amount.
	/// Specifying a `timeout` of `-1` causes `epoll_wait()` to block indefinitely, while specifying a `timeout` equal to zero cause `epoll_wait()` to return immediately, even if no events are available.
	///
	/// When successful, `epoll_wait()` returns the number of file descriptors ready for the requested I/O, or zero if no file descriptor became ready during the requested `timeout` milliseconds.
	/// When an error occurs, `epoll_wait()` returns `-1` and `errno` is set appropriately.
	///
	/// Errors documented to be returned from `epoll_wait()` in `errno`:-
	///
	/// * `EBADF`: `epfd` is not a valid file descriptor.
	/// * `EFAULT`: The memory area pointed to by events is not accessible with write permissions.
	/// * `EINTR`: The call was interrupted by a signal handler before either (1) any of the requested events occurred or (2) the `timeout` expired.
	/// * `EINVAL`: `epfd` is not an epoll file descriptor, or `maxevents` is less than or equal to zero.
	pub(crate) fn epoll_wait(epfd: RawFd, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
}

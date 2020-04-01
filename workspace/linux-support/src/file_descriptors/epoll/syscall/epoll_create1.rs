// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// This is a flag value to pass to `epoll_create1()`.
///
/// Sets the close-on-exec (`FD_CLOEXEC`) flag on the new file descriptor.
///
/// Valid on Linux.
/// Valid on Solaris.
pub(crate) const EPOLL_CLOEXEC: c_int = O_CLOEXEC;

#[link(name = "c")]
extern "C"
{
	/// Modern version of epoll suitable for all modern versions of Linux.
	///
	/// Flags can be either `0` or `EPOLL_CLOEXEC`.
	///
	/// On success, these system calls return a nonnegative file descriptor.
	//  On error, -1 is returned, and `errno` is set to indicate the error.
	///
	/// Errors documented to be returned from `epoll_ctl()` in `errno`:-
	///
	/// * `EINVAL`: Invalid value specified in flags.
	/// * `EMFILE`: The per-user limit on the number of epoll instances imposed by `/proc/sys/fs/epoll/max_user_instances` was encountered.
	/// * `EMFILE`: The per-process limit on the number of open file descriptors has been reached.
	/// * `ENFILE`: The system-wide limit on the total number of open files has been reached.
	/// * `ENOMEM`: There was insufficient memory to create the kernel object.
	pub(crate) fn epoll_create1(flags: c_int) -> c_int;
}

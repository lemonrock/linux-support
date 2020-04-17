// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `flags` is either zero or a bitwise or of `IN_CLOEXEC` or `IN_NONBLOCK`.
	///
	/// As its return value, `inotify_init1()` returns a new file descriptor that can be used to refer to the inotify object.
	/// On error, -1 is returned and errno is set to indicate the error.
	///
	/// Documented errors are:-
	///
	/// * `EINVAL`: An unsupported value was specified in `flags`.
	/// * `EMFILE`: The per-process limit on the number of open file descriptors has been reached.
	/// * `ENFILE`: The system-wide limit on the total number of open files has been reached.
	/// * `ENOMEM`: There was insufficient kernel memory to create the inotify data structures.
	///
	/// Since Linux 2.6.27.
	pub(crate) fn inotify_init1(flags: c_int) -> c_int;
}

/// Set the close-on-exec (`FD_CLOEXEC`) flag on the new file descriptor.
///
/// Since Linux 2.6.27.
pub(crate) const IN_CLOEXEC: c_int = O_CLOEXEC;

/// Set the `O_NONBLOCK` file status flag on the new file descriptor.
///
/// Since Linux 2.6.27.
pub(crate) const IN_NONBLOCK: c_int = O_NONBLOCK;

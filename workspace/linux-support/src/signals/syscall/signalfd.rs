// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `signalfd()` creates a file descriptor that can be used to accept signals targeted at the caller.
	///
	/// The `mask` argument specifies the set of signals that the caller wishes to accept via the file descriptor.
	/// This argument is a signal set.
	///
	/// The `flags` argument is either zero or a bitmask or of `SFD_NONBLOCK` or `SFD_CLOEXEC`.
	///
	/// Normally, the set of signals to be received via the file descriptor should be blocked using `sigprocmask()`, to prevent the signals being handled according to their default dispositions.
	///
	/// It is not possible to receive `SIGKILL` or `SIGSTOP` signals via a signalfd file descriptor; these signals are silently ignored if specified in `mask`.
	///
	/// If the `fd` argument is `-1`, then the call creates a new file descriptor and associates the signal set specified in `mask` with that file descriptor.
	///
	/// If fd is not `-1`, then it must specify a valid existing signalfd file descriptor, and `mask` is used to replace the signal set associated with that file descriptor.
	///
	/// On success, `signalfd()` returns a signalfd file descriptor; this is either a new file descriptor (if `fd` was `-1`), or `fd` if `fd` was a valid signalfd file descriptor.
	/// On error, `-1` is returned and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EBADF`: The `fd` file descriptor is not a valid file descriptor.
	/// * `EINVAL`: `fd` is not a valid signalfd file descriptor or `flags` is invalid; or, in Linux 2.6.26 or earlier, `flags` is nonzero.
	/// * `EMFILE`: The per-process limit on the number of open file descriptors has been reached.
	/// * `ENFILE`: The system-wide limit on the total number of open files has been reached.
	/// * `ENODEV`: Could not mount (internal) anonymous inode device.
	/// * `ENOMEM`: There was insufficient kernel memory to create the signalfd.
	pub(crate) fn signalfd(fd: RawFd, mask: *const sigset_t, flags: c_int) -> c_int;
}

/// Sets the `O_NONBLOCK` file status flag on the newly opened signalfd file description.
///
/// Since Linux 2.6.27.
pub(crate) const SFD_NONBLOCK: c_int = O_NONBLOCK;

/// Set the close-on-exec (`FD_CLOEXEC`) flag on the newly opened signalfd file description.
///
/// Since Linux 2.6.27.
pub(crate) const SFD_CLOEXEC: c_int = O_CLOEXEC;


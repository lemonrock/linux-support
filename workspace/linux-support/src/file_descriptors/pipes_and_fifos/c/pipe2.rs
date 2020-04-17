// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `pipe2()` creates a pipe, a unidirectional data channel that can be used for interprocess communication.
	///
	/// The array `pipefd` is used to return two file descriptors referring to the ends of the pipe.
	/// `pipefd[0]` refers to the read end of the pipe.
	/// `pipefd[1]` refers to the write end of the pipe.
	///
	/// Data written to the write end of the pipe is buffered by the kernel until it is read from the read end of the pipe.
	///
	/// The following values can be bitwise ORed in flags to obtain different behavior:-
	///
	/// * `O_NONBLOCK`: Set the `O_NONBLOCK` file status flag on the two new open file descriptions.
	/// * `O_CLOEXEC`: Set the close-on-exec (`FD_CLOEXEC`) flag on the two new file descriptors.
	///
	/// On success, zero is returned.
	/// On error, `-`1 is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EFAULT`: `pipefd` is not valid.
	/// * `EINVAL`: Invalid value in `flags`.
	/// * `EMFILE`: Too many file descriptors are in use by the process.
	/// * `ENFILE`: The system limit on the total number of open files has been reached.
	pub(crate) fn pipe2(pipefd: *mut [RawFd; 2], flags: c_int) -> c_int;
}

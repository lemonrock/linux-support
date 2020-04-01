// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


extern "C"
{
	/// `tee()` copies data between two file descriptors without copying between kernel address space and user address space.
	///
	/// It transfers up to `len` bytes of data from the file descriptor `fd_in` to the file descriptor `fd_out`, where one of the file descriptors must refer to a pipe.
	///
	/// The `flags` argument is a bit mask that is composed by ORing together `SPLICE_F_NONBLOCK` and `SPLICE_F_MORE`.
	///
	/// Upon successful completion, `tee()` returns the number of bytes copied to or from the pipe.
	///
	/// A return value of 0 means end of input.
	/// If `fd_in` refers to a pipe, then this means that there was no data to transfer, and it would not make sense to block because there are no writers connected to the write end of the pipe.
	///
	/// On error, `tee()` returns `-1` and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: `SPLICE_F_NONBLOCK` was specified in flags, and the operation would block.
	/// * `EINVAL`: `fd_in` and `fd_out` does not refer to a pipe; or `fd_in` and `fd_out` refer to the same pipe.
	/// * `ENOMEM`: Out of memory.
	pub(crate) fn tee(fd_in: RawFd, fd_out: RawFd, len: size_t, flags: c_uint) -> ssize_t;
}

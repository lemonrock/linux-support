// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


extern "C"
{
	/// `splice()` moves data between two file descriptors without copying between kernel address space and user address space.
	///
	/// It transfers up to `len` bytes of data from the file descriptor `fd_in` to the file descriptor `fd_out`, where one of the file descriptors must refer to a pipe.
	///
	/// The following semantics apply for `fd_in` and `off_in`:
	///
	/// *  If `fd_in` refers to a pipe, then `off_in` must be `NULL`.
	/// *  If `fd_in` does not refer to a pipe and `off_in` is `NULL`, then bytes are read from `fd_in` starting from the file offset, and the file offset is adjusted appropriately.
	/// *  If `fd_in` does not refer to a pipe and `off_in` is not NULL, then `off_in` must point to a buffer which specifies the starting offset from which bytes will be read from `fd_in`; in this case, the file offset of `fd_in` is not changed.
	///
	/// Analogous statements apply for `fd_out` and `off_out`.
	///
	/// The `flags` argument is a bit mask that is composed by ORing together `SPLICE_F_MOVE`, `SPLICE_F_NONBLOCK` and `SPLICE_F_MORE`.
	///
	/// Upon successful completion, `splice()` returns the number of bytes spliced to or from the pipe.
	///
	/// A return value of 0 means end of input.
	/// If `fd_in` refers to a pipe, then this means that there was no data to transfer, and it would not make sense to block because there are no writers connected to the write end of the pipe.
	///
	/// On error, `splice()` returns `-1` and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: `SPLICE_F_NONBLOCK` was specified in flags, and the operation would block.
	/// * `EBADF`: One or both file descriptors are not valid, or do not have proper read-write mode.
	/// * `EINVAL`: The target filesystem doesn't support splicing.
	/// * `EINVAL`: The target file is opened in append mode.
	/// * `EINVAL`: Neither of the file descriptors refers to a pipe.
	/// * `EINVAL`: An offset was given for a non-seekable device (eg, a pipe).
	/// * `EINVAL`: `fd_in` and `fd_out` refer to the same pipe.
	/// * `ENOMEM`: Out of memory.
	/// * `ESPIPE`: Either `off_in` or `off_out` was not `NULL`, but the corresponding file descriptor refers to a pipe.
	pub(crate) fn splice(fd_in: RawFd, off_in: *mut loff_t, fd_out: RawFd, off_out: *mut loff_t, len: size_t, flags: c_uint) -> ssize_t;
}

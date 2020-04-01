// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


extern "C"
{
	/// The `vmsplice()` system call maps `nr_segs` ranges of user memory described by iov into a pipe.
	///
	/// The file descriptor `fd` must refer to a pipe.
	///
	/// The flags argument is a bit mask that is composed by ORing together any of `SPLICE_F_NONBLOCK` and `SPLICE_F_GIFT`.
	///
	/// Upon successful completion, `vmsplice()` returns the number of bytes transferred to the pipe.
	/// On error, `vmsplice()` returns `-1` and `errno` is set to indicate the error.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: `SPLICE_F_NONBLOCK` was specified in flags, and the operation would block.
	/// * `EBADF`: `fd` is either not valid, or doesn't refer to a pipe.
	/// * `EINVAL`: `nr_segs` is greater than `IOV_MAX`; or memory not aligned if `SPLICE_F_GIFT` set.
	/// * `ENOMEM`: Out of memory.
	pub(crate) fn vmsplice(fd: RawFd, iov: *const iovec, nr_segs: c_ulong, flags: c_uint) -> ssize_t;
}

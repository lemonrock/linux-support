// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Submission Queue Entry (SQE): IO submission data structure.
#[repr(C)]
#[derive(Debug, Clone)]
pub(super) struct io_uring_sqe
{
	/// Type of operation for this SQE.
	pub(super) opcode: IORING_OP,

	/// `IOSQE_*` flags.
	pub(super) flags: u8,

	/// Specifies specifies the I/O priority.
	///
	/// See `man 2 ioprio_get()` for a description of Linux I/O priorities.
	pub(super) ioprio: u16,

	/// Either:-
	/// * Index of file descriptor in file descriptors array to do io_uring operation (eg `IOSQE_FIXED_FILE` is specified)
	///
	/// Or:-
	/// * An actual `RawFd` file descriptor.
	pub(super) fd: i32,

	pub(super) anonymous_1: io_uring_sqe_anonymous_1,

	pub(super) anonymous_2: io_uring_sqe_anonymous_2,

	/// buffer size or number of iovecs.
	pub(super) len: u32,

	pub(super) anonymous_3: io_uring_sqe_anonymous_3,

	/// This is an application-supplied value that will be copied into the `CompletionQueueEntry` verbatim.
	pub(super) user_data: u64,

	pub(super) anonymous_4: io_uring_sqe_anonymous_4,
}

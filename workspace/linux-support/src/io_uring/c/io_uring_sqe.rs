// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Submission Queue Entry (SQE): IO submission data structure.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct io_uring_sqe
{
	/// Type of operation for this SQE.
	pub(super) opcode: IORING_OP,

	/// `IOSQE_*` flags.
	pub(super) flags: SubmissionQueueEntryFlags,

	/// Specifies specifies the I/O priority.
	///
	/// See `man 2 ioprio_get()` for a description of Linux I/O priorities.
	pub(super) ioprio: CompressedIoPriority,

	/// Either:-
	/// * Index of file descriptor in file descriptors array to do io_uring operation (ie flags contains `SubmissionQueueEntryFlags::FixedFile` is specified).
	///
	/// Or:-
	/// * An actual `RawFd` file descriptor.
	///
	/// Exceptions when it is only ever a raw fd:-
	///
	/// * `IORING_OP_CLOSE`,
	/// * `IORING_OP_OPENAT`,
	/// * `IORING_OP_OPENAT2`,
	/// * `IORING_OP_STATX`,
	pub(super) fd: FileDescriptorKind,

	pub(super) anonymous_1: io_uring_sqe_anonymous_1,

	pub(super) anonymous_2: io_uring_sqe_anonymous_2,

	/// buffer size or number of iovecs.
	pub(super) len: u32,

	pub(super) anonymous_3: io_uring_sqe_anonymous_3,

	/// This is an application-supplied value that will be copied into the `CompletionQueueEntry` verbatim.
	pub(super) user_data: u64,

	pub(super) anonymous_4: io_uring_sqe_anonymous_4,
}

impl io_uring_sqe
{
	#[inline(always)]
	pub(super) fn prepare(&mut self, ioring_operation: IORING_OP, file_descriptor: FileDescriptorOrigin<impl FileDescriptor>, address: u64, length: u32, offset: u64, flags: SubmissionQueueEntryFlags, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, user_data: u64)
	{
		let (fd, flags) = file_descriptor.into_and_adjust_flags(flags);

		self.opcode = ioring_operation;
		self.flags = flags;
		self.ioprio = io_priority;
		self.fd = fd;
		self.anonymous_1.off = offset;
		self.anonymous_2.addr = address;
		self.len = length;
		self.user_data = user_data;
		self.anonymous_4.anonymous_1.personality = unsafe { transmute(personality) };
		unsafe { self.anonymous_4.__pad2 = zeroed() }
	}
}

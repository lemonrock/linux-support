// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	pub(super) struct SubmissionQueueEntryFlags: u8
	{
		/// When this flag is specified, `fd` is an index into the files array registered with the `io_uring` instance (see the `IORING_REGISTER_FILES` section of the `io_uring_register()` man page).
		///
		/// If this flag is not specified, then `fd` is a regular file descriptor.
		///
		/// Since Linux 5.1.
		const FixedFile = IOSQE_FIXED_FILE;

		/// When this flag is specified, the `SubmissionQueueEntry` will not be started before previously submitted `SubmissionQueueEntry`s have completed, and new `SubmissionQueueEntry`s will not be started before this one completes.
		///
		/// Since Linux 5.2.
		const IoDrain = IOSQE_IO_DRAIN;

		/// When this flag is specified, it forms a link with the next `SubmissionQueueEntry` in the submission ring.
		///
		/// That next `SubmissionQueueEntry` will not be started before this one completes.
		/// This, in effect, forms a chain of `SubmissionQueueEntry`s, which can be arbitrarily long.
		/// The tail of the chain is denoted by the first `SubmissionQueueEntry` that does not have this flag set.
		/// This flag has no effect on previous `SubmissionQueueEntry` submissions, nor does it impact `SubmissionQueueEntry`s that are outside of the chain tail.
		/// This means that multiple chains can be executing in parallel, or chains and individual `SubmissionQueueEntry`s.
		/// Only members inside the chain are serialized.
		/// A chain of `SubmissionQueueEntry`s will be broken, if any request in that chain ends in error.
		/// io_uring considers any unexpected result an error.
		/// This means that, eg, a short read will also terminate the remainder of the chain.
		/// If a chain of `SubmissionQueueEntry` links is broken, the remaining unstarted part of the chain will be terminated and completed with -`ECANCELED` as the error code.
		///
		/// Since Linux 5.3.
		const Link = IOSQE_IO_LINK;

		/// Like `Link`, but it doesn't sever regardless of the completion result.
		///
		/// Note that the link will still sever if we fail submitting the parent request, hard links are only resilient in the presence of completion results for requests that did submit correctly.
		///
		/// Implies `Link`.
		///
		/// Since Linux 5.5.
		const HardLink = IOSQE_IO_HARDLINK;

		/// Normal operation for io_uring is to try and issue an `SubmissionQueueEntry` as non-blocking first, and if that fails, execute it in an async manner.
		///
		/// To support more efficient overlapped operation of requests that the application knows/assumes will always (or most of the time) block, the application can ask for a `SubmissionQueueEntry` to be issued async from the start.
		///
		/// Since Linux 5.6.
		const AlwaysGoAsynchronous = IOSQE_ASYNC;

		/// Select a buffer for a read-like operation using `buf_group`.
		///
		/// Since Linux 5.7.
		const SelectBufferUsingBufferGroup = IOSQE_BUFFER_SELECT;
	}
}

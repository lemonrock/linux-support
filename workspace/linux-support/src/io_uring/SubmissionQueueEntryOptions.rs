// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Options.
	pub struct SubmissionQueueEntryOptions: u8
	{
		/// Wait for previous submissions to complete before starting this one.
		///
		/// Important for ordering `close()`.
		const FlushPrevious = IOSQE_IO_DRAIN;

		/// This submission, along with any immediately subsequent or future forms a chain which should complete together.
		const Chain = IOSQE_IO_LINK;

		/// This submission, along with any immediately subsequent or future forms a chain which should complete together.
		///
		/// Implies `Chain`.
		const StrongChain = IOSQE_IO_LINK | IOSQE_IO_HARDLINK;

		/// Force completion to not be undertaken non-blocking but using asynchronous internal Linux kernel threads.
		const ForceAsynchronous = IOSQE_ASYNC;
	}
}

impl SubmissionQueueEntryOptions
{
	#[inline(always)]
	fn into_flags(self) -> SubmissionQueueEntryFlags
	{
		unsafe { transmute(self) }
	}
}

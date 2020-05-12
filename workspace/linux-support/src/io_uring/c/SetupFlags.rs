// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	pub(super) struct SetupFlags: u32
	{
		/// Using this causes many, many kinds of io_uring operations to be unusable.
		const IoPoll = IORING_SETUP_IOPOLL;

		/// Using this requires a kernel thread and to *only* use registered file descriptors.
		const SubmissionQueuePoll = IORING_SETUP_SQPOLL;

		/// Requires `SubmissionQueuePoll` to be specified.
		const SubmissionQueueAffinity = IORING_SETUP_SQ_AFF;

		#[allow(missing_docs)]
		const CompletionQueueSize = IORING_SETUP_CQSIZE;

		/// If specified then if the number of entries requested exceeds `IORING_MAX_ENTRIES` or the number of CompletionQueue entries exceeds `IORING_MAX_CQ_ENTRIES` the number of entries is clamped and an error of `EINVAL` is not returned.
		const Clamp = IORING_SETUP_CLAMP;

		#[allow(missing_docs)]
		const AttachWorkQueue = IORING_SETUP_ATTACH_WQ;
	}
}

impl Default for SetupFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

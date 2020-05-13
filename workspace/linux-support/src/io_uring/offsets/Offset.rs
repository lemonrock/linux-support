// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See `WithOffset` and `WithoutOffset`.
pub trait Offset<'a, FD: 'a + FileDescriptor>
{
	#[doc(hidden)]
	fn into_raw_for_splice_in(&self, splice_flags: SpliceFlags, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, u32), u64);
	
	#[doc(hidden)]
	fn into_raw_for_splice_out(&self, options: SubmissionQueueEntryOptions, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, SubmissionQueueEntryFlags), u64);
	
	#[doc(hidden)]
	fn into_raw(&self, options: SubmissionQueueEntryOptions, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, SubmissionQueueEntryFlags), u64);
}

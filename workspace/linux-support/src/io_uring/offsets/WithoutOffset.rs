// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Splice without offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithoutOffset<'a, FD: 'a + PipeLikeFileDescriptor>
{
	/// Origin.
	pub file_descriptor_origin: FileDescriptorOrigin<'a, FD>,
}

impl<'a, FD: 'a + PipeLikeFileDescriptor> Offset<'a, FD> for WithoutOffset<'a, FD>
{
	#[inline(always)]
	fn into_raw_for_splice_in(&self, splice_flags: SpliceFlags, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, u32), u64)
	{
		(self.file_descriptor_origin.into_and_adjust_splice_flags(splice_flags, using_kernel_submission_queue_poll), Self::NoSpliceOffset)
	}
	
	#[inline(always)]
	fn into_raw_for_splice_out(&self, options: SubmissionQueueEntryOptions, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, SubmissionQueueEntryFlags), u64)
	{
		(self.file_descriptor_origin.into_and_adjust_flags(options, using_kernel_submission_queue_poll), Self::NoSpliceOffset)
	}
	
	#[inline(always)]
	fn into_raw(&self, options: SubmissionQueueEntryOptions, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, SubmissionQueueEntryFlags), u64)
	{
		(self.file_descriptor_origin.into_and_adjust_flags(options, using_kernel_submission_queue_poll), 0)
	}
}

impl<'a, FD: 'a + PipeLikeFileDescriptor> WithoutOffset<'a, FD>
{
	const NoSpliceOffset: u64 = -1i64 as u64;
}

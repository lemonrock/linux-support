// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// With offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithOffset<'a, FD: 'a + SeekableFileDescriptor>
{
	/// Origin.
	pub file_descriptor_origin: FileDescriptorOrigin<'a, FD>,
	
	/// Offset.
	pub offset: ReadOrWriteOffset,
}

impl<'a, FD: 'a + SeekableFileDescriptor> Offset<'a, FD> for WithOffset<'a, FD>
{
	#[inline(always)]
	fn into_raw_for_splice_in(&self, splice_flags: SpliceFlags, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, u32), u64)
	{
		(self.file_descriptor_origin.into_and_adjust_splice_flags(splice_flags, using_kernel_submission_queue_poll), self.offset.into())
	}
	
	#[inline(always)]
	fn into_raw_for_splice_out(&self, options: SubmissionQueueEntryOptions, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, SubmissionQueueEntryFlags), u64)
	{
		(self.file_descriptor_origin.into_and_adjust_flags(options, using_kernel_submission_queue_poll), self.offset.into())
	}
	
	#[inline(always)]
	fn into_raw(&self, options: SubmissionQueueEntryOptions, using_kernel_submission_queue_poll: bool) -> ((FileDescriptorKind, SubmissionQueueEntryFlags), u64)
	{
		(self.file_descriptor_origin.into_and_adjust_flags(options, using_kernel_submission_queue_poll), self.offset.into())
	}
}

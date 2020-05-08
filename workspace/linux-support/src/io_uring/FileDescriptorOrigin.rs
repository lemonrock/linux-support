// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Origin of file descriptor to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum FileDescriptorOrigin<'a, FD: 'a + FileDescriptor>
{
	/// An index into the file descriptors previously registered with the `IoUring`.
	Index(u32),

	/// A file descriptor reference.
	Absolute(&'a FD),

	/// Irrelevant.
	Irrelevant,
}

impl<'a, FD: 'a + FileDescriptor> FileDescriptorOrigin<'a, FD>
{
	#[inline(always)]
	fn into_and_adjust_flags(self, flags: SubmissionQueueEntryFlags) -> (FileDescriptorKind, SubmissionQueueEntryFlags)
	{
		use self::FileDescriptorOrigin::*;

		match self
		{
			Index(index) => (FileDescriptorKind::Index(index), flags | SubmissionQueueEntryFlags::FixedFile),

			Absolute(file_descriptor) => (FileDescriptorKind::from(file_descriptor), flags),

			Irrelevant => (FileDescriptorKind::Irrelevant, flags)
		}
	}

	#[inline(always)]
	fn into_and_adjust_splice_flags(self, flags: SpliceFlags) -> (FileDescriptorKind, u32)
	{
		use self::FileDescriptorOrigin::*;

		let flags = flags.bits;

		match self
		{
			Index(index) => (FileDescriptorKind::Index(index), flags | SPLICE_F_FD_IN_FIXED),

			Absolute(file_descriptor) => (FileDescriptorKind::from(file_descriptor), flags),

			Irrelevant => panic!("Unsupported for Splice"),
		}
	}
}

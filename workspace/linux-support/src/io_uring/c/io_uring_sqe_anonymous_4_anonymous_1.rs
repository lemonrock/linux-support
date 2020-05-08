// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(super) struct io_uring_sqe_anonymous_4_anonymous_1
{
	pub(super) anonymous_1: io_uring_sqe_anonymous_4_anonymous_1_anonymous_1,

	/// The credentials identifier (personality) to use for this operation.
	/// See `io_uring_register()` for how to register personalities with io_uring.
	///
	/// `0` is interpreted as the current personality of the submitting task.
	pub(super) personality: Option<PersonalityCredentialsIdentifier>,

	pub(super) splice_fd_in: FileDescriptorKind,
}

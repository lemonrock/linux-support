// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Completion Queue Entry (SQE): IO completion data structure.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub(super) struct io_uring_cqe
{
	/// As supplied in `io_uring_sqe.user_data`.
	pub(super) user_data: u64,

	/// Result; might be an error code; might be a buffer identifier (check `self.flags`).
	pub(super) res: i32,

	pub(super) flags: CompletionQueueEntryFlags,
}

impl io_uring_cqe
{
	#[inline(always)]
	pub(super) fn has_buffer_identifier(&self) -> bool
	{
		self.flags.contains(CompletionQueueEntryFlags::Upper16BitsAreBufferIdentifier)
	}

	#[inline(always)]
	pub(super) fn buffer_identifier(&self) -> u16
	{
		debug_assert!(self.has_buffer_identifier());

		((self.res as u32) >> (IORING_CQE::IORING_CQE_BUFFER_SHIFT as u32)) as u16
	}
}

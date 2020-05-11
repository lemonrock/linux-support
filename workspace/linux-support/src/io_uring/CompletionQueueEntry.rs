// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Completion queue entry, `CQE`.
#[derive(Debug, Copy, Clone)]
pub struct CompletionQueueEntry<'a>(&'a io_uring_cqe);

impl CompletionQueueEntry<'_>
{
	/// User data.
	///
	/// Use this to link back to the file descriptor, any state associated with it and any buffers used in the original submission queue entry that can now be freed.
	///
	/// Some buffers can be freed or reused once the submission queue entry has been removed.
	#[inline(always)]
	pub fn user_data(self) -> u64
	{
		self.0.user_data
	}

	/// Overlaps with `buffer_identifier`, below.
	///
	/// If negative, and between -1 and -4095 inclusive, should represent a Linux error code negated such as `-EINVAL`.
	#[inline(always)]
	pub fn result_or_error(self) -> CompletionResponse
	{
		CompletionResponse(self.0.res)
	}

	/// Has buffer identifier.
	#[inline(always)]
	pub fn has_buffer_identifier(self) -> bool
	{
		self.0.has_buffer_identifier()
	}

	/// Buffer identifier.
	///
	/// Check `has_buffer_identifier()` first.
	#[inline(always)]
	pub fn buffer_identifier(self) -> u16
	{
		self.0.buffer_identifier()
	}
}

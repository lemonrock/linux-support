// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Assumes that only the 'zero' user bits are in use.
#[derive(Debug)]
pub(crate) struct SimpleIoUringYielder<'yielder, Complete: Sized>
{
	yielder: Yielder<'yielder, SimpleIoUringResumeArguments, SimpleIoUringYields, Complete>,
	
	io_uring: Rc<IoUring<'static>>,
	
	user_data_for_io_uring_operation_0: u64,
}

impl<'yielder, Complete: Sized> SimpleIoUringYielder<'yielder, Complete>
{
	#[inline(always)]
	pub(crate) fn new(yielder: Yielder<'yielder, SimpleIoUringResumeArguments, SimpleIoUringYields, Complete>, io_uring: Rc<IoUring<'static>>, coroutine_instance_handle: CoroutineInstanceHandle) -> Self
	{
		Self
		{
			yielder,
			io_uring,
			user_data_for_io_uring_operation_0: coroutine_instance_handle.set_user_bits(UserBits::Zero).unwrap(),
		}
	}
	
	/// Called from coroutine.
	///
	/// Yields to coroutine manager thread.
	///
	/// Returns to coroutine.
	///
	/// Assumes that only the 'zero' user bits are in use (`user_data_for_io_uring_operation_0`).
	#[inline(always)]
	pub(crate) fn yield_submit_io_uring(&mut self, mut add_entry: impl FnMut(SubmissionQueueEntry, u64)) -> bool
	{
		let user_data_for_io_uring_operation_0 = self.user_data_for_io_uring_operation_0;
		
		let mut entry = |submission_queue_entry|
		{
			add_entry(submission_queue_entry, user_data_for_io_uring_operation_0)
		};
		
		SimpleIoUringYields::yield_submit_io_uring(&mut self.yielder, &self.io_uring, &mut entry)
	}
	
	/// Called from coroutine.
	///
	/// Yields to coroutine manager thread.
	///
	/// Returns to coroutine.
	///
	/// Assumes that only the 'zero' user bits are in use.
	#[inline(always)]
	pub(crate) fn yield_awaiting_io_uring(&mut self) -> Result<CompletionResponse, ()>
	{
		SimpleIoUringYields::yield_awaiting_io_uring(&mut self.yielder)
	}
}

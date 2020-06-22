// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait CoroutineDispatch
{
	fn dispatch_retry_because_io_uring_submission_queue_was_full(&mut self, coroutine_instance_handle: CoroutineInstanceHandle) -> CoroutineRequiresReEntry;
	
	fn dispatch_io_uring(self, coroutine_instance_handle_and_completion_response: (CoroutineInstanceHandle, CompletionResponse)) -> CoroutineRequiresReEntry;
}

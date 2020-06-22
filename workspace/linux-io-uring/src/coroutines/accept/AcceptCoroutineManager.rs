// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) type AcceptCoroutineManager<SA: SocketAddress, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, StackSizeAccept: MemorySize> = CoroutineManager<CoroutineHeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<SA, CoroutineHeapSize, GTACSA>, AcceptCoroutineInformation>;

impl<SA: SocketAddress, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, StackSizeAccept: MemorySize, NonCoroutineHandlerError: error::Error> CoroutineDispatch<NonCoroutineHandlerError> for AcceptCoroutineManager<SA, CoroutineHeapSize, GTACSA, StackSizeAccept>
{
	#[inline(always)]
	fn dispatch_retry_because_io_uring_submission_queue_was_full(&mut self, coroutine_instance_handle: CoroutineInstanceHandle) -> CoroutineRequiresReEntry
	{
		let coroutine_instance_pointer = unsafe { CoroutineInstancePointer::<CoroutineHeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<SA, CoroutineHeapSize, GTACSA>, AcceptCoroutineInformation>::from_handle(coroutine_instance_handle) };
		
		use self::ResumeOutcome::*;
		use self::AcceptYields::*;
		use self::CoroutineRequiresReEntry::*;
		match self.resume_coroutine(coroutine_instance_pointer, AcceptResumeArguments::TrySubmissionQueueAgain)
		{
			WouldLikeToResume(AwaitingIoUring) => CarryOn,
			
			WouldLikeToResume(SubmissionQueueFull) => IoUringSubmissionQueueWasFull(coroutine_instance_handle),
			
			Complete(()) => panic!("An accept loop should never complete"),
		}
	}
	
	#[inline(always)]
	fn dispatch_io_uring(self, coroutine_instance_handle_and_completion_response: (CoroutineInstanceHandle, CompletionResponse)) -> Result<CoroutineRequiresReEntry, DispatchIoUringError<NonCoroutineHandlerError>>
	{
		let (coroutine_instance_handle, completion_response) = coroutine_instance_handle_and_completion_response;
		
		let coroutine_instance_pointer = unsafe { CoroutineInstancePointer::<CoroutineHeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<SA, CoroutineHeapSize, GTACSA>, AcceptCoroutineInformation>::from_handle(coroutine_instance_handle) };
		
		use self::ResumeOutcome::*;
		use self::AcceptYields::*;
		use self::CoroutineRequiresReEntry::*;
		match self.resume_coroutine(coroutine_instance_pointer, AcceptResumeArguments::Submitted(coroutine_instance_handler.user_bits(), completion_response))
		{
			WouldLikeToResume(AwaitingIoUring) => Ok(CarryOn),
			
			WouldLikeToResume(SubmissionQueueFull) => Ok(IoUringSubmissionQueueWasFull(coroutine_instance_handle)),
			
			Complete(()) => panic!("An accept loop should never complete"),
		}
	}
}

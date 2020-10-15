// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Manages dispatch to a server-loop coroutine.
#[derive(Debug)]
pub(crate) struct ServerLoopCoroutineManager<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineStackSize: MemorySize, C: Coroutine<ResumeArguments=SimpleIoUringResumeArguments, Yields=SimpleIoUringYields, Complete=()>>
{
	coroutine_manager: CoroutineManager<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, UnusedComplete>,
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineStackSize: MemorySize, C: Coroutine<ResumeArguments=SimpleIoUringResumeArguments, Yields=SimpleIoUringYields, Complete=()>> From<CoroutineManager<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, UnusedComplete>> for ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, CoroutineStackSize, C>
{
	#[inline(always)]
	fn from(coroutine_manager: CoroutineManager<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, UnusedComplete>) -> Self
	{
		Self
		{
			coroutine_manager
		}
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineStackSize: MemorySize, C: Coroutine<ResumeArguments=SimpleIoUringResumeArguments, Yields=SimpleIoUringYields, Complete=()>> Deref for ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, CoroutineStackSize, C>
{
	type Target = CoroutineManager<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, UnusedComplete>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.coroutine_manager
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineStackSize: MemorySize, C: Coroutine<ResumeArguments=SimpleIoUringResumeArguments, Yields=SimpleIoUringYields, Complete=()>> DerefMut for ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, CoroutineStackSize, C>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.coroutine_manager
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineStackSize: MemorySize, C: Coroutine<ResumeArguments=SimpleIoUringResumeArguments, Yields=SimpleIoUringYields, Complete=()>> CoroutineDispatch for ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, CoroutineStackSize, C>
{
	#[inline(always)]
	fn dispatch_retry_because_io_uring_submission_queue_was_full(&mut self, coroutine_instance_handle: CoroutineInstanceHandle) -> CoroutineRequiresReEntry
	{
		let coroutine_instance_pointer = unsafe { CoroutineInstancePointer::<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, UnusedCoroutineInformation>::from_handle(coroutine_instance_handle) };
		
		use self::ResumeOutcome::*;
		use self::SimpleIoUringYields::*;
		use self::CoroutineRequiresReEntry::*;
		match self.resume_coroutine(coroutine_instance_pointer, SimpleIoUringResumeArguments::TrySubmissionQueueAgain)
		{
			WouldLikeToResume(AwaitingIoUring) => CarryOn,
			
			WouldLikeToResume(SubmissionQueueFull) => IoUringSubmissionQueueWasFull(coroutine_instance_handle),
			
			Complete(()) => panic!("A server loop should never complete after resuming because `dispatch_retry_because_io_uring_submission_queue_was_full`"),
		}
	}
	
	#[inline(always)]
	fn dispatch_io_uring(&mut self, coroutine_instance_handle_and_completion_response: (CoroutineInstanceHandle, CompletionResponse)) -> CoroutineRequiresReEntry
	{
		let (coroutine_instance_handle, completion_response) = coroutine_instance_handle_and_completion_response;
		
		let coroutine_instance_pointer = unsafe { CoroutineInstancePointer::<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, UnusedCoroutineInformation>::from_handle(coroutine_instance_handle) };
		
		use self::ResumeOutcome::*;
		use self::SimpleIoUringYields::*;
		use self::CoroutineRequiresReEntry::*;
		match self.resume_coroutine(coroutine_instance_pointer, SimpleIoUringResumeArguments::Submitted(coroutine_instance_handle.user_bits(), completion_response))
		{
			WouldLikeToResume(AwaitingIoUring) => CarryOn,
			
			WouldLikeToResume(SubmissionQueueFull) => IoUringSubmissionQueueWasFull(coroutine_instance_handle),
			
			Complete(()) => panic!("A server loop should never complete after resuming because `dispatch_io_uring`"),
		}
	}
}

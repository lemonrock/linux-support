// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct ThreadLoop<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	global_allocator: &'static GTACSA,
	io_uring: IoUring<'static>,
	registered_buffers: RegisteredBuffers,
	signal_file_descriptor: SignalFileDescriptor,
	our_hyper_thread: HyperThread,
	queues: Queues<(), DequeuedMessageProcessingError>,
	accept_connections_coroutine_manager: CoroutineManager<HeapSize, StackSize, GTACSA, C, AcceptConnectionsCoroutine>,
}

/*
	Operation combinations
	
		Obtain Buffer then Operation Read Fixed link-with Operation Timeout then Release Buffer
			or, Cant Obtain so Operation Timeout and Try Again
		
		Obtain Buffer then Operation Write Fixed link-with Timeout then Release Buffer
			or, Cant Obtain so Operation Timeout and Try Again
		
		Obtain File Descriptor Index, Obtain Buffer then Operation Register File Descriptor link-with Operation Read/Write Fixed link-with Operation (De)Register File Descriptor link-with Operation Timeout then Release Buffer then Release File Descriptor Index
			- that's a lot of steps for a very, very minor performance gain of using File Descriptor Index
			- File Descriptor Index are in very short supply compared to buffers
			- Links are only performant on Linux 5.7, and, even then, probably add overhead
			- Registered file descriptor indices only make sense for long-lived network connections
		
		Multiple Queued operations (Reads, Writes, etc) link-with Timeout
		
		Obtain File Descriptor Index then Operation Accept then Operation Update File Descriptor
			or, Can't Obtain so Operation Timeout and Try Again
			alternative, Operation Accept, Obtain File Descriptor, can't so Operation Timeout and Try Again
				or, can't, so Operation Close
		
		Operation Connect link-with Operation Timeout
		
		Operation Close link-with Operation Update File Descriptor then Release File Descriptor Index
			- Need to decide order; File Descriptor Index is a precious resource
				- Close does not use a File Descriptor Index; not sure of impact of leaving a registered file descriptor index if using close
	
	Cleaning up a coroutine 'on drop'
	
		Need to cancel all outstanding SQEs
		Need to ignore all results
		Need to tell the coroutine "you are dead"
		Need to identify incoming CQE user data for a dead coroutine w/o having to keep coroutine heap and stack alive
		Need to Deregister Buffer(s)
	
	Question to answer
		Should sockets now be opened blocking? (some hints that is the better case)
	
	Tracking outstanding completions
		If we know the total we can wait for all of them
			- but no advantage, as we're block for all of them
		We can know the total for a particular coroutine, and so put in place a timeout (which we can then cancel)
			We can have a timeout to kill a coroutine after X seconds regardless of what's pending, a sort-of coroutine inactivity timeout, but I think a coroutine is best placed to know this
	
	Coroutine wake up combinations
		All pending completions have completed
		Some pending completions have completed
		No pending completions have completed but some sort of timeout has occurred
		Non-IO completion to wake up to try something again (eg obtaining a buffer, posting a message)
		Wake ups are very cheap, the cost of an indirect function call, so it's probably best to just get a coroutine to decide whether to continue by waking it up.
		One worry is the use of multiple buffers by a coroutine
	
	Forgetting file descriptors
		We need to change file descriptor logic to 'know' they shouldn't close on drop, or come up with a better way
			Close on drop works well for a panic or unexpected exit, however
	
	Coroutine completion coalescing
		Instead of waking-up a coroutine immediately for each completion, we pull off all of them and push them into an internal queue or array (indexed by a 4-bit operation index code).
		We must remember there is no order to completions; this is tricky for 'linked' completions.
		
		Some completions require immediate wake up, eg a timeout of a read, write or connect which may be coroutine-ending.
			This can be an 'always wake up on any of these operation index codes' bit mask.
		
		Some completions do not, eg we are waiting on two reads. Until both reads are true we do not want to wake up.
			This can be the inverse of the 'always wake up on any of these operation index codes' bit mask.
		
		However, we may want to also wake up when one read fails of the two both being done. Thus we want to wake up when:-
			- one or both reads fail (failure being permament but not a temporary EAGAIN like situation)
			- one or both reads timeout (ie report that they are cancelled OR the timeout reports it is succcesful)
			- both reads succeed without timeout
				- we don't want to wake up at all for 'without timeout completions'
			- but not one read succeeds and the other is still pending
			
		The simplest approach is to leave processing from i32 to Result _to the coroutine_, and wake up the coroutine every single time a result arrives
			- the downside is that we won't have committed our completions yet, so the completion queue and / or submission queue may still be full.
			- if the submission queue is full the coroutine will want to be woken up AGAIN as soon as it isn't at the exact point it was full.
	
	https://github.com/CarterLi/liburing4cpp
	Performance suggestions
	
		Batch syscalls
			Use io_uring_for_each_cqe to peek multiple cqes once, handle them and enqueue multiple sqes.
			After all cqes are handled, submit all sqes.
			ie Handle multiple CQEs then submit ALL SQEs ONCE
		
		Blocking for reads of non-disk I/O
			For operations that may block, kernel will punt them into a kernel worker called io-wq, which turns out to have a high overhead cost.
			Always make sure that the fd to read is ready to read.
			Thus use POLL_ADD then *separately* FIXED-READ
			This is changing in Linux 5.7.
		
		Link-With
			This is borked in Linux 5.6 and before; it forces operations after poll be executed async, that makes POLL_ADD mostly useless (See: https://lore.kernel.org/io-uring/5f09d89a-0c6d-47c2-465c-993af0c7ae71@kernel.dk/).
		
		Link-With for READ link-with WRITE
			A short read is an error for the link-chain which then cancels WRITE.
		
		Fixed file descriptors have very little performance boost for a lot of complexity and a *very* limited resource.
 */

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ThreadLoopBodyFunction for ThreadLoop<HeapSize, StackSize, GTACSA>
{
	#[inline(always)]
	fn invoke(&mut self, terminate: &Arc<impl Terminate>)
	{
		debug_assert!(terminate.should_continue());
		
		self.process_all_outstanding_completions();

		let exit = self.process_thread_control_messages(terminate);
		if unlikely!(exit)
		{
			return
		}
		
		match self.io_uring.initiate_asynchronous_io()
		{
			Ok(n) => (),
			Err(submit_error) => xxx,
		}
	}
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ThreadLoop<HeapSize, StackSize, GTACSA>
{
	#[inline(always)]
	fn process_all_outstanding_completions(&mut self)
	{
		let iterator = self.io_uring.current_completion_queue_entries();
		for completion_queue_entry in iterator
		{
			let user_data = TaggedAbsolutePointer(completion_queue_entry.user_data());
			let completion_response = completion_queue_entry.completion_response();
			
			xxx(user_data, completion_response)
		}
		drop(iterator)
	}
	
	#[inline(always)]
	fn process_thread_control_messages(&mut self, terminate: &Arc<impl Terminate>) -> bool
	{
		if Err(dequeued_message_processing_error) = self.queues.subscriber(self.our_hyper_thread).receive_and_handle_messages(terminate, &())
		{
			use self::DequeuedMessageProcessingError::*;
			
			let message_handler_arguments = ();
			let result = self.incoming_messages.receive_and_handle_messages(());
			match result
			{
				Err(Fatal(ref cause)) =>
				{
					terminate.begin_termination_due_to_irrecoverable_error(cause, None);
					true
				},
				
				Err(CarryOn(ref cause)) => LocalSyslogSocket::syslog(Severity::Warning, format!("CarryOn:{}", cause)),
				
				Ok(()) => false,
			}
		}
		else
		{
			false
		}
	}
}

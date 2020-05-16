// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct ThreadLoopInitiation<HeapSize: Sized, StackSize: Sized, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	defaults: DefaultPageSizeAndHugePageSizes,
	global_allocator: &'static GTACSA,
	thread_local_allocator_settings: ThreadLocalAllocatorSettings,
	io_uring_settings: IoUringSettings,
	signal_mask: BitSet<Signal>,
}

pub fn x(hyper_threads: &BitSet<HyperThread>, defaults: &DefaultPageSizeAndHugePageSizes)
{
	// TODO: WHAT ABOUT message handlers????
	
	Queue::queues(&hyper_threads, defaults, preferred_buffer_size, inclusive_maximum_bytes_wasted);
}


type MessageHandlerArguments = ();

type MessageHandlersRegisterationArguments = ();

struct SimpleMessageHandlersRegistration
{
}

impl MessageHandlersRegistration for SimpleMessageHandlersRegistration
{
	/// Message handler arguments type.
	type MessageHandlerArguments = MessageHandlerArguments;
	
	/// Error type.
	type Error = DequeuedMessageProcessingError;
	
	/// Arguments passed when registering.
	type Arguments = MessageHandlersRegisterationArguments;
	
	/// Register all messages handlers.
	///
	/// Useful extension: return set of registrations?
	///
	/// This design is overly complicated, partly because `Mutable...CompressedMap` uses an ArrayVec and a HashMap.
	/// We want something that is much easier to share across threads.
	fn register_all_message_handlers(&self, register: &mut impl Register<Self::MessageHandlerArguments, Result<(), Self::Error>>, arguments: &Self::Arguments)
	{
		type MessageType0 = ();
		fn handler_for_function_compressed_type_identifier_0(receive: &mut MessageType0, message_handler_arguments: MessageHandlerArguments)
		{
		}
		
		// To publish a message, do the following:-
		
		// (1) Once only: Find the CompressedTypeIdentifier for a Receiver using MutableTypeErasedBoxedFunctionCompressedMap.find_compressed_type_identifier().
		// (2) Publish the message using the compressed type identifier and the receive kind
		// Note: One possible option might be try to create a PHF at runtime once all registrations are done.
		
		let compressed_type_identifier = register.enter_into_the_register(handler_for_function_compressed_type_identifier_0);
	}
}


impl<HeapSize: Sized, StackSize: Sized, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ThreadFunction for ThreadLoopInitiation<HeapSize, StackSize, GTACSA>
{
	type TLBF = ThreadLoop<HeapSize, StackSize, GTACSA>;
	
	fn initialize(self) -> Self::TLBF
	{
		self.thread_local_allocator_settings.setup(&self.defaults, self.global_allocator).expect("Could not create thread local allocator memory map source");
		
		let coroutine_memory_warehouse = CoroutineMemoryWarehouse::new(self.global_allocator, self.ideal_maximum_number_of_coroutines, &self.defaults).expect("Could not create coroutine memory warehouse");
		
		self.io_uring_settings.setup(&self.defaults).expect("Could not setup IoUring");
		
		self.signal_mask.block_all_signals_on_current_thread_bar();
		let signal_file_descriptor = SignalFileDescriptor::new(&self.signal_mask.to_sigset_t()).expect("Could not create signal file descriptor");
		
		// TODO: ???
		let queue_per_threads_publisher = XXXXXX;
		let message_handlers_registration = SimpleMessageHandlersRegistration {};
		let message_handlers_registration_arguments = MessageHandlersRegisterationArguments;
		
		
		ThreadLoop
		{
			incoming_messages: PerThreadQueueSubscriber::new(queue_per_threads_publisher, &message_handlers_registration, &message_handlers_registration_arguments),
			io_uring,
			registered_buffers,
			signal_file_descriptor,
			coroutine_memory_warehouse,
		}
	}
}

impl<HeapSize: Sized, StackSize: Sized, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ThreadLoopInitiation<HeapSize, StackSize, GTACSA>
{
	#[inline(always)]
	fn initialize_internal(self) -> Result<Self::TLBF, ThreadLoopInitializationError>
	{
		use self::ThreadLoopInitializationError::*;
		
		self.thread_local_allocator_settings.setup(&self.defaults, self.global_allocator).map_err(ThreadLocalAllocator)?;
		
		let coroutine_memory_warehouse = CoroutineMemoryWarehouse::new(self.global_allocator, self.ideal_maximum_number_of_coroutines, &self.defaults).map_err(CoroutineMemoryWarehouse)?;
		
		let (io_uring, registered_buffers) = self.io_uring_settings.setup(&self.defaults)?;
		
		let signal_file_descriptor = self.signals()?;
		
		Ok
		(
			ThreadLoop
			{
				incoming_messages: XXXX,
				coroutine_memory_warehouse,
				io_uring,
				registered_buffers,
				signal_file_descriptor,
			}
		)
	}
	
	#[inline(always)]
	fn signals(self) -> Result<SignalFileDescriptor, ThreadLoopInitializationError>
	{
		self.signal_mask.block_all_signals_on_current_thread_bar();
		Ok(SignalFileDescriptor::new(&self.signal_mask.to_sigset_t()).map_err(ThreadLoopInitializationError::SignalFileDescriptor)?)
	}
}

#[derive(Debug)]
pub struct ThreadLoop<HeapSize: Sized, StackSize: Sized, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	incoming_messages: PerThreadQueueSubscriber<MessageHandlerArguments, DequeuedMessageProcessingError>,
	coroutine_memory_warehouse: CoroutineMemoryWarehouse<HeapSize, StackSize, GTACSA>,
	io_uring: IoUring<'static>,
	registered_buffers: RegisteredBuffers,
	signal_file_descriptor: SignalFileDescriptor,

}

/*
	Coroutine Operations supported
		Read Fixed Linked to Timeout
		Write Fixed Linked to Timeout
		Accept
		Connect Linked to Timeout
		Close
		Open ?Linked to Timeout?
		
		By linking to a timeout we can kill coroutine operations
	
	Coroutine panics or dies in someway we need to clean up
		Need to cancel all outstanding SQEs
			Need to know user data for
				Read Fixed (x) + Linked to Timeout (x) so we can kill them
				All other operation kinds above
			If we are using fixed buffers, we have to clean up extremely quickly or we'll run out of buffer identifiers
	
	Fixed buffers
		We can have upto 1024
		They can be 1Gb
		We can use them for different size 'buffers'.
		
	Thread Operations supported
		Read (SignalFd)

	Question to answer
		Should sockets now be opened blocking? (some hints that is the better case)
 */



impl<T: Terminate> ThreadLoopBodyFunction for ThreadLoop<T>
{
	#[inline(always)]
	fn invoke(&mut self)
	{
		// TODO: Each coroutine is allocated a fixed number of buffers which we (pre-register) on its behalf unless we find a way to use the register / unregister buffer group functionality.
			// The list of buffers we initially register can be all sorts of sizes.
		
		// TODO: Each coroutine is given a block of 16 file descriptor indices into the registered list.
			// TODO: Not sure how that works for accept() coroutines.
		
		// TODO: Why use poll?
			// TODO: Use it for eventfd readiness
				// eg with `POLLIN`
			// TODO: Use it for signalfd readiness
				// eg `SignalFileDescriptor::read_signals()` with `POLLIN`.
				// There was bug using signalfd with poll_add
		
		// TODO: IDEA:-
		/*
			For a read or write (or send / recv or sendmsg / recvmsg or connect())
			
			- register a read op
			- register a linked timeout op
				- we can then automatically kill most kinds of coroutine
				
			For a close
				- need to wait for close to complete so we can release file descriptor
				
			Registered file descriptor add
				- when receiving a connection with accept()
				- after calling openat2()
				
			Registered file descriptor release
				- after calling close
			
			If doing registerations updates asynchronously we need to impose some-sort of total ordering
				- we can use drain but I suspect that could be expensive
				- we need to track which file descriptor to register next
			
			Supporting 2^32 file descriptors would require a 16Gb array
				- We only need to support half this, so 8Gb.
				- file descriptors are per-process, so the array in theory could be smaller per thread.
				
				- We need a way to allocate and relinquish indices using links
			
			We should track how many completions are outstanding so that we can submit_and_wait() appropriately?
			
			
		 */
		
		/*
			TODO: Registering file descriptors
		 */
		
		Submission polling requires us to always register file descriptors. This can be expensive if not thought about. Inevitably we will have holes in our index necessitating the use of an arena linked list.
		
		Coroutines will need to co-operate.
		
		Closing a file descriptor will require calling a sqe to update the file descriptor entries.
		
		// TODO: Registered buffers allow us to avoid kernel to userspace copies
		// TODO: Registered file descriptors allow us to use a kernel SQ thread.
		
		// TODO: Need to change file descriptor to NOT CLOSE ON drop() if using non-syscall close!!!
		
		// TODO: IDEA:-
		/*
			For coroutines starting several operations in parallel
				- link them
				- use a linked timeout
		 */
		
		// TODO: call .enter() occassionally.

		if self.io_uring.submission_queue_ring_is_full()
		{
			// TODO: drawdown
		}

		let iterator = self.io_uring.current_completion_queue_entries();
		for completion_queue_entry in iterator
		{
			let user_data = CoroutineUserData(completion_queue_entry.user_data());
			let result = completion_queue_entry.result_or_error();

			xxx(user_data, result)
		}
		drop(iterator);

		let result = self.io_uring.initiate_asynchronous_io();


		// TODO: Use this when the submission queue is rather full?
		// TODO: Use this when we know how many operations all woken coroutines have made?
		self.io_uring.initiate_asynchronous_io_and_wait(minimum_wanted_to_completed);




		use self::DequeuedMessageProcessingError::*;

		let message_handler_arguments = ();
		let result = self.incoming_messages.receive_and_handle_messages(());
		match result
		{
			Err(Fatal(ref cause)) =>
			{
				self.terminate.begin_termination_due_to_irrecoverable_error(cause, None);
				return
			},

			Err(CarryOn(ref cause)) => ProcessLoggingConfiguration::syslog(SyslogPriority::warning, format!("CarryOn:{}", cause)),

			Ok(()) => (),
		}

		// TODO: Make sure we draw-down the completion queue as much as possible before invoking new coroutines or re-entering existing ones.
	}
}

// TODO: Revisit design to allow cancel of timeouts and SQEs (and possibly poll adds).
// TODO: We can now get to CoroutineUserData which we can use to cancel a request.

/*
	user_data =>
		- last operation performed so we can interpret cqe.res
			- and file descriptor type so we can 'rehydrate' eg a particular SocketEnum
				- eg 2 types of file (memory, regular file)
			- this type code needs to combine file-descriptor-type + IORING_OP
				- not all ops for all file descriptor types
		- file descriptor or file descriptor index
			- non-atomic count of file descriptor so we know when to close.
		- coroutine to invoke to continue operation
			- not all operations are per-coroutine; some are per thread
				- eg if trying to invoke a new coroutine for a new socket file descriptor, we're out of coroutine memory; we'll need to close(new_socket_fd) using io-uring
		- any data structures, such as buffers, open_how, &mut statx, etc that can now be freed (recycled)
			- these should reside in an Arena of the particular type.
	closing a file descriptor
		- count reaches 0
			- add a close() event, but still retain entry as close() can fail
			- if using reg'd file descriptors, chain-link close SQE to a file update SQE

	Do we need to reference count file descriptors
		- No - Use dup2() et al so different coroutines and threads don't need to open the same file path

	Challenges
		- There is no ordering to completions unless 'linked'
		- We can avoid having ANY file descriptor typing information by relying solely on the coroutine to 'know'
			- If we want to enable coroutines that can start multiple operations in parallel, we need to provide a way so they know which of their operations has completed
			- we could do this by using, say, 4 bits of the user_data to give 16 parallel operations per coroutine.


https://github.com/CarterLi/liburing4cpp
Performance suggestions:

Until Linux 5.6 at least



Batch syscalls

Use io_uring_for_each_cqe to peek multiple cqes once, handle them and enqueue multiple sqes. After all cqes are handled, submit all sqes.

Handle multiple (cqes), submit (sqes) once
For non-disk I/O, always POLL before READ/RECV

For operations that may block, kernel will punt them into a kernel worker called io-wq, which turns out to have high overhead cost. Always make sure that the fd to read is ready to read.

This will change in Linux 5.7



Don't use IOSQE_IO_LINK

As for Linux 5.6, IOSQE_IO_LINK has an issue that force operations after poll be executed async, that makes POLL_ADD mostly useless.

See: https://lore.kernel.org/io-uring/5f09d89a-0c6d-47c2-465c-993af0c7ae71@kernel.dk/

Note: For READ-WRITE chain, be sure to check -ECANCELED result of WRITE operation ( a short read is considered an error in a link chain which will cancel operations after the operation ). Never use IOSQE_IO_LINK for RECV-SEND chain because you can't control the number of bytes to send (a short read for RECV is NOT considered an error. I don't know why).

This will change in Linux 5.7


Don't use FIXED_FILE & FIXED_BUFFER

They have little performace boost but increase much code complexity. Because the number of files and buffers can be registered has limitation, you almost always have to write fallbacks. In addition, you have to reuse the old file slots and buffers. See example: https://github.com/CarterLi/liburing4cpp/blob/daf6261419f39aae9a6624f0a271242b1e228744/demo/echo_server.cpp#L37

Note RECV/SEND have no _fixed variant.

*/

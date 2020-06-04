// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl<SA: SocketAddress> Coroutine for AcceptConnectionsCoroutine<SA>
{
	/// Type of the arguments the coroutine is initially called with, eg `(usize, String)`.
	type StartArguments = (Rc<IoUring<'static>>, ServerSocketSettings<SA>);

	type ResumeArguments = CompletionResponse;

	type Yields = ();

	type Complete = Result<(), NewSocketServerListenerError>;
	
	const LifetimeHint: LifetimeHint = LifetimeHint::LongLived;
	
	const HeapMemoryAllocatorBlockSizeHint: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(64) };
	
	#[inline(always)]
	fn coroutine(coroutine_instance_handle: CoroutineInstanceHandle, start_arguments: Self::StartArguments, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (io_uring, server_socket_settings) = start_arguments;
		
		let socket_file_descriptor = server_socket_settings.new_socket(HyperThread::current_hyper_thread())?;
		
		Self::main_loop(coroutine_instance_handle, yielder)
	}
}

impl<SA: SocketAddress> AcceptConnectionsCoroutine<SA>
{
	#[inline(always)]
	fn main_loop(coroutine_instance_handle: CoroutineInstanceHandle, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>, socket_file_descriptor: StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor, remote_peer_based_access_control: &Rc<RemotePeerAddressBasedAccessControl<()>>)
	{
		// TODO: The coroutine_instance_handle approach doesnt distinguish the wake up cause; we need more information if (a) using multiple events or (b) using timeouts.
			// TODO: We can steal 1 - 4 bits w/o overly making generation useless.
			// Alternatively we can steal 1 - 4 bits from relative_index; 4.2 bn coroutines aren't needed.
			// IORING_MAX_ENTRIES is 32,768 and completions is double that.
			// Thus we need 15 bits if we were to encode SQE no.
		
		
		
		xxxxx;
		
		
		
		
		loop
		{
			let mut pending_accept_connection = PendingAcceptConnection::new();
			let result = io_uring.push_submission_queue_entry(|submission_queue_entry|
			{
				xxx;
				// TODO: Do we want to do accept4() with non-blocking or not?
				// TODO: We have a full submission queue - now what?
					// TODO: We want to be woken up again WITHOUT any completions due - this might be needed in other coroutines.
				
				xxx; coroutine_instance_handle - now has user bits we can use so we can identify what to wake-up with.
				
				submission_queue_entry.prepare_accept(coroutine_instance_handle, SubmissionQueueEntryOptions::empty(), None, FileDescriptorOrigin::Absolute(&socket_file_descriptor), &mut pending_accept_connection)
			});
			
			use self::SocketAcceptError::*;
			
			match yielder.yields((), ())
			{
				Ok(completion_response) =>
				{
					match completion_response.accept(pending_accept_connection)
					{
						Ok(Some(accepted_connection)) => match remote_peer_based_access_control.is_remote_peer_allowed(&accepted_connection)
						{
							None => (),
							Some(value) =>
							{
								self.publisher.publish_message::<AcceptedStreamingSocketMessage<SD>, _>(logical_core_identifier, self.accepted_streaming_socket_message_compressed_type_identifier, |receiver| AcceptedStreamingSocketMessage::<SD>::initialize(receiver, streaming_socket_file_descriptor, self.streaming_socket_service_identifier));
							}
						}
						
						Ok(None) => panic!("Who cancelled our accept?"),
						
						Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded) => (),
						Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded) => (),
						Err(KernelWouldBeOutOfMemory) => (),
						
						Err(Interrupted) => (),
						
						Err(Again) => panic!("Our socket is supposed to be blocking"),
						
						ConnectionFailed(_) => panic!("Our socket is supposed to be blocking"),
						
						// TODO: Log error if not interrupted or again - we should use structured logging.
					}
				}
				
				Err(()) => return
			}
		}
	}
}

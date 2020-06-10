// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct AcceptCoroutine<SA: SocketAddress>(PhantomData<SA>);

impl<SA: SocketAddress> Coroutine for AcceptCoroutine<SA>
{
	/// Type of the arguments the coroutine is initially called with, eg `(usize, String)`.
	type StartArguments = (Rc<IoUring<'static>>, Queues<(), DequeuedMessageProcessingError>, StreamingServerListenerSocketFileDescriptor<SA::SD>, RemotePeerAddressBasedAccessControl<RemotePeerAddressBasedAccessControlValue>, ServiceProtocolIdentifier, DogStatsDMessageSubscribers);

	type ResumeArguments = AcceptResumeArguments;

	type Yields = AcceptYields;

	type Complete = AcceptComplete;
	
	const LifetimeHint: LifetimeHint = LifetimeHint::LongLived;
	
	const HeapMemoryAllocatorBlockSizeHint: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(64) };
	
	#[inline(always)]
	fn coroutine(coroutine_instance_handle: CoroutineInstanceHandle, start_arguments: Self::StartArguments, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (io_uring, queues, socket_file_descriptor, remote_peer_based_access_control, service_protocol_identifier, dog_stats_d_message_subscribers) = start_arguments;
		
		let default_hyper_thread = HyperThread::current().1;
		
		let accept_publisher = queues.publisher(default_hyper_thread);
		
		let dog_stats_d_publisher = queues.round_robin_publisher(dog_stats_d_message_subscribers);
		
		let mut accept = Accept
		{
			coroutine_instance_handle,
			yielder,
		
			io_uring,
			socket_file_descriptor,
			remote_peer_based_access_control,
			service_protocol_identifier,
			accept_publisher,
			dog_stats_d_publisher,
		};
		
		loop
		{
			// Must be pinned until `yield_awaiting_accept()` returns.
			let mut pending_accept_connection = PendingAcceptConnection::new();
			
			let killed = accept.submit_accept(&mut pending_accept_connection);
			if unlikely!(killed)
			{
				return ()
			}
			
			match accept.yield_awaiting_accept()
			{
				Ok(completion_response) => completion_response,
				Err(()) => return (),
			}
			
			accept.process_accept(completion_response, pending_accept_connection)
		}
	}
}

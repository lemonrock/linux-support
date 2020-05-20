// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
struct AcceptConnectionsCoroutineStartArguments
{
	pub socket_address: SocketAddrV4,
	pub send_buffer_size_in_bytes: usize,
	pub receive_buffer_size_in_bytes: usize,
	pub idles_before_keep_alive_seconds: u16,
	pub keep_alive_interval_seconds: u16,
	pub maximum_keep_alive_probes: u16,
	pub linger_seconds: u16,
	pub linger_in_FIN_WAIT2_seconds: u16,
	pub maximum_SYN_transmits: u16,
	pub back_log: u32,
}

impl AcceptConnectionsCoroutineStartArguments
{
	#[inline(always)]
	fn new_socket(self, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor, NewSocketServerListenerError>
	{
		SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_4_server_listener
		(
			self.socket_address,
			self.send_buffer_size_in_bytes,
			self.receive_buffer_size_in_bytes,
			self.idles_before_keep_alive_seconds,
			self.keep_alive_interval_seconds,
			self.maximum_keep_alive_probes,
			self.linger_seconds,
			self.linger_in_FIN_WAIT2_seconds,
			self.maximum_SYN_transmits,
			self.back_log,
			hyper_thread
		)
	}
}

impl Coroutine for AcceptConnectionsCoroutine
{
	/// Type of the arguments the coroutine is initially called with, eg `(usize, String)`.
	type StartArguments = (Rc<IoUring<'static>>, AcceptConnectionsCoroutineStartArguments);

	type ResumeArguments = CompletionResponse;

	type Yields = ();

	type Complete = Result<(), NewSocketServerListenerError>;
	
	const LifetimeHint: LifetimeHint = LifetimeHint::LongLived;
	
	const HeapMemoryAllocatorBlockSizeHint: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(64) };
	
	#[inline(always)]
	fn coroutine(coroutine_instance_handle: CoroutineInstanceHandle, start_arguments: Self::StartArguments, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete
	{
		let (io_uring, connection_details) = start_arguments;
		
		// TODO: This logic NEEDS TO happen before the coroutine starts.
			// This allows us to drop capabilities on the thread for binding to ports below 1024.
		let socket_file_descriptor = connection_details.new_socket(HyperThread::current_hyper_thread())?;
		
		Self::main_loop(coroutine_instance_handle, yielder)
	}
}

impl AcceptConnectionsCoroutine
{
	#[inline(always)]
	fn main_loop(coroutine_instance_handle: CoroutineInstanceHandle, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>, socket_file_descriptor: StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor, remote_peer_based_access_control: &Rc<RemotePeerAddressBasedAccessControl>)
	{
		// TODO: The coroutine_instance_handle approach doesnt distinguish the wake up cause; we need more information if (a) using multiple events or (b) using timeouts.
			// TODO: We can steal 1 - 4 bits w/o overly making generation useless.
			// Alternatively we can steal 1 - 4 bits from relative_index; 4.2 bn coroutines aren't needed.
		
		loop
		{
			let mut pending_accept_connection = PendingAcceptConnection::new();
			let result = io_uring.push_submission_queue_entry(|submission_queue_entry|
			{
				xxx;
				// TODO: Do we want to do accept4() with non-blocking or not?
				// TODO: We have a full submission queue - now what?
					// TODO: We want to be woken up again WITHOUT any completions due - this might be needed in other coroutines.
				
				submission_queue_entry.prepare_accept(coroutine_instance_handle, SubmissionQueueEntryOptions::empty(), None, FileDescriptorOrigin::Absolute(&socket_file_descriptor), &mut pending_accept_connection)
			});
			
			match yielder.yields((), ())
			{
				Ok(completion_response) =>
				{
					match completion_response.accept(pending_accept_connection)
					{
						Ok(Some(accepted_connection)) =>
						{
							if likely!(remote_peer_based_access_control.is_remote_peer_allowed(accepted_connection.peer_address, accepted_connection.streaming_socket_file_descriptor))
							{
							
							}
							
							self.publisher.publish_message::<AcceptedStreamingSocketMessage<SD>, _>(logical_core_identifier, self.accepted_streaming_socket_message_compressed_type_identifier, |receiver| AcceptedStreamingSocketMessage::<SD>::initialize(receiver, streaming_socket_file_descriptor, self.streaming_socket_service_identifier));
						}
						
						// TODO: Log error if not interrupted or again
						Err(socket_accept_error) => (),
						
						Ok(None) => panic!("Who cancelled our accept?"),
					}
				}
				
				Err(()) => return
			}
		}
	}
}

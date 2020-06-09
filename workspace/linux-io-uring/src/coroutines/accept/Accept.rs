// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct Accept<'yielder, SA: SocketAddress>
{
	coroutine_instance_handle: CoroutineInstanceHandle,
	yielder: Yielder<'yielder, AcceptResumeArguments, AcceptYields, AcceptComplete>,
	
	io_uring: Rc<IoUring<'static>>,
	socket_file_descriptor: StreamingServerListenerSocketFileDescriptor<SA::SD>,
	remote_peer_based_access_control: RemotePeerAddressBasedAccessControl<RemotePeerAddressBasedAccessControlValue>,
	queues: Queues<(), DequeuedMessageProcessingError>,
	service_protocol_identifier: ServiceProtocolIdentifier,
}

impl<'yielder, SA: SocketAddress> Accept<'yielder, SA>
{
	const KillError: () = ();
	
	#[inline(always)]
	fn submit_accept(&mut self, pending_accept_connection: &mut PendingAcceptConnection<SA::SD>) -> bool
	{
		// TODO: We have a full submission queue - now what?
		// TODO: We want to be woken up again WITHOUT any completions due - this might be needed in other coroutines.
		
		let empty1 = SubmissionQueueEntryOptions::empty();
		let origin = FileDescriptorOrigin::Absolute(&self.socket_file_descriptor);
		
		const SubmissionSucceeded: Result<(), ()> = Ok(());
		const SubmissionQueueIsFull: Result<(), ()> = Err(());
		
		loop
		{
			use self::AcceptResumeArguments::*;
			
			match io_uring.push_submission_queue_entry(|submission_queue_entry| submission_queue_entry.prepare_accept(self.coroutine_instance_handle, empty1, None, origin, &mut pending_accept_connection))
			{
				SubmissionSucceeded => return false,
				
				SubmissionQueueIsFull => match self.yielder.yields(AcceptYields::SubmissionQueueFull, Self::KillError)
				{
					Ok(TrySubmissionQueueAgain) => continue,
					
					Ok(Accepted(_, _)) => unreachable!("Logic design flaw"),
					
					Err(_kill_error) => return true
				}
			}
		}
	}
	
	#[inline(always)]
	fn yield_awaiting_accept(&mut self) -> Result<CompletionResponse, ()>
	{
		use self::AcceptResumeArguments::*;
		
		match self.yielder.yields(AcceptYields::AwaitingIoUring, Self::KillError)
		{
			Ok(Accepted(UserBits::Zero, completion_response)) => Ok(completion_response),
			
			Ok(Accepted(_, _)) => unreachable!("Logic design flaw"),
			
			Ok(TrySubmissionQueueAgain) => unreachable!("Logic design flaw"),
			
			Err(_kill_error) => Err(())
		}
	}
	
	#[inline(always)]
	fn process_accept(&mut self, completion_response: CompletionResponse, pending_accept_connection: PendingAcceptConnection<SA::SD>)
	{
		use self::SocketAcceptError::*;
		
		match completion_response.accept(pending_accept_connection)
		{
			Ok(Some(accepted_connection)) => match self.remote_peer_based_access_control.is_remote_peer_allowed(&accepted_connection)
			{
				None =>
				{
					xxxx; // we need a different yield to close an unwanted file descriptor!!!
					// TODO: We need to integrate with Linux firewall iptables to directly allow or deny a connection for an ip address. Oh F--k if we have to use netlink
					
					self.log(AcceptLogEvent { peer_address: accepted_connection.peer_address })
				}
				
				// TODO: CLose file descriptor
				
				Some(value) =>
				{
					
					// NOTE: Slow as it incurs a system call.
					let to_hyper_thread = accepted_connection.streaming_socket_file_descriptor.hyper_thread();
					
					// TODO: THis is very slow - it uses a hashmap when we could cache the compressed type identifier.
					xxxx;
					// TODO: Use `value`.
					self.queues.publish_safe_but_slow(to_hyper_thread, arguments)
					
						// TODO: Forget file descriptor!
					
					
					self.publisher.publish_message::<AcceptedStreamingSocketMessage<SD>, _>(logical_core_identifier, self.accepted_streaming_socket_message_compressed_type_identifier, |receiver| AcceptedStreamingSocketMessage::<SD>::initialize(receiver, streaming_socket_file_descriptor, self.streaming_socket_service_identifier));
				}
			}

			Ok(None) => unreachable!("Logic error: who cancelled our accept?"),
			
			Err(Again) => unreachable!("Our socket is supposed to be blocking"),
			
			ConnectionFailed(reason) => self.log(AcceptLogEvent::ConnectionFailed(reason)),

			Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded) => self.log(AcceptLogEvent::PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
			Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded) => self.log(AcceptLogEvent::SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
			Err(KernelWouldBeOutOfMemory) => self.log(AcceptLogEvent::KernelWouldBeOutOfMemory),

			Err(Interrupted) => (),
		}
	}
	
	fn log(&mut self, log_event: AcceptLogEvent<SA::SD>)
	{
		// IDEA: Use our messages queue to get this off the critical path of this thread and use a logging thread.
		
		// IDEA: it's really not that hard to send RawSpan messages to DataDog using OpenTracing: https://github.com/pipefy/datadog-apm-rust/blob/master/src/client.rs
		
		// IDEA: Also for dogstatsd: https://github.com/mcasper/dogstatsd-rs (eg <https://github.com/mcasper/dogstatsd-rs/blob/master/src/metrics.rs#L3> for the guts of a formatted metric sent over UDP)
			// Can also use Unix domain sockets: `statsd_socket_path` / <https://docs.datadoghq.com/developers/dogstatsd/?tab=hostagent#client-instantiation-parameters>
			// Supports metrics, events, and service checks.
			// * Service Check:  https://docs.datadoghq.com/developers/service_checks/dogstatsd_service_checks_submission/?tab=python#function
		// IDEA: Alternative to DataDog insecure DogStatsD agent is to use code to forward over TLS eg <https://github.com/dirk/metrics_distributor/blob/master/src/forwarders/datadog.rs>
			// See <http://docs.datadoghq.com/api/>
		xxx;
	}
}
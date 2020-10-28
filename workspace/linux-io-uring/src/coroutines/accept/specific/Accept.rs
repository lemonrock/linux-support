// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct Accept<'yielder, SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>>
{
	simple_io_uring_yielder: SimpleIoUringYielder<'yielder, UnusedComplete>,
	socket_file_descriptor: StreamingServerListenerSocketFileDescriptor<SA::SD>,
	access_control: AC,
	service_protocol_identifier: ServiceProtocolIdentifier,
	accept_publisher: AcceptPublisher<SA>,
	socket_hyper_thread: HyperThread,
	
	dog_stats_d: AcceptDogStatsD<CoroutineHeapSize, GTACSA>,
}

impl<'yielder, SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>> Accept<'yielder, SA, CoroutineHeapSize, GTACSA, AC>
{
	#[inline(always)]
	pub(crate) fn new(coroutine_instance_handle: CoroutineInstanceHandle, yielder: Yielder<'yielder, SimpleIoUringResumeArguments, SimpleIoUringYields, UnusedComplete>, start_arguments: AcceptStartArguments<SA, CoroutineHeapSize, GTACSA, AC>) -> Self
	{
		let (io_uring, accept_publisher, socket_file_descriptor, socket_hyper_thread, access_control, service_protocol_identifier, dog_stats_d_start_arguments) = start_arguments;
		
		Self
		{
			simple_io_uring_yielder: SimpleIoUringYielder::new(yielder, io_uring, coroutine_instance_handle),
			socket_file_descriptor,
			access_control,
			service_protocol_identifier,
			accept_publisher,
			socket_hyper_thread,
			
			dog_stats_d: AcceptDogStatsD::new(dog_stats_d_start_arguments, service_protocol_identifier, socket_hyper_thread),
		}
	}
	
	#[inline(always)]
	pub(crate) fn yield_submit_accept(&mut self, pending_accept_connection: &mut PendingAcceptConnection<SA::SD>) -> bool
	{
		let socket_file_descriptor = &self.socket_file_descriptor;
		self.simple_io_uring_yielder.yield_submit_io_uring(|submission_queue_entry, user_data| submission_queue_entry.prepare_accept(user_data, SubmissionQueueEntryOptions::empty(), None, FileDescriptorOrigin::Absolute(socket_file_descriptor), pending_accept_connection))
	}
	
	#[inline(always)]
	pub(crate) fn yield_awaiting_accept(&mut self) -> Result<CompletionResponse, ()>
	{
		self.simple_io_uring_yielder.yield_awaiting_io_uring()
	}
	
	#[inline(always)]
	pub(crate) fn process_accept(&mut self, completion_response: CompletionResponse, pending_accept_connection: PendingAcceptConnection<SA::SD>) -> bool
	{
		use self::SocketAcceptError::*;
		use self::ConnectionFailedReason::*;
		
		macro_rules! log
		{
			($self: ident, $title: literal, $aggregation_key: literal, $priority: ident, $alert_type: ident) =>
			{
				{
					$self.dog_stats_d.log(alert!($title, $aggregation_key, $priority, $alert_type), format_args!(""));
					false
				}
			}
		}
		
		match completion_response.accept(pending_accept_connection)
		{
			Ok(Some(accepted_connection)) => match self.access_control.is_remote_peer_allowed(&accepted_connection)
			{
				Some(value) => self.use_wanted_connection(accepted_connection, value),
				
				None => self.close_unwanted_connection(accepted_connection),
			}

			Ok(None) => unreachable_code(format_args!("Logic error: who cancelled our accept?")),

			Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded) => log!(self, "PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded", "PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded", Normal, Error),
			
			Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded) => log!(self, "SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded", "SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded", Normal, Error),
			
			Err(KernelWouldBeOutOfMemory) => log!(self, "KernelWouldBeOutOfMemory", "KernelWouldBeOutOfMemory", Normal, Error),
			
			Err(Again) => unreachable_code(format_args!("Our socket is supposed to be blocking")),
			
			Err(Interrupted) => log!(self, "Interrupted", "Interrupted", Low, Informational),
			
			Err(ConnectionFailed(Aborted)) => log!(self, "ConnectionFailed::Aborted", "ConnectionFailed", Low, Informational),
			
			Err(ConnectionFailed(FirewallPermissionDenied)) => log!(self, "ConnectionFailed::FirewallPermissionDenied", "ConnectionFailed", Low, Informational),
			
			Err(ConnectionFailed(TimedOut)) => log!(self, "ConnectionFailed::TimedOut", "ConnectionFailed", Low, Informational),
			
			Err(ConnectionFailed(Protocol)) => log!(self, "ConnectionFailed::Protocol", "ConnectionFailed", Low, Informational),
		}
	}
	
	#[inline(always)]
	pub(crate) fn use_wanted_connection(&self, accepted_connection: AcceptedConnection<SA::SD>, value: &Arc<AccessControlValue>) -> bool
	{
		let processing_hyper_thread = self.accept_publisher.publish(self.socket_hyper_thread, accepted_connection, self.service_protocol_identifier, value);
		self.dog_stats_d.increment_connection_count(processing_hyper_thread);
		false
	}
	
	#[inline(always)]
	pub(crate) fn close_unwanted_connection(&mut self, accepted_connection: AcceptedConnection<SA::SD>) -> bool
	{
		self.dog_stats_d.log(alert!("ConnectionDenied", "ConnectionDenied", Low, Informational), format_args!("{}", accepted_connection.peer));
		
		let streaming_socket_file_descriptor = accepted_connection.streaming_socket_file_descriptor;
		
		let killed = self.yield_submit_close(&streaming_socket_file_descriptor);
		if unlikely!(killed)
		{
			return true
		}
		
		let completion_response = match self.yield_awaiting_close()
		{
			Ok(completion_response) => completion_response,
			Err(()) => return true,
		};
		
		self.process_close(completion_response, streaming_socket_file_descriptor)
	}
	
	#[inline(always)]
	pub(crate) fn yield_submit_close(&mut self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SA::SD>) -> bool
	{
		self.simple_io_uring_yielder.yield_submit_io_uring(|submission_queue_entry, user_data| submission_queue_entry.prepare_close(user_data, SubmissionQueueEntryOptions::empty(), None, streaming_socket_file_descriptor))
	}
	
	#[inline(always)]
	pub(crate) fn yield_awaiting_close(&mut self) -> Result<CompletionResponse, ()>
	{
		self.simple_io_uring_yielder.yield_awaiting_io_uring()
	}
	
	#[inline(always)]
	pub(crate) fn process_close(&self, completion_response: CompletionResponse, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SA::SD>) -> bool
	{
		forget(streaming_socket_file_descriptor);
		
		match completion_response.close()
		{
			Some(false) => (),
			
			Some(true) => self.dog_stats_d.log(alert!("ConnectionDenied::CloseError", "ConnectionDenied", Low, Informational), format_args!("")),
			
			None => unreachable_code(format_args!("Logic error: who cancelled our close?")),
		}
		
		false
	}
}

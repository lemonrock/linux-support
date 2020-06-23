// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct Accept<'yielder, SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>>
{
	coroutine_instance_handle: CoroutineInstanceHandle,
	yielder: Yielder<'yielder, AcceptResumeArguments, AcceptYields, AcceptComplete>,
	io_uring: Rc<IoUring<'static>>,
	socket_file_descriptor: StreamingServerListenerSocketFileDescriptor<SA::SD>,
	access_control: AC,
	service_protocol_identifier: ServiceProtocolIdentifier,
	accept_publisher: AcceptPublisher<SA>,
	dog_stats_d_publisher: DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
	thread_local_socket_hyper_thread_additional_dog_stats_d_cache: Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	thread_local_processing_hyper_thread_additional_dog_stats_d_cache: Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	service_protocol_additional_dog_stats_d_tag: AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>,
}

impl<'yielder, SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>> Accept<'yielder, SA, CoroutineHeapSize, GTACSA, AC>
{
	#[inline(always)]
	fn new(coroutine_instance_handle: CoroutineInstanceHandle, start_arguments: AcceptStartArguments<SA, CoroutineHeapSize, GTACSA, AC>, yielder: Yielder<'yielder, AcceptResumeArguments, AcceptYields, AcceptComplete>) -> Self
	{
		let (io_uring, accept_publisher, socket_file_descriptor, access_control, service_protocol_identifier, dog_stats_d_publisher,  global_allocator, thread_local_socket_hyper_thread_additional_dog_stats_d_cache, thread_local_processing_hyper_thread_additional_dog_stats_d_cache) = start_arguments;
		
		Self
		{
			coroutine_instance_handle,
			yielder,
			io_uring,
			socket_file_descriptor,
			access_control,
			service_protocol_identifier,
			accept_publisher,
			dog_stats_d_publisher,
			thread_local_socket_hyper_thread_additional_dog_stats_d_cache,
			thread_local_processing_hyper_thread_additional_dog_stats_d_cache,
			service_protocol_additional_dog_stats_d_tag: AdditionalDogStatsDTag::from(DogStatsDTag::from_u8("service_protocol", service_protocol_identifier).unwrap(), global_allocator),
		}
	}
	
	#[inline(always)]
	fn yield_submit_accept(&self, pending_accept_connection: &mut PendingAcceptConnection<SA::SD>) -> bool
	{
		let mut entry = |submission_queue_entry: SubmissionQueueEntry| submission_queue_entry.prepare_accept(self.user_data_for_io_uring_operation_0(), Self::NoSubmissionOptions, None, FileDescriptorOrigin::Absolute(&self.socket_file_descriptor), pending_accept_connection);
		AcceptYields::yield_submit_io_uring(&self.yielder, &self.io_uring, &mut entry)
	}
	
	#[inline(always)]
	fn yield_awaiting_accept(&self) -> Result<CompletionResponse, ()>
	{
		AcceptYields::yield_awaiting_io_uring(&self.yielder)
	}
	
	#[inline(always)]
	fn process_accept(&self, completion_response: CompletionResponse, pending_accept_connection: PendingAcceptConnection<SA::SD>) -> bool
	{
		use self::SocketAcceptError::*;
		use self::ConnectionFailedReason::*;
		
		macro_rules! log
		{
			($self: ident, $title: literal, $aggregation_key: literal, $priority: ident, $alert_type: ident) =>
			{
				{
					$self.log(alert!($title, $aggregation_key, $priority, $alert_type), format_args!(""));
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

			Ok(None) => unreachable!("Logic error: who cancelled our accept?"),

			Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded) => log!(self, "PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded", "PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded", Normal, Error),
			
			Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded) => log!(self, "SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded", "SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded", Normal, Error),
			
			Err(KernelWouldBeOutOfMemory) => log!(self, "KernelWouldBeOutOfMemory", "KernelWouldBeOutOfMemory", Normal, Error),
			
			Err(Again) => unreachable!("Our socket is supposed to be blocking"),
			
			Err(Interrupted) => log!(self, "Interrupted", "Interrupted", Low, Informational),
			
			Err(ConnectionFailed(Aborted)) => log!(self, "ConnectionFailed::Aborted", "ConnectionFailed", Low, Informational),
			
			Err(ConnectionFailed(FirewallPermissionDenied)) => log!(self, "ConnectionFailed::FirewallPermissionDenied", "ConnectionFailed", Low, Informational),
			
			Err(ConnectionFailed(TimedOut)) => log!(self, "ConnectionFailed::TimedOut", "ConnectionFailed", Low, Informational),
			
			Err(ConnectionFailed(Protocol)) => log!(self, "ConnectionFailed::Protocol", "ConnectionFailed", Low, Informational),
		}
	}
	
	#[inline(always)]
	fn use_wanted_connection(&self, accepted_connection: AcceptedConnection<SA::SD>, value: &Arc<AccessControlValue>) -> bool
	{
		let socket_hyper_thread = accepted_connection.streaming_socket_file_descriptor.hyper_thread();
		let processing_hyper_thread = self.accept_publisher.publish(socket_hyper_thread, accepted_connection, self.service_protocol_identifier, value);
		self.increment_connection_count(socket_hyper_thread, processing_hyper_thread);
		false
	}
	
	#[inline(always)]
	fn close_unwanted_connection(&self, accepted_connection: AcceptedConnection<SA::SD>) -> bool
	{
		self.log(alert!("ConnectionDenied", "ConnectionDenied", Low, Informational), format_args!("{}", accepted_connection.peer));
		
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
	fn yield_submit_close(&self, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<SA::SD>) -> bool
	{
		let mut entry = |submission_queue_entry: SubmissionQueueEntry| submission_queue_entry.prepare_close(self.user_data_for_io_uring_operation_0(), Self::NoSubmissionOptions, None, streaming_socket_file_descriptor);
		AcceptYields::yield_submit_io_uring(&self.yielder, &self.io_uring, &mut entry)
	}
	
	#[inline(always)]
	fn yield_awaiting_close(&self) -> Result<CompletionResponse, ()>
	{
		AcceptYields::yield_awaiting_io_uring(&self.yielder)
	}
	
	#[inline(always)]
	fn process_close(&self, completion_response: CompletionResponse, streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SA::SD>) -> bool
	{
		forget(streaming_socket_file_descriptor);
		
		match completion_response.close()
		{
			Some(false) => (),
			
			Some(true) => self.log(alert!("ConnectionDenied::CloseError", "ConnectionDenied", Low, Informational), format_args!("")),
			
			None => unreachable!("Logic error: who cancelled our close?"),
		}
		
		false
	}
	
	#[inline(always)]
	fn log(&self, alert: &'static EventTemplate, message: Arguments)
	{
		let additional_tags = additional_dog_stats_d_tags!
		[
			self.service_protocol_additional_dog_stats_d_tag()
		];
		self.dog_stats_d_publisher.log(alert, additional_tags, message)
	}
	
	#[inline(always)]
	fn increment_connection_count(&self, socket_hyper_thread: HyperThread, processing_hyper_thread: HyperThread)
	{
		#[thread_local] static IncrementConnectionCount: MetricTemplate = MetricTemplate::new_with_common_tags("connection.count");
		
		self.dog_stats_d_publisher.publish
		(
			IncrementConnectionCount.message
			(
				additional_dog_stats_d_tags!
				[
					self.service_protocol_additional_dog_stats_d_tag(),
					self.thread_local_socket_hyper_thread_additional_dog_stats_d_cache.get(socket_hyper_thread),
					self.thread_local_processing_hyper_thread_additional_dog_stats_d_cache.get(processing_hyper_thread)
				],
				MetricValue::IncrementUnsampled
			)
		)
	}
	
	#[inline(always)]
	fn service_protocol_additional_dog_stats_d_tag(&self) -> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
	{
		self.service_protocol_additional_dog_stats_d_tag.clone()
	}
	
	#[inline(always)]
	fn user_data_for_io_uring_operation_0(&self) -> u64
	{
		self.coroutine_instance_handle.unwrap()
	}
	
	const NoSubmissionOptions: SubmissionQueueEntryOptions = SubmissionQueueEntryOptions::empty();
}

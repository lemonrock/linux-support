// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Coroutine managers partial abstraction.
pub struct CoroutineManagers<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, StackSizeAccept: MemorySize>
(
	CoroutineManager<CoroutineHeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<sockaddr_in, CoroutineHeapSize, GTACSA>, AcceptCoroutineInformation>,
	CoroutineManager<CoroutineHeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<sockaddr_in6, CoroutineHeapSize, GTACSA>, AcceptCoroutineInformation>,
	CoroutineManager<CoroutineHeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<UnixSocketAddress<PathBuf>, CoroutineHeapSize, GTACSA>, AcceptCoroutineInformation>,
);

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, StackSizeAccept: MemorySize> CoroutineManagers<CoroutineHeapSize, GTACSA, StackSizeAccept>
{
	pub fn new
	(
		global_allocator: &'static GTACSA,
		defaults: &DefaultPageSizeAndHugePageSizes,
		io_uring: &Rc<IoUring<'static>>,
		queues: &Queues<(), DequeuedMessageProcessingError>,
		our_hyper_thread: HyperThread,
		dog_stats_d_publisher: &DogStatsDPublisher<CoroutineHeapSize, GTACSA, MessageHandlerArguments>,
		thread_local_socket_hyper_thread_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		thread_local_processing_hyper_thread_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in>>,
		transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in6>>,
		streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>>>,
	) -> Result<Self, ThreadLoopInitializationError>
	{
		let factory = AcceptCoroutineManagerFactory::new
		(
			global_allocator,
			defaults,
			io_uring,
			queues,
			dog_stats_d_publisher,
			thread_local_socket_hyper_thread_additional_dog_stats_d_cache,
			thread_local_processing_hyper_thread_additional_dog_stats_d_cache,
			our_hyper_thread,
		);
		
		Ok
		(
			Self
			(
				factory.create_and_start(0, transmission_control_protocol_over_internet_protocol_version_4_server_listeners)?,
				factory.create_and_start(1, transmission_control_protocol_over_internet_protocol_version_6_server_listeners)?,
				factory.create_and_start(2, streaming_unix_domain_socket_server_listener_server_listeners)?,
			)
		)
	}
	
	pub fn dispatch_retry_because_io_uring_submission_queue_was_full(&mut self, coroutine_instance_handle: CoroutineInstanceHandle) -> CoroutineRequiresReEntry
	{
		choose_coroutine_manager!
		{
			coroutine_instance_handle.coroutine_manager_index(),
			dispatch_retry_because_io_uring_submission_queue_was_full,
			coroutine_instance_handle,
			self,
			0 => 0,
			1 => 1,
			2 => 2,
		}
	}
	
	pub fn dispatch_io_uring<NonCoroutineHandler: FnMut(u64, CompletionResponse) -> Result<(), NonCoroutineHandlerError>, NonCoroutineHandlerError: error::Error + Into<DispatchIoUringError<NonCoroutineHandlerError>>>(&mut self, completion_queue_entry: CompletionQueueEntry, mut non_coroutine_handler: &mut NonCoroutineHandler) -> Result<CoroutineRequiresReEntry, DispatchIoUringError<NonCoroutineHandlerError>>
	{
		let completion_response = completion_queue_entry.completion_response();
		let user_data = completion_queue_entry.user_data();
		if CoroutineInstanceHandle::is_not_for_a_coroutine(user_data)
		{
			non_coroutine_handler(user_data, completion_response)?;
			return Ok(CoroutineRequiresReEntry::CarryOn)
		}
		
		let coroutine_instance_handle = CoroutineInstanceHandle::wrap(user_data);
		
		choose_coroutine_manager!
		{
			coroutine_instance_handle.coroutine_manager_index(),
			dispatch_io_uring,
			(coroutine_instance_handle, completion_response),
			self,
			0 => 0,
			1 => 1,
			2 => 2,
		}
	}
}

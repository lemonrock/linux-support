// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct AcceptCoroutineManagerFactory<'a, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize>
{
	global_allocator: &'static GTACSA,
	defaults: &'a DefaultPageSizeAndHugePageSizes,
	io_uring: &'a Rc<IoUring<'static>>,
	queues: &'a Queues<(), DequeuedMessageProcessingError>,
	dog_stats_d_publisher: &'a DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
	thread_local_socket_hyper_thread_additional_dog_stats_d_cache: &'a Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	thread_local_processing_hyper_thread_additional_dog_stats_d_cache: &'a Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	our_hyper_thread: HyperThread,
	marker: PhantomData<AcceptStackSize>,
}

impl<'a, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize> AcceptCoroutineManagerFactory<'a, CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	#[inline(always)]
	pub(crate) fn new
	(
		global_allocator: &'static GTACSA,
		defaults: &'a DefaultPageSizeAndHugePageSizes,
		io_uring: &'a Rc<IoUring<'static>>,
		queues: &'a Queues<(), DequeuedMessageProcessingError>,
		dog_stats_d_publisher: &'a DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
		thread_local_socket_hyper_thread_additional_dog_stats_d_cache: &'a Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		thread_local_processing_hyper_thread_additional_dog_stats_d_cache: &'a Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		our_hyper_thread: HyperThread,
	) -> Self
	{
		Self
		{
			global_allocator,
			defaults,
			io_uring,
			queues,
			dog_stats_d_publisher,
			thread_local_socket_hyper_thread_additional_dog_stats_d_cache,
			thread_local_processing_hyper_thread_additional_dog_stats_d_cache,
			our_hyper_thread,
			marker: PhantomData
		}
	}
	
	pub(crate) fn create_and_start<SA: SocketAddress, AC: AccessControl<SA::SD, AccessControlValue>>(&self, coroutine_manager_index: u8, transmission_control_protocol_server_listener_settings: Vec<AcceptConnectionsCoroutineSettings<SA, AC>>) -> Result<AcceptCoroutineManager<SA, CoroutineHeapSize, GTACSA, AC, AcceptStackSize>, ThreadLoopInitializationError>
	{
		let length = transmission_control_protocol_server_listener_settings.len();
		let ideal_maximum_number_of_coroutines = if length == 0
		{
			unsafe { NonZeroU64::new_unchecked(1) }
		}
		else
		{
			unsafe { NonZeroU64::new_unchecked(length as u64) }
		};
		
		let mut coroutine_manager = CoroutineManager::new(CoroutineManagerIndex(coroutine_manager_index), self.global_allocator, ideal_maximum_number_of_coroutines, self.defaults).map_err(ThreadLoopInitializationError::AcceptConnectionsCoroutineManager)?;
		
		let accept_publisher = AcceptPublisher::new(&self.queues, self.our_hyper_thread);
		for AcceptConnectionsCoroutineSettings { transmission_control_protocol_service_listener_settings, access_control, service_protocol_identifier} in transmission_control_protocol_server_listener_settings
		{
			let start_arguments: AcceptStartArguments<SA, CoroutineHeapSize, GTACSA, AC> =
			(
				self.io_uring.clone(),
				accept_publisher.clone(),
				transmission_control_protocol_service_listener_settings.new_socket(self.our_hyper_thread)?,
				access_control,
				service_protocol_identifier,
				self.dog_stats_d_publisher.clone(),
				self.global_allocator,
				self.thread_local_socket_hyper_thread_additional_dog_stats_d_cache.clone(),
				self.thread_local_processing_hyper_thread_additional_dog_stats_d_cache.clone(),
			);
			
			let result = coroutine_manager.start_coroutine(AcceptCoroutineInformation, start_arguments);
			AcceptYields::yield_start(result)?;
		}
		
		Ok(coroutine_manager)
	}
}

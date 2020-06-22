// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct AcceptCoroutineManagerFactory<'a, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, StackSizeAccept: MemorySize>
{
	global_allocator: &'static GTACSA,
	defaults: &'a DefaultPageSizeAndHugePageSizes,
	io_uring: &'a Rc<IoUring<'static>>,
	queues: &'a Queues<(), DequeuedMessageProcessingError>,
	dog_stats_d_publisher: &'a DogStatsDPublisher<CoroutineHeapSize, GTACSA, MessageHandlerArguments>,
	thread_local_socket_hyper_thread_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	thread_local_processing_hyper_thread_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	our_hyper_thread: HyperThread,
	marker: PhantomData<StackSizeAccept>,
}

impl<'a, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, StackSizeAccept: MemorySize> AcceptCoroutineManagerFactory<'a, CoroutineHeapSize, GTACSA, StackSizeAccept>
{
	#[inline(always)]
	pub(crate) fn new
	(
		global_allocator: &'static GTACSA,
		defaults: &'a DefaultPageSizeAndHugePageSizes,
		io_uring: &'a Rc<IoUring<'static>>,
		queues: &'a Queues<(), DequeuedMessageProcessingError>,
		dog_stats_d_publisher: &'a DogStatsDPublisher<CoroutineHeapSize, GTACSA, MessageHandlerArguments>,
		thread_local_socket_hyper_thread_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		thread_local_processing_hyper_thread_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
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
	
	pub(crate) fn create_and_start<SA: SocketAddress>(&self, coroutine_manager_index: u8, transmission_control_protocol_server_listener_settings: Vec<AcceptConnectionsCoroutineSettings<SA>>) -> Result<AcceptCoroutineManager<SA, CoroutineHeapSize, GTACSA, StackSizeAccept>, ThreadLoopInitializationError>
	{
		let length = transmission_control_protocol_server_listener_settings.len();
		let ideal_maximum_number_of_coroutines = if length == 0
		{
			unsafe { NonZeroU64::new_unchecked(1) }
		}
		else
		{
			unsafe { NonZeroU64::new_unchecked(length as usize) }
		};
		
		let coroutine_manager = CoroutineManager::new(CoroutineManagerIndex(coroutine_manager_index), self.global_allocator, ideal_maximum_number_of_coroutines, self.defaults).map_err(ThreadLoopInitializationError::AcceptConnectionsCoroutineManager)?;
		
		let accept_publisher = AcceptPublisher::new(&self.queues, self.our_hyper_thread);
		for settings in transmission_control_protocol_server_listener_settings
		{
			let start_arguments: AcceptStartArguments<SA, CoroutineHeapSize, GTACSA> =
			(
				self.io_uring.clone(),
				accept_publisher.clone(),
				settings.new_socket()?,
				settings.remote_peer_adddress_based_access_control(),
				settings.service_protocol_identifier(),
				self.dog_stats_d_publisher.clone(),
				self.global_allocator,
				self.thread_local_socket_hyper_thread_additional_dog_stats_d_cache.clone(),
				self.thread_local_processing_hyper_thread_additional_dog_stats_d_cache.clone(),
				self.our_hyper_thread,
			);
			
			let result = coroutine_manager.start_coroutine(AcceptCoroutineInformation, start_arguments);
			AcceptYields::yield_start(result)?;
		}
		
		Ok(coroutine_manager)
	}
}

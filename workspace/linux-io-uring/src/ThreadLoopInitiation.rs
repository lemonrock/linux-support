// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Initiation.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct ThreadLoopInitiation<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: 'static + MemorySize>
{
	pub defaults: DefaultPageSizeAndHugePageSizes,
	pub global_allocator: &'static GTACSA,
	pub queues: Queues<(), DequeuedMessageProcessingError>,
	pub signal_mask: Signals,
	pub dog_stats_d_message_subscribers: DogStatsDMessageSubscribers,
	
	pub io_uring_settings: IoUringSettings,
	pub transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<SocketAddrV4, InternetProtocolVersion4AccessControl<AccessControlValue>>>,
	pub transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<SocketAddrV6, InternetProtocolVersion6AccessControl<AccessControlValue>>>,
	pub streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>, UnixDomainSocketAccessControl<AccessControlValue>>>,

	marker: PhantomData<(CoroutineHeapSize, AcceptStackSize)>,
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: 'static + MemorySize> Clone for ThreadLoopInitiation<CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			defaults: self.defaults.clone(),
			global_allocator: self.global_allocator,
			queues: self.queues.clone(),
			signal_mask: self.signal_mask.clone(),
			dog_stats_d_message_subscribers: self.dog_stats_d_message_subscribers.clone(),
			io_uring_settings: self.io_uring_settings.clone(),
			transmission_control_protocol_over_internet_protocol_version_4_server_listeners: self.transmission_control_protocol_over_internet_protocol_version_4_server_listeners.clone(),
			transmission_control_protocol_over_internet_protocol_version_6_server_listeners: self.transmission_control_protocol_over_internet_protocol_version_6_server_listeners.clone(),
			streaming_unix_domain_socket_server_listener_server_listeners: self.streaming_unix_domain_socket_server_listener_server_listeners.clone(),
			marker: PhantomData,
		}
	}
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: 'static + MemorySize> ThreadFunction for ThreadLoopInitiation<CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	type TLBF = ThreadLoop<CoroutineHeapSize, GTACSA, AcceptStackSize>;
	
	fn initialize(self) -> Self::TLBF
	{
		self.initialize_internal().expect("Could not initialize")
	}
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: 'static + MemorySize> ThreadLoopInitiation<CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	/// New instance.
	#[inline(always)]
	pub fn new
	(
		defaults: DefaultPageSizeAndHugePageSizes,
		global_allocator: &'static GTACSA,
		queues: Queues<(), DequeuedMessageProcessingError>,
		signal_mask: Signals,
		dog_stats_d_message_subscribers: DogStatsDMessageSubscribers,
		
		io_uring_settings: IoUringSettings,
		transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<SocketAddrV4, InternetProtocolVersion4AccessControl<AccessControlValue>>>,
		transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<SocketAddrV6, InternetProtocolVersion6AccessControl<AccessControlValue>>>,
		streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>, UnixDomainSocketAccessControl<AccessControlValue>>>,
	) -> Self
	{
		Self
		{
			defaults,
			global_allocator,
			queues,
			signal_mask,
			dog_stats_d_message_subscribers,
			
			io_uring_settings,
			transmission_control_protocol_over_internet_protocol_version_4_server_listeners,
			transmission_control_protocol_over_internet_protocol_version_6_server_listeners,
			streaming_unix_domain_socket_server_listener_server_listeners,
		
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	fn initialize_internal(self) -> Result<ThreadLoop<CoroutineHeapSize, GTACSA, AcceptStackSize>, ThreadLoopInitializationError>
	{
		let (io_uring, registered_buffers) = self.io_uring_settings.setup(&self.defaults)?;
		
		let signal_file_descriptor = Self::signals(self.signal_mask)?;
		
		let our_hyper_thread = HyperThread::current().1;
		
		let dog_stats_d_publisher = DogStatsDPublisher::new(&self.queues, self.dog_stats_d_message_subscribers, self.global_allocator);
		let thread_local_socket_hyper_thread_additional_dog_stats_d_cache = ThreadLocalNumericAdditionalDogStatsDTagsCache::new("local_socket_hyper_thread", self.global_allocator);
		let thread_local_processing_hyper_thread_additional_dog_stats_d_cache = ThreadLocalNumericAdditionalDogStatsDTagsCache::new("processing_socket_hyper_thread", self.global_allocator);
		
		let subscriber = self.queues.subscriber(our_hyper_thread);

		let coroutine_managers = CoroutineManagers::new
		(
			self.global_allocator,
			&self.defaults,
			&io_uring,
			&self.queues,
			our_hyper_thread,
			&dog_stats_d_publisher,
			&thread_local_socket_hyper_thread_additional_dog_stats_d_cache,
			&thread_local_processing_hyper_thread_additional_dog_stats_d_cache,
			self.transmission_control_protocol_over_internet_protocol_version_4_server_listeners,
			self.transmission_control_protocol_over_internet_protocol_version_6_server_listeners,
			self.streaming_unix_domain_socket_server_listener_server_listeners,
		)?;
		
		Ok
		(
			ThreadLoop
			{
				io_uring,
				registered_buffers,
				signal_file_descriptor,
				our_hyper_thread,
				coroutine_managers,
				retry_submission_queue_was_full_coroutine_instance_handle: None,
				dog_stats_d_publisher,
				subscriber,
			}
		)
	}
	
	#[inline(always)]
	fn signals(signal_mask: Signals) -> Result<SignalFileDescriptor, ThreadLoopInitializationError>
	{
		signal_mask.block_all_signals_on_current_thread_bar();
		Ok(SignalFileDescriptor::new(&signal_mask.to_sigset_t()).map_err(ThreadLoopInitializationError::SignalFileDescriptor)?)
	}
}

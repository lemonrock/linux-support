// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Coroutine managers partial abstraction.
#[derive(Debug)]
pub(crate) struct ThreadLocalCoroutineManagers<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize>
{
	_0: ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, AcceptStackSize, AcceptCoroutine<SocketAddrV4, CoroutineHeapSize, GTACSA, InternetProtocolVersion4AccessControl<AccessControlValue>>>,
	_1: ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, AcceptStackSize, AcceptCoroutine<SocketAddrV6, CoroutineHeapSize, GTACSA, InternetProtocolVersion6AccessControl<AccessControlValue>>>,
	_2: ServerLoopCoroutineManager<CoroutineHeapSize, GTACSA, AcceptStackSize, AcceptCoroutine<UnixSocketAddress<PathBuf>, CoroutineHeapSize, GTACSA, UnixDomainSocketAccessControl<AccessControlValue>>>,
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize> ThreadLocalCoroutineManagers<CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	pub(crate) fn new
	(
		global_allocator: &'static GTACSA,
		defaults: &DefaultHugePageSizes,
		io_uring: &Rc<IoUring<'static>>,
		queues: &Queues<(), DequeuedMessageProcessingError>,
		our_hyper_thread: HyperThread,
		dog_stats_d_publisher: &DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
		socket_hyper_thread_thread_local_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		processing_hyper_thread_thread_local_additional_dog_stats_d_cache: &Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
		transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<SocketAddrV4, InternetProtocolVersion4AccessControl<AccessControlValue>>>,
		transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<SocketAddrV6, InternetProtocolVersion6AccessControl<AccessControlValue>>>,
		streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>, UnixDomainSocketAccessControl<AccessControlValue>>>,
	) -> Result<Self, ThreadLoopInitializationError>
	{
		let factory = AcceptCoroutineManagerFactory::new
		(
			global_allocator,
			defaults,
			io_uring,
			queues,
			dog_stats_d_publisher,
			socket_hyper_thread_thread_local_additional_dog_stats_d_cache,
			processing_hyper_thread_thread_local_additional_dog_stats_d_cache,
			our_hyper_thread,
		);
		
		Ok
		(
			Self
			{
				_0: factory.create_and_start(0, transmission_control_protocol_over_internet_protocol_version_4_server_listeners)?,
				_1: factory.create_and_start(1, transmission_control_protocol_over_internet_protocol_version_6_server_listeners)?,
				_2: factory.create_and_start(2, streaming_unix_domain_socket_server_listener_server_listeners)?,
			}
		)
	}
	
	pub(crate) fn dispatch_retry_because_io_uring_submission_queue_was_full(&mut self, coroutine_instance_handle: CoroutineInstanceHandle) -> CoroutineRequiresReEntry
	{
		choose_coroutine_manager!
		{
			coroutine_instance_handle.coroutine_manager_index(),
			dispatch_retry_because_io_uring_submission_queue_was_full,
			coroutine_instance_handle,
			self,
			0 => _0,
			1 => _1,
			2 => _2,
		}
	}
	
	pub(crate) fn dispatch_io_uring(&mut self, user_data: u64, completion_response: CompletionResponse) -> CoroutineRequiresReEntry
	{
		let coroutine_instance_handle = CoroutineInstanceHandle::wrap(user_data);
		
		choose_coroutine_manager!
		{
			coroutine_instance_handle.coroutine_manager_index(),
			dispatch_io_uring,
			(coroutine_instance_handle, completion_response),
			self,
			0 => _0,
			1 => _1,
			2 => _2,
		}
	}
}

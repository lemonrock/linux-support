// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Initiation.
pub struct ThreadLoopInitiation<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	defaults: DefaultPageSizeAndHugePageSizes,
	global_allocator: &'static GTACSA,
	queues: Queues<(), DequeuedMessageProcessingError>,
	io_uring_settings: IoUringSettings,
	signal_mask: BitSet<Signal>,
	
	transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in>>,
	transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in6>>,
	streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>>>,
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ThreadFunction for ThreadLoopInitiation<HeapSize, StackSize, GTACSA>
{
	type TLBF = ThreadLoop<HeapSize, StackSize, GTACSA>;
	
	fn initialize(self) -> Self::TLBF
	{
		self.initialize_internal().expect("Could not initialize")
	}
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ThreadLoopInitiation<HeapSize, StackSize, GTACSA>
{
	#[inline(always)]
	fn initialize_internal(self) -> Result<Self::TLBF, ThreadLoopInitializationError>
	{
		use self::ThreadLoopInitializationError::*;
		
		let (io_uring, registered_buffers) = self.io_uring_settings.setup(&self.defaults)?;
		
		let signal_file_descriptor = self.signals()?;
		
		let transmission_control_protocol_over_internet_protocol_version_4_server_listener_coroutine_manager = self.new_server_listener_coroutine_manager(&mut self.transmission_control_protocol_over_internet_protocol_version_4_server_listeners)?;
		let transmission_control_protocol_over_internet_protocol_version_6_server_listener_coroutine_manager = self.new_server_listener_coroutine_manager(&mut self.transmission_control_protocol_over_internet_protocol_version_6_server_listeners)?;
		let streaming_unix_domain_socket_server_listener_coroutine_manager = self.new_server_listener_coroutine_manager(&mut self.streaming_unix_domain_socket_server_listener_server_listeners)?;
		
		Ok
		(
			ThreadLoop
			{
				global_allocator: self.global_allocator,
				io_uring,
				registered_buffers,
				signal_file_descriptor,
				our_hyper_thread: HyperThread::current().1,
				queues: self.queues,
				transmission_control_protocol_over_internet_protocol_version_4_server_listener_coroutine_manager,
				transmission_control_protocol_over_internet_protocol_version_6_server_listener_coroutine_manager,
				streaming_unix_domain_socket_server_listener_coroutine_manager,
			}
		)
	}
	
	#[inline(always)]
	fn signals(self) -> Result<SignalFileDescriptor, ThreadLoopInitializationError>
	{
		self.signal_mask.block_all_signals_on_current_thread_bar();
		Ok(SignalFileDescriptor::new(&self.signal_mask.to_sigset_t()).map_err(ThreadLoopInitializationError::SignalFileDescriptor)?)
	}
	
	#[inline(always)]
	fn new_server_listener_coroutine_manager<C: Coroutine, CoroutineInformation: Sized, SA: SocketAddress>(&self, server_sockets_settings: &mut Vec<AcceptConnectionsCoroutineSettings<SA>>) -> Result<Option<(CoroutineManager<HeapSize, StackSize, GTACSA, C, CoroutineInformation>, Vec<StreamingServerListenerSocketFileDescriptor<SA::SD>>)>, ThreadLoopInitializationError>
	{
		use self::ThreadLoopInitializationError::*;
		
		let length = server_sockets_settings.len();
		if length == 0
		{
			Ok(None)
		}
		else
		{
			match CoroutineManager::new(self.global_allocator, unsafe { NonZeroU64::new_unchecked(length as usize) }, &self.defaults)
			{
				Err(error) => Err(AcceptConnectionsCoroutineManager(error)),
				
				Ok(coroutine_manager) =>
				{
					let mut server_sockets = Vec::with_capacity(length);
					for server_socket_settings in server_sockets_settings.drain(..)
					{
						// TODO: These are going to become coroutines
						xxxx;
						server_sockets.push(server_socket_settings.new_socket()?)
					}
					
					Ok(Some((coroutine_manager, server_sockets)))
				},
			}
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Initiation.
#[allow(missing_docs)]
#[derive(Clone)]
pub struct ThreadLoopInitiation<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	pub defaults: DefaultPageSizeAndHugePageSizes,
	pub global_allocator: &'static GTACSA,
	pub queues: Queues<(), DequeuedMessageProcessingError>,
	pub io_uring_settings: IoUringSettings,
	pub signal_mask: Signals,
	
	pub transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in>>,
	pub transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in6>>,
	pub streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>>>,
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> ThreadFunction for ThreadLoopInitiation<CoroutineHeapSize, GTACSA>
{
	type TLBF = ThreadLoop<CoroutineHeapSize, StackSize, GTACSA>;
	
	fn initialize(self) -> Self::TLBF
	{
		self.initialize_internal().expect("Could not initialize")
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> ThreadLoopInitiation<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn initialize_internal(self) -> Result<Self::TLBF, ThreadLoopInitializationError>
	{
		use self::ThreadLoopInitializationError::*;
		
		let (io_uring, registered_buffers) = self.io_uring_settings.setup(&self.defaults)?;
		
		let signal_file_descriptor = self.signals()?;
		
		let coroutine_managers = CoroutineManagers::new
		(
			self.global_allocator,
			&self.defaults,
			&io_uring,
			&self.queues,
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
				our_hyper_thread: HyperThread::current().1,
				queues: self.queues,
				coroutine_managers,
			}
		)
	}
	
	#[inline(always)]
	fn signals(self) -> Result<SignalFileDescriptor, ThreadLoopInitializationError>
	{
		self.signal_mask.block_all_signals_on_current_thread_bar();
		Ok(SignalFileDescriptor::new(&self.signal_mask.to_sigset_t()).map_err(ThreadLoopInitializationError::SignalFileDescriptor)?)
	}
}

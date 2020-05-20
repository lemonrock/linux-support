// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Initiation.
pub struct ThreadLoopInitiation<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	defaults: DefaultPageSizeAndHugePageSizes,
	global_allocator: &'static GTACSA,
	thread_local_allocator_settings: ThreadLocalAllocatorSettings,
	queues: Queues<(), DequeuedMessageProcessingError>,
	io_uring_settings: IoUringSettings,
	signal_mask: BitSet<Signal>,
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
		
		self.thread_local_allocator_settings.setup(&self.defaults, self.global_allocator).map_err(ThreadLocalAllocator)?;
		
		let (io_uring, registered_buffers) = self.io_uring_settings.setup(&self.defaults)?;
		
		let signal_file_descriptor = self.signals()?;
		
		// One of these exists for every kind of coroutine unless we use an enum...
		let accept_connections_coroutine_manager = CoroutineManager::new(self.global_allocator, unsafe { NonZeroU64::new_unchecked(1) }, &self.defaults).map_err(AcceptConnectionsCoroutineManager)?;
		
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
				accept_connections_coroutine_manager,
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

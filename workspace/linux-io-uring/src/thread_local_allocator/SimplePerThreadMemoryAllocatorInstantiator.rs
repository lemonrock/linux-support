// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread local allocator instantiator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimplePerThreadMemoryAllocatorInstantiator<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	marker: PhantomData<(CoroutineHeapSize, GTACSA)>,
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize> + Send> PerThreadMemoryAllocatorInstantiator for SimplePerThreadMemoryAllocatorInstantiator<CoroutineHeapSize, GTACSA>
{
	type InstantiationArguments = (DefaultHugePageSizes, &'static GTACSA);
	
	type ThreadDropGuard = SimplePerThreadMemoryAllocatorInstantiatorDropGuard<CoroutineHeapSize, GTACSA>;
	
	/// Instantiate.
	fn instantiate(thread_local_allocator_configuration: Arc<ThreadLocalAllocatorConfiguration>, instantiation_arguments: Arc<Self::InstantiationArguments>) -> Result<Self::ThreadDropGuard, MemoryMapError>
	{
		let (defaults, global_allocator) = instantiation_arguments.as_ref();
		
		let thread_allocator_memory_settings = thread_local_allocator_configuration.mapped_memory_settings(defaults);
		let memory_source = MemoryMapSource::new(thread_local_allocator_configuration.heap_size, thread_allocator_memory_settings)?;
		let thread_local_allocator = GTACSA::ThreadLocalAllocator::new_local_allocator(memory_source, LifetimeHint::LongLived, thread_local_allocator_configuration.block_size_hint);
		
		let global_allocator = *global_allocator;
		global_allocator.initialize_thread_local_allocator(thread_local_allocator);
		
		Ok(SimplePerThreadMemoryAllocatorInstantiatorDropGuard::new(global_allocator))
	}
}

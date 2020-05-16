// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread local allocator settings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadLocalAllocatorSettings
{
	heap_size: NonZeroU64,
	inclusive_maximum_bytes_wasted: u64,
	strict_numa_memory_policy: bool,
	block_size_hint: NonZeroUsize,
}

impl ThreadLocalAllocatorSettings
{
	fn setup<HeapSize: Sized, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>(&self, defaults: &DefaultPageSizeAndHugePageSizes, global_allocator: &'static GTACSA) -> Result<(), MemoryMapError>
	{
		let huge_memory_page_size = defaults.best_fit_huge_page_size_if_any(self.heap_size.get(), self.inclusive_maximum_bytes_wasted).map(|huge_page_size| Some(huge_page_size));
		
		let numa_memory_policy =
			{
				use self::MemoryAdvice::*;
				use self::SetMemoryPolicyStrictness::*;
				
				if self.strict_numa_memory_policy
				{
					Some((SetMemoryPolicy::Local, JustSetPolicy))
				}
				else
				{
					Some((SetMemoryPolicy::BindCurrent(), SetPolicyAndMovePagesOrFail))
				}
			};
		
		
		let thread_allocator_memory_settings = MappedMemorySettings
		{
			address_hint: AddressHint::any(),
			protection: Protection::ReadWrite,
			sharing: Sharing::Private,
			huge_memory_page_size,
			prefault: true,
			reserve_swap_space: false,
			numa_memory_policy,
			lock: Some(MemoryLockSettings::Normal),
			advice: hashset!
			{
				DontFork
			}
		};
		
		let memory_source = MemoryMapSource::new(self.heap_size, thread_allocator_memory_settings, defaults)?;
		let thread_local_allocator = GTACSA::ThreadLocalAllocator::new_local_allocator(memory_source, LifetimeHint::LongLived, self.block_size_hint);
		global_allocator.initialize_thread_local_allocator(thread_local_allocator);
		Ok(())
	}
}

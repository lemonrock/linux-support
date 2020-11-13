// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A thread-local cache of common tags which can't be created statically, intended for NUMA nodes and HyperThreads.
#[derive(Debug)]
pub struct ThreadLocalNumericAdditionalDogStatsDTagsCache<Numeric: Into<usize>, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	cache: UnsafeCell<Vec<AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>>>,
	name: &'static str,
	global_allocator: &'static GTACSA,
	marker: PhantomData<Numeric>,
}

impl<Numeric: Into<usize>, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> ThreadLocalNumericAdditionalDogStatsDTagsCache<Numeric, CoroutineHeapSize, GTACSA>
{
	/// New instance.
	///
	/// Should be created on a ThreadLocalAllocator.
	#[inline(always)]
	pub fn new(name: &'static str, global_allocator: &'static GTACSA) -> Rc<Self>
	{
		Rc::new
		(
			Self
			{
				cache: UnsafeCell::new(Vec::with_capacity(8)),
				name,
				global_allocator,
				marker: PhantomData,
			}
		)
	}
	
	/// Get.
	#[inline(always)]
	pub fn get(&self, numeric: Numeric) -> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
	{
		let cache = unsafe { &mut * self.cache.get() };
		let index = numeric.into();
		
		self.add_if_necessary(cache, index);
		Self::get_internal(cache, index)
	}
	
	#[inline(always)]
	fn add_if_necessary(&self, cache: &mut Vec<AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>>, index: usize)
	{
		let old_length = cache.len();
		if unlikely!(index >= old_length)
		{
			self.add(cache, index, old_length)
		}
	}
	
	#[inline(always)]
	fn add(&self, cache: &mut Vec<AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>>, index: usize, old_length: usize)
	{
		let new_length = index + 1;
		if cache.capacity() < new_length
		{
			let additional = new_length - old_length;
			cache.reserve(additional);
		}
		
		let mut new_index = old_length;
		while new_index != new_length
		{
			let tag = AdditionalDogStatsDTag::from(DogStatsDTag::from_usize(self.name, new_index).unwrap(), self.global_allocator);
			unsafe { cache.as_mut_ptr().add(new_index).write(tag) };
			new_index += 1;
		}
		
		unsafe { cache.set_len(new_length) };
	}
	
	#[inline(always)]
	fn get_internal(cache: &mut Vec<AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>>, index: usize) -> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
	{
		let tag_reference = cache.get_unchecked_safe(index);
		tag_reference.clone()
	}
}

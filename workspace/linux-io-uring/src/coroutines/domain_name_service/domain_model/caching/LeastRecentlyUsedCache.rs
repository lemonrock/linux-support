// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct LeastRecentlyUsedCache<'cache, V: LeastRecentlyUsedCacheValue>
{
	cache: HashMap<LeastRecentlyUsedListKeyReference<'cache>, NonNull<LeastRecentlyUsedListPointer<'cache, V>>>,
	records_count: usize,
	maximum_records_count: NonZeroUsize,
	
	/// Least recently used.
	least_recently_used_list_head: *mut LeastRecentlyUsedListPointer<'cache, V>,
	
	/// Most recently used.
	least_recently_used_list_tail: *mut LeastRecentlyUsedListPointer<'cache, V>,
}

impl<'cache, V: LeastRecentlyUsedCacheValue> Drop for LeastRecentlyUsedCache<'cache, V>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		while likely!(!self.least_recently_used_list_head.is_null())
		{
			self.remove_least_recently_used()
		}
		debug_assert_eq!(self.records_count, 0);
	}
}

impl<'cache, V: LeastRecentlyUsedCacheValue> LeastRecentlyUsedCache<'cache, V>
{
	#[inline(always)]
	fn new(maximum_records_count: NonZeroUsize) -> Self
	{
		Self
		{
			cache: HashMap::new(),
		
			records_count: 0,
		
			maximum_records_count,
		
			least_recently_used_list_head: null_mut(),
		
			least_recently_used_list_tail: null_mut(),
		}
	}
	
	#[inline(always)]
	fn put(&mut self, key: EfficientCaseFoldedName, value: V)
	{
		let adds_records_count = value.records_count();
		
		let (key_reference, pointer) = unsafe { LeastRecentlyUsedListPointer::new(&mut self.least_recently_used_list_head, &mut self.least_recently_used_list_tail, key, value) };
		
		if let Some(was) = self.cache.insert(key_reference, pointer)
		{
			self.destroy_after_remove(was);
		}
		
		self.records_count += adds_records_count.get();
		
		while self.records_count >= self.maximum_records_count.get()
		{
			self.remove_least_recently_used();
		}
	}
	
	#[inline(always)]
	fn get_mut(&mut self, key: &EfficientCaseFoldedName) -> Option<&mut V>
	{
		let key = Self::key(key);
		match self.cache.get(&key)
		{
			None => None,
			
			Some(pointer) =>
			{
				let mut pointer = *pointer;
				
				let value = unsafe
				{
					let pointer = pointer.as_mut();
					pointer.move_to_tail(&mut self.least_recently_used_list_head, &mut self.least_recently_used_list_tail);
					
					&mut * ((&mut pointer.value) as *mut V)
				};
				
				Some(value)
			}
		}
	}
	
	#[inline(always)]
	fn remove(&mut self, key: &EfficientCaseFoldedName)
	{
		let key = Self::key(key);
		if let Some(removed) = self.cache.remove(&key)
		{
			self.destroy_after_remove(removed);
		}
	}
	
	#[inline(always)]
	fn remove_least_recently_used(&mut self)
	{
		let key = unsafe
		{
			let instance = &mut * self.least_recently_used_list_head;
			instance.key_reference()
		};
		
		if let Some(removed) = self.cache.remove(&key)
		{
			debug_assert_eq!(removed.as_ptr(), self.least_recently_used_list_head);
			self.destroy_after_remove(removed);
		}
	}
	
	#[inline(always)]
	fn destroy_after_remove(&mut self, removed: NonNull<LeastRecentlyUsedListPointer<'cache, V>>)
	{
		unsafe { LeastRecentlyUsedListPointer::destroy(removed, &mut self.least_recently_used_list_head, &mut self.least_recently_used_list_tail, &mut self.records_count) }
	}
	
	#[inline(always)]
	fn key(key: &EfficientCaseFoldedName) -> LeastRecentlyUsedListKeyReference<'cache>
	{
		let key = unsafe { NonNull::new_unchecked(key as *const EfficientCaseFoldedName as *mut EfficientCaseFoldedName) };
		LeastRecentlyUsedListKeyReference { key }
	}
}

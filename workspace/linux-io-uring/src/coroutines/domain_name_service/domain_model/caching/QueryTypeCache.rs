// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct QueryTypeCache<Record: Sized + Clone>
{
	cache: HashMap<KeyReference, NonNull<LeastRecentlyUsedListPointer<Record>>>,
	records_count: usize,
	maximum_records_count: usize,
	
	/// Least recently used.
	least_recently_used_list_head: *mut LeastRecentlyUsedListPointer<Record>,
	
	/// Most recently used.
	least_recently_used_list_tail: *mut LeastRecentlyUsedListPointer<Record>,
}

impl<Record: Sized + Clone> Drop for QueryTypeCache<Record>
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

impl<Record: Sized + Clone> QueryTypeCache<Record>
{
	#[inline(always)]
	pub(crate) fn new(maximum_records_count: usize) -> Self
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
	
	/// Returns:-
	///
	/// * `None`: No details at all; one could query for the data.
	/// * `Some(None)`: Negatively cached for one-time use only; do not query for the data.
	/// * `Some(Some(records))`: Cached, valid answers.
	#[inline(always)]
	pub(crate) fn get(&mut self, name: &NameAsLabelsIncludingRoot, now: NanosecondsSinceUnixEpoch) -> Option<Option<Vec<QP::Record>>>
	{
		use std::collections::hash_map::Entry::*;
		
		use self::CacheEntry::*;
		
		let (result, records_count_reduction) = match self.get_mut(name)
		{
			None => return None,
			
			Some(&mut AbsentButUncached) => (Some(None), None),
			
			Some(&mut AbsentNegativelyCached(negative_cache_until)) => if negative_cache_until < now
			{
				(None, None)
			}
			else
			{
				(Some(None), Some(0))
			},
			
			Some(&mut Present(Present { ref mut uncached, ref mut cached })) =>
			{
				let mut records_to_return = take(uncached);
				let mut records_count_reduction = records_to_return.len();
				
				let should_still_be_cached = cached.split_off(&now);
				for cached_records in should_still_be_cached.values()
				{
					records_to_return.extend_from_slice(&cached_records[..]);
				}
				for expired in cached.values()
				{
					records_count_reduction += expired.len()
				}
				replace(cached, should_still_be_cached);
				
				if records_to_return.is_empty()
				{
					occupied.remove_entry();
					(None, None)
				}
				else
				{
					(Some(Some(records_to_return)), Some(records_count_reduction))
				}
			}
		};
		
		match records_count_reduction
		{
			None => self.remove(name),
			
			Some(records_count_reduction) => self.records_count -= records_count_reduction,
		}
		
		result
	}
	
	#[inline(always)]
	pub(crate) fn put_present<'message>(&mut self, records: HashMap<WithCompressionParsedName<'message>, Present<Ipv4Addr>>)
	{
		for (name, present) in records
		{
			let key = NameAsLabelsIncludingRoot::new_from_parsed_data(name);
			
			self.put(key, CacheEntry::Present(present));
		}
	}
	
	#[inline(always)]
	pub(crate) fn put_name_error<'message>(&mut self, name: WithCompressionParsedName<'message>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message>)
	{
		self.put_absent::<'message>(name, negative_cache_until, record)
	}
	
	#[inline(always)]
	pub(crate) fn put_no_data<'message>(&mut self, name: WithCompressionParsedName<'message>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message>)
	{
		self.put_absent::<'message>(name, negative_cache_until, record)
	}
	
	#[inline(always)]
	fn put_absent<'message>(&mut self, name: WithCompressionParsedName<'message>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message>)
	{
		let key = NameAsLabelsIncludingRoot::new_from_parsed_data(name);
		
		use self::CacheEntry::*;
		
		self.put
		(
			key,
			match negative_cache_until
			{
				None => AbsentButUncached,
				
				Some(negative_cache_until) => AbsentNegativelyCached(negative_cache_until)
			}
		);
	}
	
	#[inline(always)]
	fn put(&mut self, key: NameAsLabelsIncludingRoot, value: CacheEntry<Record>)
	{
		let adds_records_count = value.records_count();
		
		let (key_reference, pointer) = unsafe { LeastRecentlyUsedListPointer::new(&mut self.least_recently_used_list_head, &mut self.least_recently_used_list_tail, key, value) };
		
		if let Some(was) = self.cache.insert(key_reference, pointer)
		{
			self.destroy_after_remove(was);
		}
		
		self.records_count += adds_records_count;
		
		while self.records_count >= self.maximum_records_count
		{
			self.remove_least_recently_used();
		}
	}
	
	#[inline(always)]
	fn get_mut(&mut self, key: &NameAsLabelsIncludingRoot) -> Option<&mut CacheEntry<Record>>
	{
		let key = Self::key(key);
		match self.cache.get(&key)
		{
			None => None,
			
			Some(pointer) =>
			{
				let pointer = *pointer;
				
				let value = unsafe
				{
					let pointer = pointer.as_mut();
					pointer.move_to_tail(&mut self.least_recently_used_list_head, &mut self.least_recently_used_list_tail);
					
					&mut * ((&mut pointer.value) as *mut CacheEntry<Record>)
				};
				
				Some(value)
			}
		}
	}
	
	#[inline(always)]
	fn remove(&mut self, key: &NameAsLabelsIncludingRoot)
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
	fn destroy_after_remove(&mut self, removed: NonNull<LeastRecentlyUsedListPointer<Record>>)
	{
		unsafe { LeastRecentlyUsedListPointer::destroy(removed, &mut self.least_recently_used_list_head, &mut self.least_recently_used_list_tail, &mut self.records_count) }
	}
	
	#[inline(always)]
	fn key(key: &NameAsLabelsIncludingRoot) -> LeastRecentlyUsedListKeyReference
	{
		let key = unsafe { NonNull::new_unchecked(key as *const NameAsLabelsIncludingRoot as *mut NameAsLabelsIncludingRoot) };
		LeastRecentlyUsedListKeyReference { key }
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct QueryTypeCache<'cache, Record: Sized>
{
	cache: HashMap<LeastRecentlyUsedListKeyReference<'cache>, NonNull<LeastRecentlyUsedListPointer<Record>>>,
	records_count: usize,
	maximum_records_count: usize,
	
	/// Least recently used.
	least_recently_used_list_head: *mut LeastRecentlyUsedListPointer<Record>,
	
	/// Most recently used.
	least_recently_used_list_tail: *mut LeastRecentlyUsedListPointer<Record>,
}

impl<'cache, Record: Sized> Drop for QueryTypeCache<'cache, Record>
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

impl<'cache, Record: Sized> QueryTypeCache<'cache, Record>
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
	
	/// Gets a result for the name.
	#[inline(always)]
	pub fn get(&mut self, name: &CaseFoldedName<'cache>, now: NanosecondsSinceUnixEpoch) -> CacheResult<Record>
	{
		use std::collections::hash_map::Entry::*;
		
		use self::CacheEntry::*;
		use self::CacheResult::*;
		
		const RemoveEntry: Option<usize> = None;
		
		let (result, expired_records_count) = match self.get_mut(name)
		{
			None => return Nothing,
			
			Some(&mut AbsentUseOnce(ref record)) => (DoesNotExist(record.clone()), RemoveEntry),
			
			Some(&mut AbsentNegativelyCached(negative_cache_until, ref record)) => if negative_cache_until < now
			{
				(Nothing, RemoveEntry)
			}
			else
			{
				(DoesNotExist(record.clone()), Some(0))
			},
			
			Some(&mut Present(ref mut present)) => present.retrieve(now),
		};
		
		match expired_records_count
		{
			None => self.remove(name),
			
			Some(expired_records_count) => self.records_count -= expired_records_count,
		}
		
		result
	}
	
	#[inline(always)]
	pub(crate) fn put_present<'message>(&mut self, records: Records<'message, Record>)
	{
		for (name, present) in records.inner()
		{
			let key = CaseFoldedName::from(name);
			
			self.put(key, CacheEntry::Present(present));
		}
	}
	
	
	/// RFC 2308, Section 8 - Changes from RFC 1034, Paragraph 3: "The SOA record from the authority section MUST be cached. Name error indications must be cached against the tuple `<query name, QCLASS>`".
	#[inline(always)]
	pub(crate) fn put_name_error<'message>(&mut self, query_name: CaseFoldedName<'cache>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>)
	{
		self.put_absent::<'message>(query_name, negative_cache_until, record)
	}
	
	/// RFC 2308, Section 8 - Changes from RFC 1034, Paragraph 3: "The SOA record from the authority section MUST be cached. … No data indications must be cached against \[the\] `<query name, QTYPE, QCLASS>` tuple".
	///
	/// However, since we only query for `QCLASS` `IN` internet and never support any other type, this is effectively the same behaviour we must implement as for `self.put_name_error()`.
	#[inline(always)]
	pub(crate) fn put_no_data<'message>(&mut self, query_name: CaseFoldedName<'cache>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>)
	{
		self.put_absent::<'message>(query_name, negative_cache_until, record)
	}
	
	#[inline(always)]
	fn put_absent<'message>(&mut self, query_name: CaseFoldedName<'cache>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>)
	{
		use self::CacheEntry::*;
		
		let record = record.into();
		
		self.put
		(
			query_name,
			
			match negative_cache_until
			{
				None => AbsentUseOnce(Rc::new(record)),
				
				Some(negative_cache_until) => AbsentNegativelyCached(negative_cache_until, Rc::new(record))
			}
		);
	}
	
	#[inline(always)]
	fn put(&mut self, key: CaseFoldedName<'cache>, value: CacheEntry<Record>)
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
	fn get_mut(&mut self, key: &CaseFoldedName<'cache>) -> Option<&mut CacheEntry<Record>>
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
	fn remove(&mut self, key: &CaseFoldedName<'cache>)
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
	fn key(key: &CaseFoldedName<'cache>) -> LeastRecentlyUsedListKeyReference<'cache>
	{
		let key = unsafe { NonNull::new_unchecked(key as *const CaseFoldedName<'cache> as *mut CaseFoldedName<'cache>) };
		LeastRecentlyUsedListKeyReference { key }
	}
}

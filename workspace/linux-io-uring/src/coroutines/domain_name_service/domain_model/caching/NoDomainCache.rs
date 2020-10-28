// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct NoDomainCache<'cache, Record: Sized + Debug>(LeastRecentlyUsedCache<'cache, (NegativeCacheUntil, )>);

impl<'cache> NoDomainCache<'cache, Record>
{
	#[inline(always)]
	pub(crate) fn new(maximum_records_count: NonZeroUsize) -> Self
	{
		Self(LeastRecentlyUsedCache::new(maximum_records_count))
	}
	
	#[inline(always)]
	pub fn has_no_domain(&mut self, name: &CaseFoldedName<'cache>)
	{
		let mut current = name;
		while !current.is_root()
		{
			self.0.get(name)
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
		
		let (result, expired_records_count) = match self.0.get_mut(name)
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
			None => self.0.remove(name),
			
			Some(expired_records_count) => self.0.records_count -= expired_records_count,
		}
		
		result
	}
	
	#[inline(always)]
	pub(crate) fn put_present<'message>(&mut self, records: Records<'message, Record>)
	{
		let hash_map: HashMap<ParsedName<'message>, Present<OldRecord>> = records.into();
		for (name, present) in hash_map
		{
			let key = CaseFoldedName::map(name);
			self.0.put(key, CacheEntry::Present(present));
		}
	}
}

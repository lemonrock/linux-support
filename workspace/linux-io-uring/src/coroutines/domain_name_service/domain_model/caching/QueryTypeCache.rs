// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct QueryTypeCache<'cache, Record: Sized + Debug>(LeastRecentlyUsedCache<'cache, QueryTypeCacheEntry<'cache, Record>>);

impl<'cache, Record: Sized + Debug> QueryTypeCache<'cache, Record>
{
	#[inline(always)]
	pub(crate) fn new(maximum_records_count: NonZeroUsize) -> Self
	{
		Self(LeastRecentlyUsedCache::new(maximum_records_count))
	}
	
	/// Gets a result for the name.
	#[inline(always)]
	pub fn get(&mut self, name: &CaseFoldedName<'cache>, now: NanosecondsSinceUnixEpoch) -> QueryTypeCacheResult<'cache, Record>
	{
		use self::QueryTypeCacheEntry::*;
		use self::QueryTypeCacheResult::*;
		
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
	pub(crate) fn put_present(&mut self, records: Records<'cache, Record>)
	{
		let hash_map: HashMap<CaseFoldedName<'cache>, PresentMultiple<Record>> = records.into();
		for (key, present) in hash_map
		{
			self.0.put(key, QueryTypeCacheEntry::Present(present));
		}
	}
	
	#[inline(always)]
	pub(crate) fn put_present_all_the_same_name(&mut self, name: CaseFoldedName<'cache>, present: PresentMultiple<Record>)
	{
		self.0.put(name, QueryTypeCacheEntry::Present(present))
	}
	
	/// RFC 2308, Section 8 - Changes from RFC 1034, Paragraph 3: "The SOA record from the authority section MUST be cached. Name error indications must be cached against the tuple `<query name, QCLASS>`".
	#[inline(always)]
	pub(crate) fn put_name_error<'message>(&mut self, query_name: CaseFoldedName<'cache>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>)
	{
		self.put_absent(query_name, negative_cache_until, record)
	}
	
	/// RFC 2308, Section 8 - Changes from RFC 1034, Paragraph 3: "The SOA record from the authority section MUST be cached. … No data indications must be cached against \[the\] `<query name, QTYPE, QCLASS>` tuple".
	///
	/// However, since we only query for `QCLASS` `IN` internet and never support any other type, this is effectively the same behaviour we must implement as for `self.put_name_error()`.
	#[inline(always)]
	pub(crate) fn put_no_data<'message>(&mut self, query_name: CaseFoldedName<'cache>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>)
	{
		self.put_absent(query_name, negative_cache_until, record)
	}
	
	#[inline(always)]
	fn put_absent<'message>(&mut self, name: CaseFoldedName<'cache>, negative_cache_until: CacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>)
	{
		use self::QueryTypeCacheEntry::*;
		
		let record = record.into();
		
		self.0.put
		(
			name,
			
			match negative_cache_until
			{
				None => AbsentUseOnce(Rc::new(record)),
				
				Some(negative_cache_until) => AbsentNegativelyCached(negative_cache_until, Rc::new(record))
			}
		);
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub enum GetNotResolvingAliasesResult<'a, ORs: 'a + OwnedRecords<OR>, OR: OwnedRecord>
{
	/// This domain is never valid.
	NeverValid,
	
	/// There is no domain.
	NoDomain,
	
	/// There is an alias to resolve.
	///
	/// Wherever possible, this is flattened.
	Alias(&'a AliasOrDomainTarget),
	
	/// There is a domain, but no data for this particular record.
	NoData,
	
	/// There is data.
	Data(&'a ORs),
}

const Keep: bool = false;

const Remove: bool = true;

impl<'a, ORs: 'a + OwnedRecords<OR>, OR: OwnedRecord> GetNotResolvingAliasesResult<'a, ORs, OR>
{
	#[inline(always)]
	fn fixed(query_types_fixed: &FixedDomainCacheEntry) -> Self
	{
		use self::GetNotResolvingAliasesResult::*;
		
		match query_types_fixed
		{
			FixedDomainCacheEntry::QueryTypesFixed(ref query_types_fixed) => match OR::retrieve_fixed(query_types_fixed)
			{
				None => NoData,
				
				Some(data) => Data(data),
			},
			
			FixedDomainCacheEntry::Alias(ref alias) => Alias(alias),
		}
	}
	
	#[inline(always)]
	fn valid(query_types_cache: &'a mut QueryTypesCache, now: NanosecondsSinceUnixEpoch) -> Option<Self>
	{
		let x = OR::retrieve(query_types_cache);
		if x.is_some()
		{
			let query_type_cache = x.as_ref().unwrap();
			
			let (result, remove) = Self::match_cache_until(query_type_cache.cache_until, now, || no_data_or_data(query_type_cache));
			
			if remove
			{
				*x = None;
			}
			
			result
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn match_cache_until(cache_until: CacheUntil, now: NanosecondsSinceUnixEpoch, callback: impl FnOnce() -> Option<Self>) -> (Option<Self>, bool)
	{
		match cache_until
		{
			CacheUntil::UseOnce { as_of_now } => Self::if_as_of_now(as_of_now, now, callback),
			
			CacheUntil::Cached { cached_until } => Self::if_cached_until(cached_until, now, callback),
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn if_as_of_now(as_of_now: NanosecondsSinceUnixEpoch, now: NanosecondsSinceUnixEpoch, callback: impl FnOnce() -> Option<Self>) -> (Option<Self>, bool)
	{
		if as_of_now == now
		{
			(callback(), Remove)
		}
		else
		{
			(None, Remove)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn if_cached_until(cached_until: NanosecondsSinceUnixEpoch, now: NanosecondsSinceUnixEpoch, callback: impl FnOnce() -> Option<Self>) -> (Option<Self>, bool)
	{
		if cached_until >= now
		{
			(callback(), Keep)
		}
		else
		{
			(None, Remove)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn no_data_or_data(query_type_cache: &'a QueryTypeCache<ORs>) -> Option<Self>
	{
		use self::GetNotResolvingAliasesResult::*;
		
		match query_type_cache.data
		{
			None => Some(NoData),
			
			Some(ref data) => Some(Data(data)),
		}
	}
}

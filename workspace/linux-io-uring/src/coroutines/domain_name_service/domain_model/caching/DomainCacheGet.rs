// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Outcome of a get.
pub enum DomainCacheGet<'a, ORs: 'a + OwnedRecords<OR>, OR: OwnedRecord>
{
	/// There is no domain.
	NoDomain,
	
	/// There is a domain, but no data for this particular record.
	NoData,
	
	/// There is an alias to resolve.
	///
	/// Wherever possible, this is flattened.
	Alias(&'a AliasOrDomainTarget),
	
	/// There is data.
	Data(&'a ORs),
}

impl<'a, ORs: 'a + OwnedRecords<OR>, OR: OwnedRecord> DomainCacheGet<'a, ORs, OR>
{
	#[inline(always)]
	fn fixed(query_types_fixed: &'a FixedDomainCacheEntry) -> Option<Self>
	{
		use self::DomainCacheGet::*;
		
		let this = match query_types_fixed
		{
			FixedDomainCacheEntry::QueryTypesFixed(ref query_types_fixed) => match OR::retrieve_fixed(query_types_fixed)
			{
				None => NoData,
				
				Some(data) => Data(data),
			},
			
			FixedDomainCacheEntry::Alias(ref alias) => Alias(alias),
		};
		Some(this)
	}
	
	#[inline(always)]
	fn alias(flattened_target: &'a SolitaryRecords<AliasOrDomainTarget>, now: NanosecondsSinceUnixEpoch) -> Option<Self>
	{
		let (result, _has_expired) = Self::match_cache_until(flattened_target.negative_cache_until, now, || Some(DomainCacheGet::Alias(&flattened_target.record)));
		result
	}
	
	#[inline(always)]
	fn valid(query_types_cache: &'a QueryTypesCache, now: NanosecondsSinceUnixEpoch) -> Option<Self>
	{
		let query_type_cache_option = OR::retrieve(query_types_cache);
		if query_type_cache_option.is_some()
		{
			let query_type_cache = query_type_cache_option.as_ref().unwrap();
			
			use self::DomainCacheGet::*;
			let (result, _has_expired) = Self::match_cache_until(query_type_cache.cache_until, now, ||
			{
				match query_type_cache.data
				{
					None => Some(NoData),
					
					Some(data) => Some(Data(data)),
				}
			});
			
			result
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn no_domain(no_domain_cache_entry: &'a NoDomainCacheEntry, now: NanosecondsSinceUnixEpoch) -> Option<Self>
	{
		use self::NoDomainCacheEntry::*;
		use self::DomainCacheGet::NoDomain;
		
		let (result, _has_expired) = match no_domain_cache_entry
		{
			&UseOnce { as_of_now, .. } => Self::if_as_of_now(as_of_now, now, || Some(NoDomain)),
			
			&Cached { cached_until, .. } => Self::if_cached_until(cached_until, now, || Some(NoDomain)),
		};
		
		result
	}
	
	#[inline(always)]
	fn match_cache_until(cache_until: CacheUntil, now: NanosecondsSinceUnixEpoch, callback: impl FnOnce() -> Option<Self>) -> (Option<Self>, HasExpired)
	{
		use self::CacheUntil::*;
		
		match cache_until
		{
			UseOnce { as_of_now } => Self::if_as_of_now(as_of_now, now, callback),
			
			Cached { cached_until } => Self::if_cached_until(cached_until, now, callback),
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn if_as_of_now(as_of_now: NanosecondsSinceUnixEpoch, now: NanosecondsSinceUnixEpoch, callback: impl FnOnce() -> Option<Self>) -> (Option<Self>, HasExpired)
	{
		if as_of_now == now
		{
			(callback(Expired), Expired)
		}
		else
		{
			(None, Expired)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn if_cached_until(cached_until: NanosecondsSinceUnixEpoch, now: NanosecondsSinceUnixEpoch, callback: impl FnOnce() -> Option<Self>) -> (Option<Self>, HasExpired)
	{
		if cached_until >= now
		{
			(callback(HasNotExpired), HasNotExpired)
		}
		else
		{
			(None, Expired)
		}
	}
}

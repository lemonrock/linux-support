// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For record type:-
///
/// * `SOA`.
/// * Possibly `DNAME`.
/// * Not used for `CNAME`.
pub(crate) struct SolitaryRecords<OR: OwnedRecord + Ord>
{
	pub(crate) negative_cache_until: NegativeCacheUntil,
	pub(crate) record: OR,
}

impl<OR: OwnedRecord + Ord> SolitaryRecords<OR>
{
	#[inline(always)]
	pub(crate) const fn new(negative_cache_until: NegativeCacheUntil, record: OR) -> Self
	{
		Self
		{
			negative_cache_until,
			record
		}
	}
	
	#[inline(always)]
	pub(crate) fn no_domain_cache_entry(&self, authority_name: DomainTarget) -> NoDomainCacheEntry
	{
		use self::CacheUntil::*;
		
		match self.negative_cache_until
		{
			UseOnce { as_of_now } => NoDomainCacheEntry::UseOnce { as_of_now, authority_name: Some(authority_name) },
			
			Cached { cached_until } => NoDomainCacheEntry::Cached { cached_until, authority_name },
		}
	}
}

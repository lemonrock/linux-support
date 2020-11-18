// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Clone)]
enum NoDomainCacheEntry
{
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3;
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4.
	UseOnce
	{
		as_of_now: NanosecondsSinceUnixEpoch,
		
		/// Looking this up will return either nothing, a `StartOfAuthority` and/or name servers.
		///
		/// Is `None` for:-
		///
		/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3.
		///
		/// Is `Some()` for:-
		///
		/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1 (in which case the `authority_name` will point to `SOA` and `NS` records);
		/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2 (in which case the `authority_name` will point to a `SOA` record);
		/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4 (in which case the `authority_name` will point to `NS` records).
		authority_name: Option<DomainTarget>,
	},
	
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1 when the TTL is *not* 0 (in which case the `authority_name` will point to `SOA` and `NS` records).
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2 when the TTL is *not* 0 (in which case the `authority_name` will point to a `SOA` record).
	Cached
	{
		cached_until: NanosecondsSinceUnixEpoch,
		
		authority_name: DomainTarget,
	}
}

impl NoDomainCacheEntry
{
	#[inline(always)]
	fn no_domain_cache_entry((start_of_authority, authority_name): (SolitaryRecords<StartOfAuthority<EfficientCaseFoldedName>>, DomainTarget)) -> (NoDomainCacheEntry, Option<NonZeroU8>)
	{
		let number_of_labels_including_root = authority_name.number_of_labels_including_root();
		use self::NoDomainCacheEntry::*;
		(
			match start_of_authority.negative_cache_until
			{
				CacheUntil::UseOnce { as_of_now } => UseOnce { as_of_now, authority_name: Some(authority_name) },
				
				CacheUntil::Cached { cached_until } => Cached { cached_until, authority_name },
			},
			Some(number_of_labels_including_root)
		)
	}
	
	#[inline(always)]
	fn use_once_without_authority_name(as_of_now: NanosecondsSinceUnixEpoch) -> (NoDomainCacheEntry, Option<NonZeroU8>)
	{
		(
			NoDomainCacheEntry::UseOnce
			{
				as_of_now,
				authority_name: None
			},
			None
		)
	}
	
	#[inline(always)]
	fn use_once_with_authority_name(as_of_now: NanosecondsSinceUnixEpoch, authority_name: DomainTarget) -> (NoDomainCacheEntry, Option<NonZeroU8>)
	{
		let number_of_labels_including_root = authority_name.number_of_labels_including_root();
		(
			NoDomainCacheEntry::UseOnce
			{
				as_of_now,
				authority_name: Some(authority_name.clone())
			},
			Some(number_of_labels_including_root)
		)
	}
}

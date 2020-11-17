// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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
	fn store(most_canonical_name: DomainTarget, response_type: NoDomainResponseType, domain_cache: &mut DomainCache) -> Result<(), AnsweredError>
	{
		use self::NoDomainCacheEntry::*;
		use self::NoDomainResponseType::*;
		
		domain_cache.guard_can_be_replaced_by_no_domain(&most_canonical_name)?;
		
		let no_domain_cache_entry = match response_type
		{
			// TODO: is_implicit_referral
			NoDomainResponseType1(authority_name_start_of_authority_name_servers) =>
			{
				let no_domain_cache_entry = Self::validate_authority_name_can_have_records_then_store(authority_name_start_of_authority_name_servers.authority_name_start_of_authority, domain_cache, |domain_cache, authority_name|
				{
					domain_cache.store_name_servers_unchecked(authority_name.clone(), authority_name_start_of_authority_name_servers.name_servers);
					authority_name
				})?;
				no_domain_cache_entry
			}
			
			NoDomainResponseType2(authority_name_start_of_authority) => Self::validate_authority_name_can_have_records_then_store(authority_name_start_of_authority, domain_cache, |_, authority_name| authority_name)?,
			
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDomainResponseType3 { as_of_now } => UseOnce
			{
				as_of_now,
				authority_name: None
			},
			
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			// TODO: is_implicit_referral
			NoDomainResponseType4 { as_of_now, authority_name_name_servers } =>
			{
				let authority_name = authority_name_name_servers.authority_name;
				domain_cache.guard_can_have_records(&authority_name)?;
				domain_cache.store_name_servers_unchecked(authority_name.clone(), authority_name_name_servers.name_servers);
				
				UseOnce
				{
					as_of_now,
					authority_name: Some(authority_name)
				}
			}
		};
		
		domain_cache.store_no_domain_unchecked
		(
			most_canonical_name,
			no_domain_cache_entry,
		);
		
		Ok(())
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn validate_authority_name_can_have_records_then_store(authority_name_start_of_authority: AuthorityNameStartOfAuthority, domain_cache: &mut DomainCache, store_name_servers_unchecked: impl FnOnce(&mut DomainCache, DomainTarget) -> DomainTarget) -> Result<Self, AnsweredError>
	{
		use self::NoDomainCacheEntry::*;
		
		let authority_name = authority_name_start_of_authority.authority_name;
		domain_cache.guard_can_have_records(&authority_name)?;
		let authority_name = store_name_servers_unchecked(domain_cache, authority_name);
		
		let no_domain_cache_entry = authority_name_start_of_authority.start_of_authority.no_domain_cache_entry(authority_name.clone());
		let start_of_authority = authority_name_start_of_authority.start_of_authority;
		
		domain_cache.store_start_of_authority_unchecked(authority_name, start_of_authority);
		
		Ok(no_domain_cache_entry)
	}
	
}

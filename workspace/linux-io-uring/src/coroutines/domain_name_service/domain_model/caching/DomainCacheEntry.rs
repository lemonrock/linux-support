// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Canonical name chains are `QTYPE`-independent.
///
/// RFC 2181, Setcion 10.1 `CNAME` resource records:-
///
/// "That is, for any label in the DNS (any domain name) exactly one of the following is true:
/// + one `CNAME` record exists, optionally accompanied by `SIG`, `NXT`, and `KEY` RRs,
/// + one or more records exist, none being `CNAME` records,
/// + the name exists, but has no associated RRs of any type,
/// + the name does not exist at all".
#[derive(Debug)]
pub(crate) enum DomainCacheEntry
{
	/// Domains that can never be valid, eg `example.com.`.
	NeverValid,

	/// Domains that have fixed, unchanging data that should never be recursively queried.
	Fixed
	{
		/// This is `true` for `*.localhost` and `*.localhost.domain`.
		///
		/// For all other usages (eg entries parsed from `/etc/hosts`), this should be `false`, in which case subdomains will resolve as `NeverValid`.
		subdomains_implicitly_resolve_to_the_same_record_as_this_one: bool,
		
		/// Either a cache of `QTYPE` or a flattened target (alias) (usually as the result of parsing an `/etc/hosts` file).
		fixed_domain_cache_entry: FixedDomainCacheEntry,
	},
	
	/// Valid domains that may, at some point, cease to exist.
	///
	/// Only occurs for `Answer::Answered` or `Answer::NoData`.
	Valid
	{
		/// Irrespective of TTLs, etc, a domain that is always valid.
		///
		/// Typically used for the root and all top-level domains.
		always_valid: bool,
		
		/// For special addresses like `ipv4only.arpa`, subdomains are never valid.
		///
		/// Also, for PTR domains, sub-domains of pointers can never be valid.
		subdomains_are_never_valid: bool,
		
		/// Cache of `QTYPE`.
		query_types_cache: QueryTypesCache,
	},
	
	/// A canonical name (`CNAME`) record.
	///
	/// Point 1 in RFC 2181, Section 10.1 `CNAME` resource records above.
	Alias
	{
		/// Skips to the end of the canonical name chain.
		///
		/// Cached until is for the lowest TTL value in the canonical name chain.
		///
		/// Can be the same target as `original_target`.
		///
		/// Note that there may be an `Alias` at the end of the canonical chain (this is possible as the DNS itself changes).
		flattened_target: SolitaryRecords<AliasOrDomainTarget>,
		
		/// Target of alias.
		original_target: AliasOrDomainTarget,
	},
	
	/// Point 4 in RFC 2181, Section 10.1 `CNAME` resource records above.
	///
	/// Only can occur if we receive an `Answer::NoDomain` for a domain that is neither `NeverValid` or `Valid { always_valid: false }`.
	NoDomain(NoDomainCacheEntry),
}

impl DomainCacheEntry
{
	#[inline(always)]
	pub(crate) const fn always_valid_domain_name() -> Self
	{
		DomainCacheEntry::Valid
		{
			always_valid: true,
			
			subdomains_are_never_valid: false,
			
			query_types_cache: Default::default()
		}
	}
	
	#[inline(always)]
	pub(crate) const fn always_valid_domain_name_but_subdomains_are_not_valid_domain_names(query_types_cache: QueryTypesCache) -> Self
	{
		DomainCacheEntry::Valid
		{
			always_valid: true,
			
			subdomains_are_never_valid: true,
		
			query_types_cache
		}
	}
	
	#[inline(always)]
	pub(crate) const fn fixed_local_machine_only() -> Self
	{
		DomainCacheEntry::Fixed
		{
			subdomains_implicitly_resolve_to_the_same_record_as_this_one: true,
			
			fixed_domain_cache_entry: FixedDomainCacheEntry::new_local_internet_protocol_addresses(),
		}
	}
	
	#[inline(always)]
	pub(crate) const fn fixed_internet_protocol_address(internet_protocol_address: IpAddr) -> Self
	{
		DomainCacheEntry::Fixed
		{
			subdomains_implicitly_resolve_to_the_same_record_as_this_one: false,
			
			fixed_domain_cache_entry: FixedDomainCacheEntry::query_types_fixed(internet_protocol_address)
		}
	}
	
	#[inline(always)]
	pub(crate) const fn fixed_internet_protocol_address_potentially_blocked(internet_protocol_address: Option<IpAddr>) -> Self
	{
		match internet_protocol_address
		{
			Some(internet_protocol_address) => Self::fixed_internet_protocol_address(internet_protocol_address),
			
			None => DomainCacheEntry::NeverValid,
		}
	}
	
	#[inline(always)]
	pub(crate) const fn fixed_pointer(canonical_name: &DomainTarget) -> Self
	{
		DomainCacheEntry::Fixed
		{
			subdomains_implicitly_resolve_to_the_same_record_as_this_one: false,
			
			fixed_domain_cache_entry: FixedDomainCacheEntry::pointer(canonical_name)
		}
	}
	
	#[inline(always)]
	pub(crate) const fn fixed_alias(canonical_name: &DomainTarget) -> Self
	{
		DomainCacheEntry::Fixed
		{
			subdomains_implicitly_resolve_to_the_same_record_as_this_one: false,
			
			fixed_domain_cache_entry: FixedDomainCacheEntry::alias(canonical_name)
		}
	}
	
	#[inline(always)]
	fn store_parsed<PR: ParsedRecord>(records: OwnerNameToParsedRecordsValue<PR>) -> Self
	{
		let mut query_types_cache = QueryTypesCache::default();
		let mut subdomains_are_never_valid: bool = unsafe_uninitialized();
		PR::store(NonNull::from(&mut subdomains_are_never_valid), &mut query_types_cache, records);
		DomainCacheEntry::Valid { always_valid: false, subdomains_are_never_valid, query_types_cache }
	}
	
	#[inline(always)]
	fn no_data<PR: ParsedRecord>(negative_cache_until: NegativeCacheUntil) -> Self
	{
		let mut query_types_cache = QueryTypesCache::default();
		let mut subdomains_are_never_valid: bool = unsafe_uninitialized();
		PR::no_data(NonNull::from(&mut subdomains_are_never_valid), &mut query_types_cache, negative_cache_until);
		DomainCacheEntry::Valid { always_valid: false, subdomains_are_never_valid, query_types_cache }
	}
	
	#[inline(always)]
	fn store_owned<OR: OwnedRecord>(records: OR::OwnedRecords) -> Self
	{
		let mut query_types_cache = QueryTypesCache::default();
		let mut subdomains_are_never_valid: bool = unsafe_uninitialized();
		OR::store(NonNull::from(&mut subdomains_are_never_valid), &mut query_types_cache, records);
		DomainCacheEntry::Valid { always_valid: false, subdomains_are_never_valid, query_types_cache }
	}
	
	#[inline(always)]
	fn can_not_be_replaced_by_alias(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		match self
		{
			&NeverValid | Fixed { ..} | Valid { always_valid: true, .. } => true,
		
			_ => false,
		}
	}
	
	#[inline(always)]
	fn can_not_be_replaced_by_no_domain(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		match self
		{
			&NeverValid | Fixed { ..}  | Valid { always_valid: true, .. } => true,
		
			_ => false,
		}
	}
	
	#[inline(always)]
	fn can_not_have_records(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		match self
		{
			&NeverValid | Fixed { ..}  => true,
			
			_ => false,
		}
	}
}

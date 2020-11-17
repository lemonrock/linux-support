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
enum DomainCacheEntry<'cache>
{
	/// Domains that can never be valid, eg `example.com.`.
	NeverValid,

	/// Irrespective of TTLs, etc, a domain that is always valid.
	///
	/// Typically used for the root and all top-level domains.
	///
	/// Also used for, say, private hosted zones in Amazon Route 53.
	///
	/// Only occurs for `Answer::Answered` or `Answer::NoData`.
	AlwaysValid
	{
		/// Cache of `QTYPE`.
		cache: QueryTypesCache,
	},
	
	/// Valid domains that may, at some point, cease to exist.
	///
	/// Only occurs for `Answer::Answered` or `Answer::NoData`.
	CurrentlyValid
	{
		/// Cache of `QTYPE`.
		cache: QueryTypesCache,
	},
	
	/// A canonical name (`CNAME`) record.
	///
	/// Point 1 in RFC 2181, Section 10.1 `CNAME` resource records above.
	Alias
	{
		target: SolitaryRecords<AliasOrDomainTarget>,
	},
	
	/// Point 4 in RFC 2181, Section 10.1 `CNAME` resource records above.
	///
	/// Only can occur if we receive an `Answer::NoDomain` for a domain that is neither `NeverValid` or `Valid { always_valid: false }`.
	NoDomain(NoDomainCacheEntry),
}

impl DomainCacheEntry
{
	#[inline(always)]
	fn answered<PR: ParsedRecord>(store_records_in_query_types_cache: impl FnOnce(&mut QueryTypesCache, OwnerNameToRecordValue<PR>), records: OwnerNameToRecordsValue<PR>) -> Self
	{
		let mut cache = QueryTypesCache::default();
		store_records_in_query_types_cache(&mut cache, records);
		DomainCacheEntry::CurrentlyValid { cache }
	}
	
	#[inline(always)]
	fn can_not_be_replaced_by_alias(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		match self
		{
			&NeverValid | AlwaysValid { .. } => true,
		
			_ => false,
		}
	}
	
	#[inline(always)]
	fn can_not_be_replaced_by_no_domain(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		match self
		{
			&NeverValid | AlwaysValid { .. } => true,
		
			_ => false,
		}
	}
	
	#[inline(always)]
	fn can_not_have_records(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		match self
		{
			&NeverValid => true,
			
			_ => false,
		}
	}
}

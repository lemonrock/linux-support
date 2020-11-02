// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct DomainCache<'cache>
{
	map: HashMap<AliasOrDomainTarget<'cache>, DomainCacheEntry<'cache>>
}

/// This can resolve to either an alias or a domain.
type AliasOrDomainTarget<'cache> = CaseFoldedName<'cache>;

/// This, when it resolves, can not resolve to `DomainCacheEntry::Alias`.
type DomainTarget<'cache> = CaseFoldedName<'cache>;

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

	/// Valid domains.
	///
	/// Only occurs for `Answer::Answered` or `Answer::NoData`.
	Valid
	{
		/// Domains that always exist.
		///
		/// Typically used for the root and all top-level domains.
		///
		/// Also used for, say, private hosted zones in Amazon Route 53.
		always_valid: bool,
	
		/// Cache of `QTYPE`.
		cache: QueryTypesCache<'cache>,
	},
	
	/// A canonical name (`CNAME`) record.
	///
	/// Point 1 in RFC 2181, Section 10.1 `CNAME` resource records above.
	Alias
	{
		target: PresentSolitary<AliasOrDomainTarget<'cache>>,
	},
	
	/// Point 4 in RFC 2181, Section 10.1 `CNAME` resource records above.
	///
	/// Only can occur if we receive an `Answer::NoDomain` for a domain that is neither `NeverValid` or `Valid { always_valid: false }`.
	NoDomain(NoDomainCacheEntry<'cache>),
}

enum NoDomainCacheEntry<'cache>
{
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3;
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4.
	UseOnce
	{
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
		authority_name: Option<DomainTarget<'cache>>,
	},
	
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1 when the TTL is *not* 0 (in which case the `authority_name` will point to `SOA` and `NS` records).
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2 when the TTL is *not* 0 (in which case the `authority_name` will point to a `SOA` record).
	Cached
	{
		authority_name: DomainTarget<'cache>,
	}
}

enum NoDataCacheEntry<'cache>
{
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
	UseOnce
	{
		/// Looking this up will return either nothing, a `StartOfAuthority` and/or name servers.
		///
		/// Is `None` for:-
		///
		/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
		///
		/// Is `Some()` for:-
		///
		/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 (in which case the `authority_name` will point to `SOA` and `NS` records);
		/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 (in which case the `authority_name` will point to a `SOA` record).
		authority_name: Option<DomainTarget<'cache>>,
	},
	
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 when the TTL is *not* 0 (in which case the `authority_name` will point to `SOA` and `NS` records).
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 when the TTL is *not* 0 (in which case the `authority_name` will point to a `SOA` record).
	Cached
	{
		authority_name: DomainTarget<'cache>,
	}
}

enum QueryTypeCacheMultiple<'cache, Record: Sized + Debug>
{
	NoData(NoDataCacheEntry<'cache>),

	Data(PresentMultiple<'cache, Record>)
}

enum QueryTypeCacheSolitary<'cache, Record: Sized + Debug>
{
	NoData(NoDataCacheEntry<'cache>),

	Data(PresentSolitary<'cache, Record>)
}

pub(crate) struct QueryTypesCache<'cache>
{
	ns: QueryTypeCacheMultiple<'cache, DomainTarget<'cache>>,
	
	soa: QueryTypeCacheSolitary<'cache, StartOfAuthority<'cache, DomainTarget<'cache>>>,
	
	dname: QueryTypeCacheSolitary<'cache, DomainTarget<'cache>>,
	
	a: QueryTypeCacheMultiple<'cache, Ipv4Addr>,
	
	aaaa: QueryTypeCacheMultiple<'cache, Ipv6Addr>,
	
	mx: QueryTypeCacheMultiple<'cache, DomainTarget<'cache>>,
}

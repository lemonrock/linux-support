// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub enum AnsweredError
{
	/// Can not be an alias because it is never a valid domain name or the domain name must always exist.
	DomainNameCanNotBeAnAlias(Alias<'static>),
}

impl Display for AnsweredError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for AnsweredError
{
}

pub(crate) struct DomainCache<'cache>
{
	map: HashMap<AliasOrDomainTarget<'cache>, DomainCacheEntry<'cache>>
}

impl<'cache> DomainCache<'cache>
{
	pub(crate) fn new(never_valid: Cow<'_, HashSet<DomainTarget<'cache>>>, top_level_domains: Cow<'_, HashSet<DomainTarget<'cache>>>, always_valid_private_domains: Cow<'_, HashSet<DomainTarget<'cache>>>) -> Self
	{
		use self::DomainCacheEntry::*;
		
		let mut this = Self
		{
			map: HashMap::with_capacity(never_valid.len() + top_level_domains.len() + always_valid_private_domains.len()),
		};
		
		this.over_write_never_valid(never_valid);
		this.over_write_always_valid(top_level_domains);
		this.over_write_always_valid(always_valid_private_domains);
		
		this
	}
	
	fn over_write_never_valid<'a>(&mut self, never_valid: Cow<'a, HashSet<DomainTarget<'cache>>>)
	{
		self.over_write(never_valid, DomainCacheEntry::NeverValid)
	}
	
	fn over_write_always_valid<'a>(&mut self, always_valid: Cow<'a, HashSet<DomainTarget<'cache>>>)
	{
		self.over_write(always_valid, DomainCacheEntry::AlwaysValid)
	}
	
	#[inline(always)]
	fn over_write<'a>(&mut self, domains: Cow<'a, HashSet<DomainTarget<'cache>>>, to_domain_cache_entry: impl Fn() -> DomainCacheEntry<'cache>)
	{
		use self::Cow::*;
		match domains
		{
			Owned(domains) => for domain in domains
			{
				self.map.insert(domain, to_domain_cache_entry())
			},
			
			Borrowed(domains) => for domain in domains
			{
				self.map.insert(domain.clone(), to_domain_cache_entry())
			},
		}
	}
	
	pub(crate) fn answered<'message, Record: Sized + Debug>(&mut self, now: NanosecondsSinceUnixEpoch, query_name: &'message EfficientCaseFoldedName, answer: Answer<cache>, canonical_name_chain_records: CanonicalNameChainRecords<'cache>, finished: Records<'cache, Record>, query: impl FnOnce(xxx) -> ()) -> Result<(), AnsweredError>
	{
		use self::Answer::*;
		use self::AnsweredError::*;
		use self::NoDomainResponseType::*;
		use self::NoDataResponseType::*;
		use self::DomainCacheEntry::*;
		
		match answer
		{
			Answered =>
			{
				debug_assert!(!finished.is_empty());
				
				self.replace_canonical_name_chain_records(canonical_name_chain_records)?;
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn replace_canonical_name_chain_records(&mut self, canonical_name_chain_records: CanonicalNameChainRecords<'cache>) -> Result<(), AnsweredError>
	{
		// Check for problems before we mutate the map.
		for alias in canonical_name_chain_records.keys()
		{
			if let Some(domain_cache_entry) = self.map.get(alias)
			{
				if unlikely!(domain_cache_entry.can_not_be_replaced_by_alias())
				{
					return Err(AnsweredError::DomainNameCanNotBeAnAlias(alias.static_clone()))
				}
			}
		}
		
		for (alias, target) in canonical_name_chain_records
		{
			use self::FastSecureHashMapEntry::*;
			let value = DomainCacheEntry::Alias { target };
			match self.map.entry(alias)
			{
				Occupied(occupied) =>
				{
					*occupied.get_mut() = value;
				},
				
				Vacant(vacant) =>
				{
					vacant.insert(value);
				}
			}
		}
		
		Ok(())
	}
}

/// This can resolve to either an alias or a domain.
pub(crate) type Alias<'cache> = EfficientCaseFoldedName;

/// This can resolve to either an alias or a domain.
pub(crate) type AliasOrDomainTarget<'cache> = EfficientCaseFoldedName;

/// This, when it resolves, can not resolve to `DomainCacheEntry::Alias`.
pub(crate) type DomainTarget<'cache> = EfficientCaseFoldedName;

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
		cache: QueryTypesCache<'cache>,
	},
	
	/// Valid domains that may, at some point, cease to exist.
	///
	/// Only occurs for `Answer::Answered` or `Answer::NoData`.
	CurrentlyValid
	{
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

impl<'cache> DomainCacheEntry<'cache>
{
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
		authority_name: Option<DomainTarget<'cache>>,
	},
	
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1 when the TTL is *not* 0 (in which case the `authority_name` will point to `SOA` and `NS` records).
	/// * RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2 when the TTL is *not* 0 (in which case the `authority_name` will point to a `SOA` record).
	Cached
	{
		cached_until: NanosecondsSinceUnixEpoch,
		
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
		as_of_now: NanosecondsSinceUnixEpoch,
		
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
		cached_until: NanosecondsSinceUnixEpoch,
		
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

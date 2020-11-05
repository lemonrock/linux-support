// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub enum AnsweredError
{
	/// Can not be an alias because it is never a valid domain name or the domain name must always exist.
	DomainNameCanNotBeAnAlias(Alias),
	
	/// Domain name must exist because it is never a valid domain name or the domain name must always exist.
	DomainNameCanNotNotExist(DomainTarget),
	
	/// Domain name is never valid.
	DomainNameCanNotNotHaveRecords(AliasOrDomainTarget),
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

pub(crate) struct DomainCache
{
	map: HashMap<AliasOrDomainTarget, DomainCacheEntry>
}

impl DomainCache
{
	pub(crate) fn new(never_valid: Cow<'_, HashSet<DomainTarget>>, top_level_domains: Cow<'_, HashSet<DomainTarget>>, always_valid_private_domains: Cow<'_, HashSet<DomainTarget>>) -> Self
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
	
	fn over_write_never_valid(&mut self, never_valid: Cow<HashSet<DomainTarget>>)
	{
		self.over_write(never_valid, DomainCacheEntry::NeverValid)
	}
	
	fn over_write_always_valid(&mut self, always_valid: Cow<HashSet<DomainTarget>>)
	{
		self.over_write(always_valid, DomainCacheEntry::AlwaysValid)
	}
	
	#[inline(always)]
	fn over_write(&mut self, domains: Cow<HashSet<DomainTarget>>, to_domain_cache_entry: impl Fn() -> DomainCacheEntry)
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
	
	// TODO: `query` will not work for CNAME, DNAME or SOA.
	pub(crate) fn answered<'message, Record: Sized + Debug>(&mut self, now: NanosecondsSinceUnixEpoch, query_name: &'message EfficientCaseFoldedName, answer: Answer<cache>, canonical_name_chain_records: CanonicalNameChainRecords, delegation_name_records: DelegationNameRecords, finished: Records<Record>, query: impl FnOnce(&mut QueryTypesCache) -> (&mut QueryTypeCacheMultiple<Record>)) -> Result<(), AnsweredError>
	{
		use self::AnsweredError::*;
		use self::FastSecureHashMapEntry::*;
		use self::NoDomainResponseType::*;
		use self::NoDataResponseType::*;
		
		self.replace_canonical_name_chain_records(canonical_name_chain_records)?;
		
		match answer
		{
			Answer::Answered { most_canonical_name } =>
			{
				debug_assert!(!finished.is_empty());
				
				
				
				
				self.can_have_records(&most_canonical_name)?;
				// // TODO: Now what? - finished is for multiple names, but we should only store records for answers given for the end of the cname chain.
				// Other answers should probably have not been returned?
				// We can intercept this at RRV::fn finished(self) -> Self::Finished after
				query(TODO_query_types_cache).store(most_canonical_name, finished)
			},
			
			Answer::NoDomain { most_canonical_name, response_type} =>
			{
				debug_assert!(finished.is_empty());
				
				NoDomainCacheEntry::store(most_canonical_name, response_type, self)?;
				
				// // TODO: Implicit referral!
			}
			
			Answer::NoData { most_canonical_name, response_type} =>
			{
				debug_assert!(finished.is_empty());
				
				self.can_have_records(&most_canonical_name)?;
				// // TODO: Now what?
				query(TODO_query_types_cache).empty(most_canonical_name)
				
				// // TODO: Implicit referral!
			}
			
			Answer::Referral { most_canonical_name, referral} =>
			{
				debug_assert!(finished.is_empty());
				
				let authority_name = referral.authority_name;
				self.can_have_records(&authority_name)?;
				self.store_name_servers_unchecked(authority_name, referral.name_servers);
				
				// // TODO: referral!
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn replace_canonical_name_chain_records(&mut self, canonical_name_chain_records: CanonicalNameChainRecords) -> Result<(), AnsweredError>
	{
		// Check for problems before we mutate the map.
		for alias in canonical_name_chain_records.keys()
		{
			self.can_be_an_alias(alias)?;
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
	
	#[inline(always)]
	fn store_start_of_authority_unchecked(&mut self, authority_name: DomainTarget, start_of_authority: PresentSolitary<StartOfAuthority<'static, EfficientCaseFoldedName>>)
	{
		xxx;
	}
	
	#[inline(always)]
	fn store_name_servers_unchecked(&mut self, authority_name: DomainTarget, name_servers: PresentMultiple<DomainTarget>)
	{
		xxx;
	}
	
	#[inline(always)]
	fn can_be_an_alias(&self, alias: &Alias) -> Result<(), AnsweredError>
	{
		if let Some(domain_cache_entry) = self.map.get(alias)
		{
			if unlikely!(domain_cache_entry.can_not_be_replaced_by_alias())
			{
				return Err(AnsweredError::DomainNameCanNotBeAnAlias(alias.clone()))
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn can_be_replaced_by_no_domain(&self, domain_target: &DomainTarget) -> Result<(), AnsweredError>
	{
		if let Some(entry) = self.map.get(domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_be_replaced_by_no_domain())
			{
				return Err(AnsweredError::DomainNameCanNotNotExist(most_canonical_name))
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn can_have_records(&self, alias_or_domain_target: &AliasOrDomainTarget) -> Result<(), AnsweredError>
	{
		if let Some(entry) = self.map.get(alias_or_domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_have_records())
			{
				return Err(AnsweredError::DomainNameCanNotNotHaveRecords(most_canonical_name))
			}
		}
		Ok(())
	}
}

/// This can resolve to either an alias or a domain.
pub(crate) type Alias = EfficientCaseFoldedName;

/// This can resolve to either an alias or a domain.
pub(crate) type AliasOrDomainTarget = EfficientCaseFoldedName;

/// This, when it resolves, can not resolve to `DomainCacheEntry::Alias`.
pub(crate) type DomainTarget = EfficientCaseFoldedName;

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
		target: PresentSolitary<AliasOrDomainTarget>,
	},
	
	/// Point 4 in RFC 2181, Section 10.1 `CNAME` resource records above.
	///
	/// Only can occur if we receive an `Answer::NoDomain` for a domain that is neither `NeverValid` or `Valid { always_valid: false }`.
	NoDomain(NoDomainCacheEntry),
}

impl DomainCacheEntry
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
		
		domain_cache.can_be_replaced_by_no_domain(&most_canonical_name)?;
		
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
				domain_cache.can_have_records(&authority_name)?;
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
		domain_cache.can_have_records(&authority_name)?;
		let authority_name = store_name_servers_unchecked(domain_cache, authority_name);
		
		let (no_domain_cache_entry, start_of_authority) = match authority_name_start_of_authority.start_of_authority
		{
			PresentSolitary::UseOnce { as_of_now, record } =>
			{
				(
					UseOnce
					{
						as_of_now,
						
						authority_name: Some(authority_name.clone()),
					},
					
					PresentSolitary::UseOnce { as_of_now, record }
				)
			}
			
			PresentSolitary::Cached { cached_until, record } =>
			{
				(
					Cached
					{
						cached_until,
						
						authority_name: authority_name.clone(),
					},
					
					PresentSolitary::Cached { cached_until, record }
				)
			}
		};
		
		domain_cache.store_start_of_authority_unchecked(authority_name, start_of_authority);
		
		Ok(no_domain_cache_entry)
	}
	
}

enum NoDataCacheEntry
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
		authority_name: Option<DomainTarget>,
	},
	
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 when the TTL is *not* 0 (in which case the `authority_name` will point to `SOA` and `NS` records).
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 when the TTL is *not* 0 (in which case the `authority_name` will point to a `SOA` record).
	Cached
	{
		cached_until: NanosecondsSinceUnixEpoch,
		
		authority_name: DomainTarget,
	}
}

enum QueryTypeCacheMultiple<Record: Sized + Debug>
{
	NoData(NoDataCacheEntry),

	Data(PresentMultiple<Record>)
}

enum QueryTypeCacheSolitary<Record: Sized + Debug>
{
	NoData(NoDataCacheEntry),

	Data(PresentSolitary<Record>)
}

pub(crate) struct QueryTypesCache
{
	ns: QueryTypeCacheMultiple<DomainTarget>,
	
	soa: QueryTypeCacheSolitary<StartOfAuthority<'static, DomainTarget>>,
	
	dname: QueryTypeCacheSolitary<DomainTarget>,
	
	a: QueryTypeCacheMultiple<Ipv4Addr>,
	
	aaaa: QueryTypeCacheMultiple<Ipv6Addr>,
	
	mx: QueryTypeCacheMultiple<DomainTarget>,
}

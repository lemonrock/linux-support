// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct DomainCache<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineHeapSize: MemorySize>
{
	map: HashMap<AliasOrDomainTarget, DomainCacheEntry>,
	gtacsa: &'static GTACSA,
	our_usage: Cell<u64>,
	marker: PhantomData<CoroutineHeapSize>,
}

// TODO:
	// There must be an instance per-interface, as things like ipv4only.arpa and CDNs change records depending on interface.
	// ipv4only.arpa has AAAA records with a NAT64 prefix and fixed A records.

// TODO: Analyze errors to deduce server failures, bad records, etc. (we want to store a bad record indicator) - probably caching HandleRecordTypeError.
// TODO: Also: server failure RFC 2308 Section 7.1 / 7.2.

// TODO: Can we optimize the use of guards and map.entry()
// TODO: Cache by size, list by cache expiry but also keep some sort of LRU count?
// TODO: How to compute size?
// TODO: Switch all Authority stuff to ParsedName not owned stuff?

// TODO: lazy_static users in uriparse and crossbeam-utils
// TODO: All memory allocations need to be made using the thread-local allocator
// TODO: Actually querying.

// TODO: PTR records. These are very special; there is no good reason for a PTR domain name to ever have any other record types.
// TODO: Likewise, any target name that is a ip6.arpa or the like is very likely to be wrong.
/*
	RFC 2317 style PTR records can use CNAMEs for delegation, but the CNAME must still be within the in-addr.arpa / ip6.arpa root?
	
	eg 88.7.4.62.in-addr.arpa. 86400   IN      CNAME   88.64-27.7.4.62.in-addr.arpa.  for a /27 network.

	In the original RFC, a `/` was used instead of a `-`.
	
	
	Multiple PTR records.
	
	Real example for google.com using 172.217.20.142, query 142.20.217.172.in-addr.arpa:-
142.20.217.172.in-addr.arpa. 28311 IN	PTR	lhr48s20-in-f14.1e100.net.
142.20.217.172.in-addr.arpa. 28311 IN	PTR	muc11s10-in-f14.1e100.net.
142.20.217.172.in-addr.arpa. 28311 IN	PTR	fra07s27-in-f142.1e100.net.
	
 */


impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineHeapSize: MemorySize> DomainCache<GTACSA, CoroutineHeapSize>
{
	#[inline(always)]
	fn callback_with_thread_local_allocator_detailing_memory_usage<F: FnOnce() -> R + UnwindSafe, R>(&self, callback: F)
	{
		self.gtacsa.callback_with_thread_local_allocator_detailing_memory_usage(&self.our_usage, callback)
	}
	
	#[inline(always)]
	pub(crate) fn new(map: HashMap<FullyQualifiedDomainName, DomainCacheEntry>, gtacsa: &'static GTACSA) -> Self
	{
		Self
		{
			map,
			gtacsa,
			our_usage: Cell::new(0),
			marker: PhantomData,
		}
	}
	
	pub(crate) fn get_not_resolving_aliases<'a, OR: OwnedRecord>(&'a mut self, domain_target: DomainTarget, now: NanosecondsSinceUnixEpoch) -> Option<GetNotResolvingAliasesResult<'a, OR::OwnedRecords, OR>>
	{
		use self::FastSecureHashMapEntry::*;
		
		match self.map.entry(domain_target)
		{
			Vacant(occupied) =>
			{
				let parent = match vacant.into_key().parent()
				{
					None => return None,
					Some(parent) => parent,
				};
				return self.get_not_resolving_aliases_recursively_check_if_parent_is_a_non_existent_domain(parent, now)
			}
			
			Occupied(occupied) =>
			{
				use self::DomainCacheEntry::*;
				
				let (result, remove) = match occupied.get_mut()
				{
					NeverValid => (Some(GetNotResolvingAliasesResult::NeverValid), Keep),
					
					Fixed { fixed_domain_cache_entry, .. } => (Some(GetNotResolvingAliasesResult::fixed(fixed_domain_cache_entry)), Keep),
					
					&mut Valid { always_valid: true, ref mut query_types_cache, .. } =>
					{
						let result = GetNotResolvingAliasesResult::valid(query_types_cache, now);
						(result, Keep)
					}
					
					&mut Valid { always_valid: false, ref mut query_types_cache, subdomains_are_never_valid: false } =>
					{
						let result = GetNotResolvingAliasesResult::valid(query_types_cache, now);
						(result, query_types_cache.is_empty())
					}
					
					Alias { flattened_target, .. } => GetNotResolvingAliasesResult::match_cache_until(flattened_target.negative_cache_until, now, || Some(GetNotResolvingAliasesResult::Alias(&flattened_target.record))),
					
					NoDomain(no_domain_cache_entry) => match no_domain_cache_entry
					{
						&NoDomainCacheEntry::UseOnce { as_of_now, .. } => GetNotResolvingAliasesResult::if_as_of_now(as_of_now, now, || Some(GetNotResolvingAliasesResult::NoDomain)),
						
						&NoDomainCacheEntry::Cached { cached_until, .. } => GetNotResolvingAliasesResult::if_cached_until(cached_until, now, || Some(GetNotResolvingAliasesResult::NoDomain)),
					},
				};
				
				if remove
				{
					occupied.remove();
				}
				
				result
			}
		}
	}
	
	fn get_not_resolving_aliases_recursively_check_if_parent_is_a_non_existent_domain<'a, OR: OwnedRecord>(&'a mut self, parent: DomainTarget, now: NanosecondsSinceUnixEpoch) -> Option<GetNotResolvingAliasesResult<'a, OR::OwnedRecords, OR>>
	{
		use self::FastSecureHashMapEntry::*;
		
		if unlikely!(parent.is_root())
		{
			return None
		}
		
		match self.map.entry(parent)
		{
			Vacant(vacant) =>
			{
				let grand_parent = vacant.into_key().parent().expect("Already checked domain_target.is_root()");
				return self.get_not_resolving_aliases_recursively_check_if_parent_is_a_non_existent_domain(grand_parent, now)
			}
			
			Occupied(occupied) =>
			{
				use self::DomainCacheEntry::*;
				
				let (result, remove) = match occupied.get_mut()
				{
					NeverValid => (Some(GetNotResolvingAliasesResult::NeverValid), Keep),
					
					Fixed { subdomains_implicitly_resolve_to_the_same_record_as_this_one: true, fixed_domain_cache_entry }  => (Some(GetNotResolvingAliasesResult::fixed(fixed_domain_cache_entry)), Keep),
					
					Fixed { subdomains_implicitly_resolve_to_the_same_record_as_this_one: false, .. } => (Some(GetNotResolvingAliasesResult::NeverValid), Keep),
					
					Valid { always_valid: true, subdomains_are_never_valid: true, .. } => (Some(GetNotResolvingAliasesResult::NeverValid), Keep),
					
					Valid { .. } => (None, Keep),
					
					Alias { .. } => (None, Keep),
					
					NoDomain(no_domain_cache_entry) => match no_domain_cache_entry
					{
						&NoDomainCacheEntry::UseOnce { as_of_now, .. } => GetNotResolvingAliasesResult::if_as_of_now(as_of_now, now, || Some(GetNotResolvingAliasesResult::NoDomain)),
						
						&NoDomainCacheEntry::Cached { cached_until, .. } => GetNotResolvingAliasesResult::if_cached_until(cached_until, now, || Some(GetNotResolvingAliasesResult::NoDomain)),
					},
				};
				
				if remove
				{
					occupied.remove();
				}
				
				result
			}
		}
	}
	
	pub(crate) fn answered<'message, PR: ParsedRecord>(&mut self, answer: Answer<PR>, canonical_name_chain: CanonicalNameChain<'message>) -> Result<(), CacheStoreError>
	{
		use self::CacheStoreError::*;
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		use self::NoDomainResponseType::*;
		use self::NoDataResponseType::*;
		
		let (most_canonical_name, canonical_name_chain_records) = self.guard_replace_canonical_name_chain(canonical_name_chain)?;
		
		let is_referral = match answer
		{
			Answer::Answered { records } => self.store_data(&most_canonical_name, records)?,
			
			Answer::NoDomain { response_type} => self.store_no_domain(&most_canonical_name, response_type)?,
			
			Answer::NoData { response_type} => self.store_no_data::<PR>(&most_canonical_name, response_type)?,
			
			Answer::Referral { referral} => self.store_referral(referral)?,
		};
		
		self.replace_canonical_name_chain(&most_canonical_name, canonical_name_chain_records);
		
		Ok(())
	}
	
	#[inline(always)]
	fn guard_replace_canonical_name_chain<'message>(&self, canonical_name_chain: CanonicalNameChain<'message>) -> Result<(FullyQualifiedDomainName, CanonicalNameChainRecords), CacheStoreError>
	{
		let most_canonical_name = canonical_name_chain.most_canonical_name().clone();
		
		let canonical_name_chain_records = canonical_name_chain.into();
		
		// Check for problems before we mutate the map.
		for alias in canonical_name_chain_records.keys()
		{
			self.guard_can_be_an_alias(alias)?;
		}
		
		Ok((most_canonical_name, canonical_name_chain_records))
	}
	
	#[inline(always)]
	fn replace_canonical_name_chain(&mut self, most_canonical_name: &FullyQualifiedDomainName, canonical_name_chain_records: CanonicalNameChainRecords)
	{
		let most_canonical_name = canonical_name_chain.most_canonical_name().clone();
		
		let mut lowest_negative_cache_until = NegativeCacheUntil::Highest;
		for (alias, target) in canonical_name_chain_records.into_iter().rev()
		{
			let target: SolitaryRecords<FullyQualifiedDomainName> = target;
			lowest_negative_cache_until.update(target.negative_cache_until);
			
			let value = DomainCacheEntry::Alias
			{
				flattened_target: SolitaryRecords::new(lowest_negative_cache_until, most_canonical_name.clone()),
				original_target: target.record,
			};
			
			use self::FastSecureHashMapEntry::*;
			
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
	}
	
	#[inline(always)]
	fn store_data<PR: ParsedRecord>(&mut self, most_canonical_name: &DomainTarget, records: OwnerNameToRecordsValue<PR>) -> Result<bool, CacheStoreError>
	{
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		
		debug_assert!(!records.is_empty());
		
		match self.map.entry(most_canonical_name.clone())
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::store_parsed(records))
			}
			
			Occupied(occupied) =>
			{
				match occupied.get_mut()
				{
					NeverValid => return Err(CacheStoreError::DomainNameCanNotNotHaveRecords(occupied.replace_key())),
					
					Fixed { .. } => return Err(CacheStoreError::DomainNameIsFixed(occupied.replace_key())),
					
					Valid { query_types_cache, .. } => PR::store(query_types_cache, records),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::store_parsed(records),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::store_parsed(records),
				}
			}
		}
		
		Ok(false)
	}
	
	#[inline(always)]
	fn store_no_data<PR: ParsedRecord>(&mut self, most_canonical_name: &DomainTarget, response_type: NoDataResponseType) -> Result<bool, CacheStoreError>
	{
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		use self::NoDataResponseType::*;
		
		// TODO: Can guard by getting .entry() just once.
		self.guard_can_be_replaced_by_no_data(most_canonical_name)?;
		
		let (negative_cache_until, is_referral) = match response_type
		{
			// RFC 2308, Section 6, Negative answers from the cache: "`NXDOMAIN` types 1 and 4 responses contain implicit referrals as does `NODATA` type 1 response".
			NoDataResponseType1(authority_name_start_of_authority_name_servers) =>
			{
				let (start_of_authority, _) = self.guard_authority_name_can_have_records_then_store_start_of_authority_and_name_servers(authority_name_start_of_authority_name_servers)?;
				(start_of_authority.negative_cache_until, true)
			},
			
			NoDataResponseType2(authority_name_start_of_authority) =>
			{
				let (start_of_authority, _) = self.guard_authority_name_can_have_records_then_store_start_of_authority(authority_name_start_of_authority)?;
				(start_of_authority.negative_cache_until, false)
			},
			
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDataResponseType3 { as_of_now } => (CacheUntil::UseOnce { as_of_now }, false),
		};
		
		match self.map.entry(most_canonical_name.clone())
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::no_data(negative_cache_until))
			}
			
			Occupied(occupied) =>
			{
				match occupied.get_mut()
				{
					NeverValid => return Err(CacheStoreError::DomainNameCanNotNotHaveRecords(occupied.replace_key())),
					
					Fixed { .. } => return Err(CacheStoreError::DomainNameIsFixed(occupied.replace_key())),
					
					Valid { query_types_cache, .. }  => PR::no_data(query_types_cache, negative_cache_until),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::no_data(negative_cache_until),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::no_data(negative_cache_until),
				}
			}
		}
		
		Ok(is_referral)
	}
	
	#[inline(always)]
	fn store_no_domain(&mut self, most_canonical_name: &DomainTarget, response_type: NoDomainResponseType) -> Result<bool, CacheStoreError>
	{
		use self::NoDomainResponseType::*;
		
		self.guard_can_be_replaced_by_no_domain(most_canonical_name)?;
		
		let ((no_domain_cache_entry, authority_name_number_of_labels_including_root), is_referral) = match response_type
		{
			// RFC 2308, Section 6, Negative answers from the cache: "`NXDOMAIN` types 1 and 4 responses contain implicit referrals as does `NODATA` type 1 response".
			NoDomainResponseType1(authority_name_start_of_authority_name_servers) =>
			{
				let start_of_authority_and_authority_name = self.guard_authority_name_can_have_records_then_store_start_of_authority_and_name_servers(authority_name_start_of_authority_name_servers)?;
				(NoDomainCacheEntry::no_domain_cache_entry(start_of_authority_and_authority_name), true)
			},
			
			NoDomainResponseType2(authority_name_start_of_authority) =>
			{
				let start_of_authority_and_authority_name = self.guard_authority_name_can_have_records_then_store_start_of_authority(authority_name_start_of_authority)?;
				(NoDomainCacheEntry::no_domain_cache_entry(start_of_authority_and_authority_name), false)
			},
			
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDomainResponseType3 { as_of_now } => (NoDomainCacheEntry::use_once_without_authority_name(as_of_now), false),
			
			// RFC 2308, Section 6, Negative answers from the cache: "`NXDOMAIN` types 1 and 4 responses contain implicit referrals as does `NODATA` type 1 response".
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDomainResponseType4 { as_of_now, authority_name_name_servers } =>
			{
				let authority_name = self.guard_authority_name_can_have_records_then_store_name_servers(authority_name_name_servers)?;
				(NoDomainCacheEntry::use_once_with_authority_name(as_of_now, authority_name), true)
			},
		};
		
		// Intermediate domains between most_canonical_name and authority_name may not exist.
		// eg `dig -t naptr 4.4.2.2.3.3.5.6.8.1.4.6.e164.arpa.` gives a `NoDomainResponseType2` response with a SOA owner name of `e164.arpa.`.
		// This means all the intermediate domains 4.2.2.3.3.5.6.8.1.4.6.e164.arpa to 6.e164.arpa do not exist.
		match authority_name_number_of_labels_including_root
		{
			None => self.store_no_domain_unchecked
			(
				most_canonical_name,
				no_domain_cache_entry,
			),
			
			Some(authority_name_number_of_labels_including_root) =>
			{
				let authority_name_number_of_labels_including_root = authority_name_number_of_labels_including_root.get();
				let one_above_authority_name = authority_name_number_of_labels_including_root + 1;
				let mut intermediate_domain = Cow::Borrowed(most_canonical_name);
				loop
				{
					let intermediate_domain_number_of_labels_including_root = intermediate_domain.number_of_labels_including_root().get();
					debug_assert!(intermediate_domain_number_of_labels_including_root > authority_name_number_of_labels_including_root);
					
					if intermediate_domain_number_of_labels_including_root == one_above_authority_name
					{
						self.store_no_domain_unchecked
						(
							intermediate_domain.deref(),
							no_domain_cache_entry,
						);
						
						break
					}
					else
					{
						self.store_no_domain_unchecked
						(
							intermediate_domain.deref(),
							no_domain_cache_entry.clone(),
						);
						
						intermediate_domain = Cow::Owned(domain_name.parent().unwrap());
					}
				}
			}
		}
		
		Ok(is_referral)
	}
	
	#[inline(always)]
	fn store_referral(&mut self, referral: AuthorityNameNameServers) -> Result<bool, CacheStoreError>
	{
		let _authority_name = self.guard_authority_name_can_have_records_then_store_name_servers(referral)?;
		Ok(true)
	}
	
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_start_of_authority_and_name_servers(&mut self, authority_name_start_of_authority_name_servers: AuthorityNameStartOfAuthorityNameServers) -> Result<(SolitaryRecords<StartOfAuthority<FullyQualifiedDomainName>>, DomainTarget), CacheStoreError>
	{
		let authority_name_start_of_authority = authority_name_start_of_authority_name_servers.authority_name_start_of_authority;
		let name_servers = authority_name_start_of_authority_name_servers.name_servers;
		
		self.guard_authority_name_can_have_records_then_store_start_of_authority_and_optionally_name_servers(authority_name_start_of_authority, |domain_cache, authority_name| domain_cache.store_name_servers_unchecked(authority_name, name_servers))
	}
	
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_start_of_authority(&mut self, authority_name_start_of_authority: AuthorityNameStartOfAuthority) -> Result<(SolitaryRecords<StartOfAuthority<FullyQualifiedDomainName>>, DomainTarget), CacheStoreError>
	{
		self.guard_authority_name_can_have_records_then_store_start_of_authority_and_optionally_name_servers(authority_name_start_of_authority, |_, _| ())
	}
	
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_name_servers(&mut self, authority_name_name_servers: AuthorityNameNameServers) -> Result<DomainTarget, CacheStoreError>
	{
		let authority_name = authority_name_name_servers.authority_name;
		let name_servers = authority_name_name_servers.name_servers;
		
		self.guard_can_have_records(&authority_name)?;
		self.store_name_servers_unchecked(&authority_name, name_servers);
		
		Ok(authority_name)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_start_of_authority_and_optionally_name_servers(&mut self, authority_name_start_of_authority: AuthorityNameStartOfAuthority, store_name_servers_unchecked: impl FnOnce(&mut DomainCache, &DomainTarget)) -> Result<(SolitaryRecords<StartOfAuthority<FullyQualifiedDomainName>>, DomainTarget), CacheStoreError>
	{
		let authority_name = authority_name_start_of_authority.authority_name;
		let start_of_authority = authority_name_start_of_authority.start_of_authority;
		
		self.guard_can_have_records(&authority_name)?;
		
		self.store_start_of_authority_unchecked(&authority_name, &start_of_authority);
		store_name_servers_unchecked(self, &authority_name);
		
		Ok((start_of_authority, authority_name))
	}
	
	#[inline(always)]
	fn store_start_of_authority_unchecked(&mut self, authority_name: &DomainTarget, start_of_authority: &SolitaryRecords<StartOfAuthority<FullyQualifiedDomainName>>)
	{
		use self::FastSecureHashMapEntry::*;
		use self::DomainCacheEntry::*;
		
		let records = start_of_authority.into_owned_record();
		match self.map.entry(authority_name.clone())
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::store_owned::<StartOfAuthority<FullyQualifiedDomainName>>(records));
			}
			
			Occupied(occupied) =>
			{
				match occupied.get_mut()
				{
					NeverValid => panic!("Should have been checked"),
					
					Valid { query_types_cache, .. }  => StartOfAuthority::<FullyQualifiedDomainName>::store(query_types_cache, records),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::store_owned::<StartOfAuthority<FullyQualifiedDomainName>>(records),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::store_owned::<StartOfAuthority<FullyQualifiedDomainName>>(records),
				}
			}
		}
	}
	
	#[inline(always)]
	fn store_name_servers_unchecked(&mut self, authority_name: &DomainTarget, name_servers: MultipleSortedRecords<NameServerName<FullyQualifiedDomainName>>)
	{
		use self::FastSecureHashMapEntry::*;
		use self::DomainCacheEntry::*;
		
		match self.map.entry(authority_name.clone())
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::store_owned::<NameServerName<FullyQualifiedDomainName>>(name_servers));
			}
			
			Occupied(occupied) =>
			{
				match occupied.get_mut()
				{
					NeverValid => panic!("Should have been checked"),
					
					Valid { query_types_cache, .. }  => NameServerName::<FullyQualifiedDomainName>::store(query_types_cache, records),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::store_owned::<NameServerName<FullyQualifiedDomainName>>(name_servers),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::store_owned::<NameServerName<FullyQualifiedDomainName>>(name_servers),
				}
			}
		}
	}
	
	#[inline(always)]
	fn store_no_domain_unchecked(&mut self, most_canonical_name: &DomainTarget, no_domain_cache_entry: NoDomainCacheEntry)
	{
		self.map.insert(most_canonical_name.clone(), DomainCacheEntry::NoDomain(no_domain_cache_entry));
	}
	
	#[inline(always)]
	fn guard_can_be_an_alias(&self, alias: &Alias) -> Result<(), CacheStoreError>
	{
		if let Some(domain_cache_entry) = self.map.get(alias)
		{
			if unlikely!(domain_cache_entry.can_not_be_replaced_by_alias())
			{
				return Err(CacheStoreError::DomainNameCanNotBeAnAlias(alias.clone()))
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn guard_can_be_replaced_by_no_data(&self, domain_target: &DomainTarget) -> Result<(), CacheStoreError>
	{
		self.guard_can_have_records(domain_target)
	}
	
	#[inline(always)]
	fn guard_can_be_replaced_by_no_domain(&self, domain_target: &DomainTarget) -> Result<(), CacheStoreError>
	{
		if let Some(domain_cache_entry) = self.map.get(domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_be_replaced_by_no_domain())
			{
				return Err(CacheStoreError::DomainNameCanNotNotExist(most_canonical_name))
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn guard_can_have_records(&self, alias_or_domain_target: &AliasOrDomainTarget) -> Result<(), CacheStoreError>
	{
		if let Some(domain_cache_entry) = self.map.get(alias_or_domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_have_records())
			{
				return Err(CacheStoreError::DomainNameCanNotNotHaveRecords(most_canonical_name))
			}
		}
		Ok(())
	}
}

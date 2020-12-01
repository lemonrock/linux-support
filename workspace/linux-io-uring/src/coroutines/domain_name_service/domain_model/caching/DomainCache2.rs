// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct DomainCache2<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineHeapSize: MemorySize>
{
	root: ZoneCut,
	
	gtacsa: &'static GTACSA,
	
	our_usage: Cell<u64>,
	
	marker: PhantomData<CoroutineHeapSize>,
}

impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineHeapSize: MemorySize> DomainCache2<GTACSA, CoroutineHeapSize>
{
	#[inline(always)]
	pub(crate) fn new(map: HashMap<FullyQualifiedDomainName, DomainCacheEntry>, gtacsa: &'static GTACSA) -> Self
	{
		XXXXX;
		
		for (key, value) in map
		{
			if key.is_root()
			{
				debug_assert!(value dadadasd);
			}
			// TODO: Create root and everything in it!!!
			// TODO: Do this with gtacsa callback so we can track memory usage and get an accurate initial value, but we're not doing anything else.
			
			// TODO: Add answered() logic from original DomainCache.
		}
		
		Self
		{
			root: XXX,
			gtacsa,
			our_usage: Cell::new(XXXX),
			marker: PhantomData,
		}
	}
	
	/// If this isn't a valid Internet Protocol address to look up then `None` is returned.
	/// If the address is valid but there are no results, `Some(None)` is returned.
	#[inline(always)]
	pub(crate) fn get_internet_protocol_version_4_address_not_resolving_aliases<'a>(&'a mut self, internet_protocol_address: Ipv4Addr, now: NanosecondsSinceUnixEpoch) -> Option<Option<DomainCacheGet<'a, MultipleSortedRecords<PointerName<DomainTarget>>, PointerName<DomainTarget>>>>
	{
		DomainTarget::internet_protocol_version_4_pointer(internet_protocol_address).map(|domain_target| self.get_not_resolving_aliases(domain_target, now))
	}
	
	/// If this isn't a valid Internet Protocol address to look up then `None` is returned.
	/// If the address is valid but there are no results, `Some(None)` is returned.
	#[inline(always)]
	pub(crate) fn get_internet_protocol_version_6_address_not_resolving_aliases<'a>(&'a mut self, internet_protocol_address: Ipv6Addr, now: NanosecondsSinceUnixEpoch) -> Option<Option<DomainCacheGet<'a, MultipleSortedRecords<PointerName<DomainTarget>>, PointerName<DomainTarget>>>>
	{
		DomainTarget::internet_protocol_version_6_pointer(internet_protocol_address).map(|domain_target| self.get_not_resolving_aliases(domain_target, now))
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
	fn store_data<PR: ParsedRecord>(&mut self, most_canonical_name: &DomainTarget, records: OwnerNameToParsedRecordsValue<PR>) -> Result<bool, CacheStoreError>
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
					
					Valid { subdomains_are_never_valid, query_types_cache, .. } => PR::store(NonNull::from(subdomains_are_never_valid), query_types_cache, records),
					
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
					
					Valid { subdomains_are_never_valid, query_types_cache, .. }  => query_types_cache.no_data::<PR>(NonNull::from(subdomains_are_never_valid), negative_cache_until),
					
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
					
					Valid { subdomains_are_never_valid, query_types_cache, .. }  => query_types_cache.store(NonNull::from(subdomains_are_never_valid), records),
					
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
					
					Valid { subdomains_are_never_valid, query_types_cache, .. }  => query_types_cache.store(NonNull::from(subdomains_are_never_valid), records),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::store_owned::<NameServerName<FullyQualifiedDomainName>>(name_servers),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::store_owned::<NameServerName<FullyQualifiedDomainName>>(name_servers),
				}
			}
		}
	}
	
	// TODO: Fix this and those above using self.map
	// TODO: This is the first of the inset
	#[inline(always)]
	fn store_no_domain_unchecked(&mut self, most_canonical_name: &DomainTarget, no_domain_cache_entry: NoDomainCacheEntry)
	{
		self.map.insert(most_canonical_name.clone(), DomainCacheEntry::NoDomain(no_domain_cache_entry));
	}
	
	#[inline(always)]
	fn guard_can_be_an_alias(&self, alias: &Alias) -> Result<(), CacheStoreError>
	{
		if let Some(domain_cache_entry) = self.root.get_domain_cache_entry(alias)
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
		if let Some(domain_cache_entry) = self.root.get_domain_cache_entry(domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_be_replaced_by_no_domain())
			{
				return Err(CacheStoreError::DomainNameCanNotNotExist(domain_target.clone()))
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn guard_can_have_records(&self, alias_or_domain_target: &AliasOrDomainTarget) -> Result<(), CacheStoreError>
	{
		if let Some(domain_cache_entry) = self.root.get_domain_cache_entry(alias_or_domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_have_records())
			{
				return Err(CacheStoreError::DomainNameCanNotNotHaveRecords(alias_or_domain_target.clone()))
			}
		}
		Ok(())
	}
}

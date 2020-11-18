// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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
	
	pub(crate) fn answered<'message, PR: ParsedRecord>(&mut self, now: NanosecondsSinceUnixEpoch, domain_cache: &mut DomainCache, answer: Answer<PR>, canonical_name_chain: CanonicalNameChain<'message>) -> Result<(), AnsweredError>
	{
		use self::AnsweredError::*;
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		use self::NoDomainResponseType::*;
		use self::NoDataResponseType::*;
		
		// TODO: This needs to be transactional!!! - we need to check for other replacements first!
		let most_canonical_name = self.replace_canonical_name_chain(canonical_name_chain)?;
		
		match answer
		{
			Answer::Answered { records } =>
			{
				self.store_data(most_canonical_name, records)?;
			}
			
			// TODO: Implicit referral!
			Answer::NoDomain { response_type} =>
			{
				// TODO: The most_canonical_name is not necessarily the child of the authority name; it can be a grandchild, etc, and so we could infer that the intermediate domains are NXDOMAIN, too.
				self.store_no_domain(most_canonical_name, response_type)?;
			}
			
			// TODO: Implicit referral!
			Answer::NoData { response_type} =>
			{
				self.store_no_data::<PR>(most_canonical_name, response_type)?;
			}
			
			// TODO: referral!
			Answer::Referral { referral} =>
			{
				self.store_referral(referral)?;
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn over_write_never_valid(&mut self, never_valid: Cow<HashSet<DomainTarget>>)
	{
		self.over_write(never_valid, DomainCacheEntry::NeverValid)
	}
	
	#[inline(always)]
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
	
	#[inline(always)]
	fn replace_canonical_name_chain<'message>(&mut self, canonical_name_chain: CanonicalNameChain<'message>) -> Result<EfficientCaseFoldedName, AnsweredError>
	{
		let most_canonical_name = canonical_name_chain.most_canonical_name().clone();
		let canonical_name_chain_records = canonical_name_chain.into();
		
		// Check for problems before we mutate the map.
		for alias in canonical_name_chain_records.keys()
		{
			self.guard_can_be_an_alias(alias)?;
		}
		
		let mut lowest_negative_cache_until = NegativeCacheUntil::Highest;
		for (alias, target) in canonical_name_chain_records.into_iter().rev()
		{
			let target: SolitaryRecords<EfficientCaseFoldedName> = target;
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
		
		Ok(most_canonical_name)
	}
	
	#[inline(always)]
	fn store_data<PR: ParsedRecord>(&mut self, most_canonical_name: DomainTarget, records: OwnerNameToRecordsValue<PR>) -> Result<(), AnsweredError>
	{
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		
		debug_assert!(!records.is_empty());
		
		match self.map.entry(most_canonical_name)
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::store(records))
			}
			
			Occupied(occupied) =>
			{
				match occupied.get_mut()
				{
					&mut NeverValid => return Err(AnsweredError::DomainNameCanNotNotHaveRecords(occupied.replace_key())),
					
					&mut AlwaysValid { ref mut query_types_cache } => PR::store(query_types_cache, records),
					
					&mut CurrentlyValid { ref mut query_types_cache } => PR::store(query_types_cache, records),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::store(records),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::store(records),
				}
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn store_no_data<PR: ParsedRecord>(&mut self, most_canonical_name: DomainTarget, response_type: NoDataResponseType) -> Result<(), AnsweredError>
	{
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		use self::NoDataResponseType::*;
		
		// TODO: Can guard by getting .entry() just once.
		self.guard_can_be_replaced_by_no_data(&most_canonical_name)?;
		
		let negative_cache_until = match response_type
		{
			// TODO: is_implicit_referral
			NoDataResponseType1(authority_name_start_of_authority_name_servers) =>
			{
				let (start_of_authority, _) = self.guard_authority_name_can_have_records_then_store_start_of_authority_and_name_servers(authority_name_start_of_authority_name_servers)?;
				start_of_authority.negative_cache_until
			},
			
			NoDataResponseType2(authority_name_start_of_authority) =>
			{
				let (start_of_authority, _) = self.guard_authority_name_can_have_records_then_store_start_of_authority(authority_name_start_of_authority)?;
				start_of_authority.negative_cache_until
			},
			
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDataResponseType3 { as_of_now } => CacheUntil::UseOnce { as_of_now },
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
					&mut NeverValid => return Err(AnsweredError::DomainNameCanNotNotHaveRecords(occupied.replace_key())),
					
					&mut AlwaysValid { ref mut query_types_cache } => PR::no_data(query_types_cache, negative_cache_until),
					
					&mut CurrentlyValid { ref mut query_types_cache } => PR::no_data(query_types_cache, negative_cache_until),
					
					x @ &mut Alias { .. } => *x = DomainCacheEntry::no_data(negative_cache_until),
					
					x @ &mut NoDomain(_) => *x = DomainCacheEntry::no_data(negative_cache_until),
				}
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn store_no_domain(&mut self, most_canonical_name: DomainTarget, response_type: NoDomainResponseType) -> Result<(), AnsweredError>
	{
		use self::NoDomainResponseType::*;
		
		self.guard_can_be_replaced_by_no_domain(&most_canonical_name)?;
		
		let no_domain_cache_entry = match response_type
		{
			// TODO: is_implicit_referral
			NoDomainResponseType1(authority_name_start_of_authority_name_servers) =>
			{
				let start_of_authority_and_authority_name = self.guard_authority_name_can_have_records_then_store_start_of_authority_and_name_servers(authority_name_start_of_authority_name_servers)?;
				NoDomainCacheEntry::no_domain_cache_entry(start_of_authority_and_authority_name)
			},
			
			NoDomainResponseType2(authority_name_start_of_authority) =>
			{
				let start_of_authority_and_authority_name = self.guard_authority_name_can_have_records_then_store_start_of_authority(authority_name_start_of_authority)?;
				NoDomainCacheEntry::no_domain_cache_entry(start_of_authority_and_authority_name)
			},
			
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDomainResponseType3 { as_of_now } => NoDomainCacheEntry::use_once_without_authority_name(as_of_now),
			
			// TODO: is_implicit_referral
			// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
			NoDomainResponseType4 { as_of_now, authority_name_name_servers } =>
			{
				let authority_name = self.guard_authority_name_can_have_records_then_store_name_servers(authority_name_name_servers)?;
				NoDomainCacheEntry::use_once_with_authority_name(as_of_now, authority_name)
			},
		};
		
		self.store_no_domain_unchecked
		(
			most_canonical_name,
			no_domain_cache_entry,
		);
		
		Ok(())
	}
	
	#[inline(always)]
	fn store_referral(&mut self, referral: AuthorityNameNameServers) -> Result<(), AnsweredError>
	{
		let _authority_name = self.guard_authority_name_can_have_records_then_store_name_servers(referral)?;
		Ok(())
	}
	
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_start_of_authority_and_name_servers(&mut self, authority_name_start_of_authority_name_servers: AuthorityNameStartOfAuthorityNameServers) -> Result<(SolitaryRecords<StartOfAuthority<EfficientCaseFoldedName>>, DomainTarget), AnsweredError>
	{
		let authority_name_start_of_authority = authority_name_start_of_authority_name_servers.authority_name_start_of_authority;
		let name_servers = authority_name_start_of_authority_name_servers.name_servers;
		
		self.guard_authority_name_can_have_records_then_store_start_of_authority_and_optionally_name_servers(authority_name_start_of_authority, |domain_cache, authority_name| domain_cache.store_name_servers_unchecked(authority_name, name_servers))
	}
	
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_start_of_authority(&mut self, authority_name_start_of_authority: AuthorityNameStartOfAuthority) -> Result<(SolitaryRecords<StartOfAuthority<EfficientCaseFoldedName>>, DomainTarget), AnsweredError>
	{
		self.guard_authority_name_can_have_records_then_store_start_of_authority_and_optionally_name_servers(authority_name_start_of_authority, |_, _| ())
	}
	
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_name_servers(&mut self, authority_name_name_servers: AuthorityNameNameServers) -> Result<DomainTarget, AnsweredError>
	{
		let authority_name = authority_name_name_servers.authority_name;
		let name_servers = authority_name_name_servers.name_servers;
		
		self.guard_can_have_records(&authority_name)?;
		self.store_name_servers_unchecked(&authority_name, name_servers);
		
		Ok(authority_name)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn guard_authority_name_can_have_records_then_store_start_of_authority_and_optionally_name_servers(&mut self, authority_name_start_of_authority: AuthorityNameStartOfAuthority, store_name_servers_unchecked: impl FnOnce(&mut DomainCache, &DomainTarget)) -> Result<(SolitaryRecords<StartOfAuthority<EfficientCaseFoldedName>>, DomainTarget), AnsweredError>
	{
		let authority_name = authority_name_start_of_authority.authority_name;
		let start_of_authority = authority_name_start_of_authority.start_of_authority;
		
		self.guard_can_have_records(&authority_name)?;
		
		self.store_start_of_authority_unchecked(&authority_name, &start_of_authority);
		store_name_servers_unchecked(self, &authority_name);
		
		Ok((start_of_authority, authority_name))
	}
	
	#[inline(always)]
	fn store_start_of_authority_unchecked(&mut self, authority_name: &DomainTarget, start_of_authority: &SolitaryRecords<StartOfAuthority<EfficientCaseFoldedName>>)
	{
		xxx;
	}
	
	#[inline(always)]
	fn store_name_servers_unchecked(&mut self, authority_name: &DomainTarget, name_servers: MultipleSortedRecords<NameServerName<EfficientCaseFoldedName>>)
	{
		xxx;
	}
	
	#[inline(always)]
	fn store_no_domain_unchecked(&mut self, most_canonical_name: DomainTarget, no_domain_cache_entry: NoDomainCacheEntry)
	{
		xxx;
	}
	
	#[inline(always)]
	fn guard_can_be_an_alias(&self, alias: &Alias) -> Result<(), AnsweredError>
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
	fn guard_can_be_replaced_by_no_data(&self, domain_target: &DomainTarget) -> Result<(), AnsweredError>
	{
		self.guard_can_have_records(domain_target)
	}
	
	#[inline(always)]
	fn guard_can_be_replaced_by_no_domain(&self, domain_target: &DomainTarget) -> Result<(), AnsweredError>
	{
		if let Some(domain_cache_entry) = self.map.get(domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_be_replaced_by_no_domain())
			{
				return Err(AnsweredError::DomainNameCanNotNotExist(most_canonical_name))
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn guard_can_have_records(&self, alias_or_domain_target: &AliasOrDomainTarget) -> Result<(), AnsweredError>
	{
		if let Some(domain_cache_entry) = self.map.get(alias_or_domain_target)
		{
			if unlikely!(domain_cache_entry.can_not_have_records())
			{
				return Err(AnsweredError::DomainNameCanNotNotHaveRecords(most_canonical_name))
			}
		}
		Ok(())
	}
}

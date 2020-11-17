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
	
	pub(crate) fn answered<'message, PR: ParsedRecord>(&mut self, now: NanosecondsSinceUnixEpoch, query_name: &'message EfficientCaseFoldedName, domain_cache: &mut DomainCache, answer: Answer<Self::PR<'message>>, canonical_name_chain_records: CanonicalNameChainRecords, delegation_names: DelegationNames, store_records_in_query_types_cache: impl FnOnce(&mut QueryTypesCache, OwnerNameToRecordValue<PR>)) -> Result<(), AnsweredError>
	{
		use self::AnsweredError::*;
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		use self::NoDomainResponseType::*;
		use self::NoDataResponseType::*;
		
		self.replace_canonical_name_chain_records(canonical_name_chain_records)?;
		
		match answer
		{
			Answer::Answered { most_canonical_name, records } =>
			{
				debug_assert!(!records.is_empty());
				
				match self.map.entry(most_canonical_name)
				{
					Vacant(vacant) =>
					{
						vacant.insert(DomainCacheEntry::answered(store_records_in_query_types_cache, records))
					}
					
					Occupied(occupied) =>
					{
						match occupied.get_mut()
						{
							&mut NeverValid => return Err(DomainNameCanNotNotHaveRecords(occupied.replace_key())),
							
							&mut AlwaysValid { ref mut cache } => store_records_in_query_types_cache(cache, records),
							
							&mut CurrentlyValid { ref mut cache } => store_records_in_query_types_cache(cache, records),
							
							x @ &mut Alias { .. } => *x = DomainCacheEntry::answered(store_records_in_query_types_cache, records),
							
							x @ &mut NoDomain(_) => *x = DomainCacheEntry::answered(store_records_in_query_types_cache, records),
						}
					}
				}
			},
			
			Answer::NoDomain { most_canonical_name, response_type} =>
			{
				// The most_canonical_name is not necessarily the child of the authority name; it can be a grandchild, etc, and so we could infer that the intermediate domains are NXDOMAIN, too.
				
				NoDomainCacheEntry::store(most_canonical_name, response_type, self)?;
				
				// // TODO: Implicit referral!
			}
			
			Answer::NoData { most_canonical_name, response_type} =>
			{
				self.guard_can_have_records(&most_canonical_name)?;
				// // TODO: Now what?
				store_records_in_query_types_cache(TODO_query_types_cache).empty(most_canonical_name)
				
				// // TODO: Implicit referral!
			}
			
			Answer::Referral { most_canonical_name, referral} =>
			{
				let authority_name = referral.authority_name;
				self.guard_can_have_records(&authority_name)?;
				self.store_name_servers_unchecked(authority_name, referral.name_servers);
				
				// // TODO: referral!
			}
		}
		
		Ok(())
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
	
	/*
		TODO: Canonical name chain
			- For each entry in the chain, store a direct (rather than multiple hops) with the lowest TTL.
		
	 */
	#[inline(always)]
	fn replace_canonical_name_chain_records(&mut self, canonical_name_chain_records: CanonicalNameChainRecords) -> Result<(), AnsweredError>
	{
		// Check for problems before we mutate the map.
		for alias in canonical_name_chain_records.keys()
		{
			self.guard_can_be_an_alias(alias)?;
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
	fn store_start_of_authority_unchecked(&mut self, authority_name: DomainTarget, start_of_authority: SolitaryRecords<StartOfAuthority<EfficientCaseFoldedName>>)
	{
		xxx;
	}
	
	#[inline(always)]
	fn store_name_servers_unchecked(&mut self, authority_name: DomainTarget, name_servers: MultipleSortedRecords<NameServerName<EfficientCaseFoldedName>>)
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

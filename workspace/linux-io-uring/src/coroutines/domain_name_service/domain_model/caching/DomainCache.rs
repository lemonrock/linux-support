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
		
		let most_canonical_name = self.replace_canonical_name_chain(canonical_name_chain)?;
		
		match answer
		{
			Answer::Answered { records } =>
			{
				debug_assert!(!records.is_empty());
				
				match self.map.entry(most_canonical_name.clone())
				{
					Vacant(vacant) =>
					{
						vacant.insert(DomainCacheEntry::store(sriaqtc, records))
					}
					
					Occupied(occupied) =>
					{
						match occupied.get_mut()
						{
							&mut NeverValid => return Err(DomainNameCanNotNotHaveRecords(occupied.replace_key())),
							
							&mut AlwaysValid { ref mut cache } => PR::store(cache, records),
							
							&mut CurrentlyValid { ref mut cache } => PR::store(cache, records),
							
							x @ &mut Alias { .. } => *x = DomainCacheEntry::store(records),
							
							x @ &mut NoDomain(_) => *x = DomainCacheEntry::store(records),
						}
					}
				}
			},
			
			Answer::NoDomain { response_type} =>
			{
				// TODO: The most_canonical_name is not necessarily the child of the authority name; it can be a grandchild, etc, and so we could infer that the intermediate domains are NXDOMAIN, too.
				
				// TODO: Verify this code.
				NoDomainCacheEntry::store(most_canonical_name, response_type, self)?;
				
				// // TODO: Implicit referral!
			}
			
			Answer::NoData { response_type} =>
			{
				// TODO: Store SOA and NS.
				xxx;
				
				// // TODO: Implicit referral!
				
				match self.map.entry(most_canonical_name.clone())
				{
					Vacant(vacant) =>
					{
						vacant.insert(DomainCacheEntry::no_data(records))
					}
					
					Occupied(occupied) =>
					{
						match occupied.get_mut()
						{
							&mut NeverValid => return Err(DomainNameCanNotNotHaveRecords(occupied.replace_key())),
							
							&mut AlwaysValid { ref mut cache } => PR::no_data(cache, records),
							
							&mut CurrentlyValid { ref mut cache } => PR::no_data(cache, records),
							
							x @ &mut Alias { .. } => *x = DomainCacheEntry::no_data(records),
							
							x @ &mut NoDomain(_) => *x = DomainCacheEntry::no_data(records),
						}
					}
				}
			}
			
			Answer::Referral { referral} =>
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

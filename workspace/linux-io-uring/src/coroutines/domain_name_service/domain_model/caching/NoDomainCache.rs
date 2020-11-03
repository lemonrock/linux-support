// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct NoDomainCache<'cache>
{
	always_valid_domain_names: HashSet<EfficientCaseFoldedName>,
	
	never_valid_domain_names: HashSet<EfficientCaseFoldedName>,
	
	least_recently_used_cache: LeastRecentlyUsedCache<'cache, NoDomainCacheEntry>,
}

impl<'cache> NoDomainCache<'cache>
{
	const Missing: bool = false;
	
	const HasNoDomain: bool = true;
	
	/// `always_valid_domain_names` and `never_valid_domain_names` must be disjoint.
	#[inline(always)]
	pub(crate) fn new(maximum_records_count: NonZeroUsize, always_valid_domain_names: HashSet<EfficientCaseFoldedName>, never_valid_domain_names: HashSet<EfficientCaseFoldedName>) -> Self
	{
		debug_assert!(always_valid_domain_names.is_disjoint(&never_valid_domain_names));
		
		Self
		{
			always_valid_domain_names,
			
			never_valid_domain_names,
			
			least_recently_used_cache: LeastRecentlyUsedCache::new(maximum_records_count)
		}
	}
	
	/// Does this name have a domain?
	#[inline(always)]
	pub fn recursive_existence(&mut self, name: &EfficientCaseFoldedName, now: NanosecondsSinceUnixEpoch) -> NoDomainCacheResult
	{
		use self::NoDomainCacheResult::*;
		
		if unlikely!(name.is_root())
		{
			return DefinitivelyHasADomain
		}
		
		match self.existence(name, now)
		{
			DefinitivelyHasADomain => return DefinitivelyHasADomain,
			DefinitivelyDoesNotHaveADomain => return DefinitivelyDoesNotHaveADomain,
			Unknown => (),
		}
		
		let mut current = name.parent().expect("Already tested for root");
		while likely!(!current.is_root())
		{
			match self.existence(&current, now)
			{
				DefinitivelyHasADomain => return DefinitivelyHasADomain,
				DefinitivelyDoesNotHaveADomain => return DefinitivelyDoesNotHaveADomain,
				Unknown => (),
			}
			
			current = current.parent().expect("Loop tests for root");
		}
		
		DefinitivelyHasADomain
	}
	
	/// Gets a result for the name.
	#[inline(always)]
	pub fn existence(&mut self, name: &EfficientCaseFoldedName, now: NanosecondsSinceUnixEpoch) -> NoDomainCacheResult
	{
		use self::NoDomainCacheResult::*;
		
		if unlikely!(name.is_root())
		{
			return DefinitivelyHasADomain
		}
		
		if self.always_valid_domain_names.contains(name)
		{
			return DefinitivelyHasADomain
		}
		
		if self.never_valid_domain_names.contains(name)
		{
			return DefinitivelyDoesNotHaveADomain
		}
		
		use self::NoDomainCacheEntry::*;
		
		const RemoveEntry: bool = true;
		const KeepEntry: bool = false;
		
		let (present, remove_entry) = match self.least_recently_used_cache.get_mut(name)
		{
			None => return Unknown,
			
			Some(&mut AbsentUseOnce) => (DefinitivelyDoesNotHaveADomain, RemoveEntry),
			
			Some(&mut Present(negative_cache_until)) => if negative_cache_until < now
			{
				(Unknown, RemoveEntry)
			}
			else
			{
				(DefinitivelyDoesNotHaveADomain, KeepEntry)
			},
		};
		
		if remove_entry
		{
			self.least_recently_used_cache.remove(name)
		}
		
		present
	}
	
	#[inline(always)]
	pub(crate) fn put_use_once_if_no_better_record<'message>(&mut self, name: EfficientCaseFoldedName, now: NanosecondsSinceUnixEpoch)
	{
		use self::NoDomainCacheResult::*;
		
		match self.existence(&name, now)
		{
			DefinitivelyHasADomain | DefinitivelyDoesNotHaveADomain => return,
			_ => (),
		}
		
		self.least_recently_used_cache.put(name, NoDomainCacheEntry::AbsentUseOnce);
	}
	
	#[inline(always)]
	pub(crate) fn put<'message>(&mut self, name: EfficientCaseFoldedName, negative_cache_until: NegativeCacheUntil)
	{
		self.least_recently_used_cache.put(name, NoDomainCacheEntry::from(negative_cache_until));
	}
}

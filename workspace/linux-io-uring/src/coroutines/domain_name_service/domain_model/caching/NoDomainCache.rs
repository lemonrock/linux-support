// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct NoDomainCache<'cache>
{
	always_valid_domain_names: HashSet<CaseFoldedName<'cache>>,
	
	never_valid_domain_names: HashSet<CaseFoldedName<'cache>>,
	
	least_recently_used_cache: LeastRecentlyUsedCache<'cache, NoDomainCacheEntry>,
}

impl<'cache> NoDomainCache<'cache>
{
	const Missing: bool = false;
	
	const HasNoDomain: bool = true;
	
	/// `always_valid_domain_names` and `never_valid_domain_names` must be disjoint.
	#[inline(always)]
	pub(crate) fn new(maximum_records_count: NonZeroUsize, always_valid_domain_names: HashSet<CaseFoldedName<'cache>>, never_valid_domain_names: HashSet<CaseFoldedName<'cache>>) -> Self
	{
		debug_assert!(always_valid_domain_names.is_disjoint(&never_valid_domain_names));
		
		Self
		{
			always_valid_domain_names,
			
			never_valid_domain_names,
			
			least_recently_used_cache: LeastRecentlyUsedCache::new(maximum_records_count)
		}
	}
	
	#[inline(always)]
	pub fn has_no_domain(&mut self, name: &'cache CaseFoldedName<'cache>, now: NanosecondsSinceUnixEpoch) -> bool
	{
		let mut current = Cow::Borrowed(name);
		
		while !current.is_root()
		{
			if self.get(&current, now)
			{
				return Self::HasNoDomain
			}
			
			current = Cow::Owned(name.parent().unwrap());
		}
		
		Self::Missing
	}
	
	/// Gets a result for the name.
	#[inline(always)]
	pub fn get(&mut self, name: &CaseFoldedName<'cache>, now: NanosecondsSinceUnixEpoch) -> bool
	{
		if self.always_valid_domain_names.contains(name)
		{
			return Self::Missing
		}
		
		if self.never_valid_domain_names.contains(name)
		{
			return Self::HasNoDomain
		}
		
		use self::NoDomainCacheEntry::*;
		
		const RemoveEntry: bool = true;
		const KeepEntry: bool = false;
		
		let (present, remove_entry) = match self.least_recently_used_cache.get_mut(name)
		{
			None => return Self::Missing,
			
			Some(&mut AbsentUseOnce) => (Self::HasNoDomain, RemoveEntry),
			
			Some(&mut Present(negative_cache_until)) => if negative_cache_until < now
			{
				(Self::Missing, RemoveEntry)
			}
			else
			{
				(Self::HasNoDomain, KeepEntry)
			},
		};
		
		if remove_entry
		{
			self.least_recently_used_cache.remove(name)
		}
		
		present
	}
	
	#[inline(always)]
	pub(crate) fn put<'message>(&mut self, name: CaseFoldedName<'cache>, negative_cache_until: NegativeCacheUntil)
	{
		self.least_recently_used_cache.put(name, NoDomainCacheEntry::from(negative_cache_until));
	}
}

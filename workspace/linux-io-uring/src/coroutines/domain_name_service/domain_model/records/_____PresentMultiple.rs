// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.































#[derive(Debug)]
pub(crate) struct PresentMultiple<R: ParsedRecord>
{
	/// One-time use.
	use_once: PriorityToWeightedRecordsMap<R>,
	
	/// Cached.
	cached: BTreeMap<NanosecondsSinceUnixEpoch, PriorityToWeightedRecordsMap<R>>,
}

impl<R: ParsedRecord> Clone for PresentMultiple<R>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			use_once: Clone::clone(&self.use_once),
		
			cached: self.cached.clone(),
		}
	}
}

impl<R: ParsedRecord> Default for PresentMultiple<R>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			use_once: PriorityToWeightedRecordsMap::default(),
			cached: BTreeMap::default(),
		}
	}
}

impl<R: ParsedRecord> PresentMultiple<R>
{
	#[inline(always)]
	pub(crate) fn store_unprioritized_and_unweighted(&mut self, cache_until: CacheUntil, record: R)
	{
		self.store_unweighted(cache_until, Priority::Unassigned, record)
	}
	
	#[inline(always)]
	pub(crate) fn store_unweighted(&mut self, cache_until: CacheUntil, priority_or_preference: Priority, record: R)
	{
		self.store(cache_until, priority_or_preference, Weight::Unassigned, record)
	}
	
	#[inline(always)]
	pub(crate) fn store(&mut self, cache_until: CacheUntil, priority: Priority, weight: Weight, record: R)
	{
		let priority_to_sorted_weighted_records_map = match cache_until
		{
			CacheUntil::UseOnce { as_of_now } => &mut self.use_once,
			
			CacheUntil::Cached { cached_until } => self.cached.entry(cached_until).or_insert_with(PriorityToWeightedRecordsMap::default),
		};
		priority_to_sorted_weighted_records_map.insert(priority, weight, record);
	}
	
	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool
	{
		self.records_count_usize() == 0
	}
	
	#[inline(always)]
	fn records_count_usize(&self) -> usize
	{
		let mut records_count = self.use_once.records_count();
		
		for records in self.cached.values()
		{
			records_count += records.records_count();
		}
		
		records_count
	}
	
	#[inline(always)]
	fn records_count(&self) -> NonZeroUsize
	{
		let records_count = self.records_count_usize();
		new_non_zero_usize(records_count)
	}
	
	fn retrieve(&mut self, now: NanosecondsSinceUnixEpoch) -> (QueryTypeCacheResult<ParsedRecord>, Option<usize>)
	{
		use self::QueryTypeCacheResult::Nothing;
		
		const RemoveEntry: Option<usize> = None;
		
		let (mut records_to_return, mut expired_records_count) = self.remove_use_once_entries();
		
		{
			let expired = self.remove_expired_cached_entries(now);
			for priority_to_sorted_weighted_records_map in expired.values()
			{
				expired_records_count += priority_to_sorted_weighted_records_map.records_count();
			}
			drop(expired);
		};
		
		let only_has_use_once_records = self.cached.is_empty();
		
		if only_has_use_once_records
		{
			if records_to_return.is_empty()
			{
				(Nothing, RemoveEntry)
			}
			else
			{
				(QueryTypeCacheResult::Exists(Exists(records_to_return)), RemoveEntry)
			}
		}
		else
		{
			for priority_to_sorted_weighted_records_map in self.cached.values()
			{
				for (priority, sorted_weighted_records) in priority_to_sorted_weighted_records_map.iter()
				{
					debug_assert!(!sorted_weighted_records.is_empty(), "If sorted_weighted_records is empty, then we've populated `Present` incorrectly");
					
					use std::collections::btree_map::Entry::*;
					match records_to_return.entry(*priority)
					{
						Vacant(vacant) =>
						{
							vacant.insert(sorted_weighted_records.clone());
						},
						
						Occupied(mut occupied_from_use_once) =>
						{
							occupied_from_use_once.get_mut().append(sorted_weighted_records);
						},
					}
				}
			}
			
			debug_assert!(!records_to_return.is_empty());
			(QueryTypeCacheResult::Exists(Exists(records_to_return)), Some(expired_records_count))
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn remove_use_once_entries(&mut self) -> (PriorityToWeightedRecordsMap<ParsedRecord>, usize)
	{
		let use_once = take(&mut self.use_once);
		let expired_records_count = use_once.records_count();
		(use_once, expired_records_count)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn remove_expired_cached_entries(&mut self, now: NanosecondsSinceUnixEpoch) -> BTreeMap<NanosecondsSinceUnixEpoch, PriorityToWeightedRecordsMap<ParsedRecord>>
	{
		let should_still_be_cached = self.cached.split_off(&now);
		let expired = replace(&mut self.cached, should_still_be_cached);
		expired
	}
}
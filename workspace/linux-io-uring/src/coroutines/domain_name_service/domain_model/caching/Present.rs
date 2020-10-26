// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
pub(crate) struct Present<Record: Sized>
{
	/// One-time use.
	use_once: BTreeMap<Priority, SortedWeightedRecords<Record>>,
	
	cached: BTreeMap<NanosecondsSinceUnixEpoch, BTreeMap<Priority, SortedWeightedRecords<Record>>>,
}

impl<Record: Sized> Present<Record>
{
	#[inline(always)]
	fn records_count(&self) -> usize
	{
		let mut records_count = Self::btree_map_records_count(&self.use_once);
		
		for records in self.cached.values()
		{
			records_count += Self::btree_map_records_count(records);
		}
		
		records_count
	}
	
	fn retrieve(&mut self, now: NanosecondsSinceUnixEpoch) -> (CacheResult<Record>, Option<usize>)
	{
		use self::CacheResult::Nothing;
		
		const RemoveEntry: Option<usize> = None;
		
		let (mut records_to_return, mut expired_records_count) = self.remove_use_once_entries();
		
		{
			let expired = self.remove_expired_cached_entries(now);
			for btree_map in expired.values()
			{
				expired_records_count += Self::btree_map_records_count(btree_map);
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
				(CacheResult::Exists(Exists(records_to_return)), RemoveEntry)
			}
		}
		else
		{
			for btree_map in self.cached.values()
			{
				for (priority, sorted_weighted_records) in btree_map
				{
					debug_assert!(!sorted_weighted_records.is_empty(), "If sorted_weighted_records is empty, then we've populated `Present` incorrectly");
					
					use std::collections::btree_map::Entry::*;
					match records_to_return.entry(*priority)
					{
						Vacant(vacant) => vacant.insert(Clone::clone(sorted_weighted_records)),
						
						Occupied(occupied_from_use_once) => occupied_from_use_once.get_mut().append(sorted_weighted_records),
					}
				}
			}
			
			debug_assert!(!records_to_return.is_empty());
			(CacheResult::Exists(Exists(records_to_return)), Some(expired_records_count))
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn remove_use_once_entries(&mut self) -> (BTreeMap<Priority, SortedWeightedRecords<Record>>, usize)
	{
		let use_once = take(&mut self.use_once);
		let expired_records_count = Self::btree_map_records_count(&use_once);
		(use_once, expired_records_count)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn remove_expired_cached_entries(&mut self, now: NanosecondsSinceUnixEpoch) -> BTreeMap<NanosecondsSinceUnixEpoch, BTreeMap<Priority, SortedWeightedRecords<Record>>>
	{
		let should_still_be_cached = self.cached.split_off(&now);
		let expired = replace(&mut self.cached, should_still_be_cached);
		expired
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn insert(btree_map: &mut BTreeMap<Priority, SortedWeightedRecords<Record>>, priority: Priority, weight: Weight, record: Record)
	{
		use std::collections::btree_map::Entry::*;
		match btree_map.entry(priority)
		{
			Vacant(vacant) =>
			{
				vacant.insert(SortedWeightedRecords::new_for_one_record(weight, record));
			},
			
			Occupied(occupied) =>
			{
				debug_assert!(!occupied.is_empty(), "If occupied is empty, then we've populated `Present` incorrectly");
				
				occupied.get_mut().add(weight, record)
			},
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn btree_map_records_count(btree_map: &BTreeMap<Priority, SortedWeightedRecords<Record>>) -> usize
	{
		let mut expired_record_count = 0;
		for expired_sorted_weighed_records in expired_sorted_weighted_records_by_priority.values()
		{
			expired_record_count += expired_record.record_count()
		}
		expired_record_count
	}
}

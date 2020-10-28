// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct PriorityToSortedWeightedRecordsMap<Record: Sized + Debug>(BTreeMap<Priority, SortedWeightedRecords<Record>>);

impl<Record: Sized + Debug> Clone for PriorityToSortedWeightedRecordsMap<Record>
{
	// NOTE: We cannot rely on `#[derive(Clone)]` or `BTreeMap.clone()` as both require `Record` to implement `Clone`; however, since `Record` is actually `Rc<Record>`, this constraint is not valid.
	#[inline(always)]
	fn clone(&self) -> Self
	{
		let mut btree_map = BTreeMap::new();
		for (priority, sorted_weighted_records) in self.iter()
		{
			btree_map.insert(*priority, sorted_weighted_records.clone());
		}
		Self(btree_map)
	}
}

impl<Record: Sized + Debug> Default for PriorityToSortedWeightedRecordsMap<Record>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(BTreeMap::default())
	}
}

impl<Record: Sized + Debug> Deref for PriorityToSortedWeightedRecordsMap<Record>
{
	type Target = BTreeMap<Priority, SortedWeightedRecords<Record>>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<Record: Sized + Debug> DerefMut for PriorityToSortedWeightedRecordsMap<Record>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<Record: Sized + Debug> PriorityToSortedWeightedRecordsMap<Record>
{
	#[inline(always)]
	fn insert(&mut self, priority: Priority, weight: Weight, record: Record)
	{
		use std::collections::btree_map::Entry::*;
		
		match self.entry(priority)
		{
			Vacant(vacant) =>
			{
				vacant.insert(SortedWeightedRecords::new_for_one_record(weight, record));
			},
			
			Occupied(occupied) =>
			{
				let occupied = occupied.get_mut();
				debug_assert!(!occupied.is_empty(), "If occupied is empty, then we've populated `Present` incorrectly");
				
				occupied.add(weight, record)
			},
		}
	}
	
	#[inline(always)]
	fn records_count(&mut self) -> usize
	{
		let mut expired_record_count = 0;
		for expired_sorted_weighed_records in self.0.values()
		{
			expired_record_count += expired_sorted_weighed_records.record_count()
		}
		expired_record_count
	}
}

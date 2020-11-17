// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct PriorityToSortedRecordsMap<OR: ParsedRecord>(BTreeMap<Priority, BTreeSet<OR>>);

impl<OR: OwnedRecord> Default for PriorityToSortedRecordsMap<OR>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(BTreeMap::default())
	}
}

impl<OR: OwnedRecord> Deref for PriorityToSortedRecordsMap<OR>
{
	type Target = BTreeMap<Priority, WeightedRecords<OR>>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<OR: OwnedRecord> DerefMut for PriorityToSortedRecordsMap<OR>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<OR: OwnedRecord> PriorityToSortedRecordsMap<OR>
{
	#[inline(always)]
	fn add(&mut self, priority: Priority, record: OR)
	{
		use std::collections::btree_map::Entry::*;
		
		match self.entry(priority)
		{
			Vacant(vacant) =>
			{
				vacant.insert
				(
					btreeset!
					{
						record
					}
				);
			},
			
			Occupied(mut occupied) =>
			{
				let occupied = occupied.get_mut();
				debug_assert!(!occupied.is_empty(), "If occupied is empty, then we've populated `PriorityToSortedRecordsMap` incorrectly");
				
				occupied.insert(record);
			},
		}
	}
}

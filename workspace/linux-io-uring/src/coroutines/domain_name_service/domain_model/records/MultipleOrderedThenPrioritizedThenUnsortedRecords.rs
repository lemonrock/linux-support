// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For the record type:-
///
/// * NAPTR.
pub struct MultipleOrderedThenPrioritizedThenUnsortedRecords<OR: OwnedRecord>
{
	records: BTreeMap<Order, PriorityToUnsortedRecordsMap<OR>>,
}

impl<OR: OwnedRecord> OwnedRecords<OR> for MultipleOrderedThenPrioritizedThenUnsortedRecords<OR>
{
}

impl<PR: ParsedRecord<OrderPriorityAndWeight=(Order, Priority), OwnedRecord=OR>, OR: OwnedRecord> From<OwnerNameToRecordsValue<PR>> for MultipleOrderedThenPrioritizedThenUnsortedRecords<OR>
{
	#[inline(always)]
	fn from(value: OwnerNameToRecordsValue<PR>) -> Self
	{
		Self
		{
			records:
			{
				let mut records = BTreeMap::default();
				for (record, (order, priority)) in value.records
				{
					use std::collections::btree_map::Entry::*;
					
					let record = record.into_owned_record();
					match records.entry(order)
					{
						Vacant(vacant) =>
						{
							vacant.insert(PriorityToUnsortedRecordsMap::new_for_one(priority, record));
						},
						
						Occupied(mut occupied) =>
						{
							let occupied = occupied.get_mut();
							debug_assert!(!occupied.is_empty(), "If occupied is empty, then we've populated `PriorityToSortedRecordsMap` incorrectly");
							
							occupied.add(priority, record);
						},
					}
				}
				records
			},
		}
	}
}

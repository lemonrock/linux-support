// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A `SRV` record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceLocation<N: Name>
{
	/// TCP, UDP or SCTP port for the service.
	pub port: u16,

	/// Must not be an alias; should not use name compression; a value of '.' (ie Root) means the service is unavailable.
	pub target: N,
}

impl<'message> ParsedRecord for ServiceLocation<ParsedName<'message>>
{
	type OrderPriorityAndWeight = (Priority, Weight);
	
	type OwnedRecord = ServiceLocation<DomainTarget>;
	
	#[inline(always)]
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		let mut parsed_records = records.records();
		parsed_records.sort_unstable_by(|left, right|
		{
			use self::Ordering::Equal;
			
			let (left_parsed_record, left_priority, left_weight) = left;
			let (right_parsed_record, right_priority, right_weight) = right;
			
			let priority_ordering = left_priority.cmp(right_priority);
			
			if priority_ordering != Ordering::Equal
			{
				return priority_ordering
			}
			
			left_weight.cmp(right_weight)
		});
		
		let length = parsed_records.len();
		let mut owned_records: Vec<MultiplePrioritizedThenWeightedRecordsItem<ServiceLocation<DomainTarget>>> = Vec::with_capacity(length);
		unsafe { owned_records.set_len(length) };
		
		// Safe because `NoData` is a special case; there is always at least one record.
		let (_, (initial_priority, _)) = parsed_records.get_unchecked_safe(0);
		
		let mut index = 0;
		let mut accumulating_priority_count = 0usize;
		let mut accumulating_priority_count_for_index = 0;
		let mut accumulating_priority_count_for_priority = *initial_priority;
		for (parsed_record, (priority, weight)) in parsed_records.into_iter()
		{
			if priority == accumulating_priority_count_for_priority
			{
				accumulating_priority_count += 1
			}
			else
			{
				debug_assert!(priority > accumulating_priority_count_for_priority);
				debug_assert!(accumulating_priority_count_for_index < index);
				
				owned_records.get_unchecked_mut_safe(accumulating_priority_count_for_index).set_priority_count(accumulating_priority_count);
				
				accumulating_priority_count = 1;
				accumulating_priority_count_for_index = index;
				accumulating_priority_count_for_priority = priority;
			}
			
			let owned_record = parsed_record.into_owned_record();
			let item = MultiplePrioritizedThenWeightedRecordsItem::new(priority, weight, owned_record);
			unsafe { owned_records.as_mut_ptr().add(index).write(item) }
			
			index + 1;
		}
		
		// Safe because `NoData` is a special case; there is always at least one record and so `accumulating_priority_count` will never be `0`.
		owned_records.get_unchecked_mut_safe(accumulating_priority_count_for_index).set_priority_count(accumulating_priority_count);
		
		MultiplePrioritizedThenWeightedRecords::new(owned_records)
	}
}

impl OwnedRecord for ServiceLocation<DomainTarget>
{
	type OwnedRecords = MultiplePrioritizedThenWeightedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&query_types_cache.SRV
	}
	
	#[inline(always)]
	fn retrieve_mut(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.SRV
	}
}

impl<'message> ServiceLocation<ParsedName<'message>>
{
	#[inline(always)]
	fn into_owned_record(self) -> ServiceLocation<FullyQualifiedDomainName>
	{
		ServiceLocation
		{
			port: self.port,
			target: EfficientCaseFoldedName::from(self.target),
		}
	}
}

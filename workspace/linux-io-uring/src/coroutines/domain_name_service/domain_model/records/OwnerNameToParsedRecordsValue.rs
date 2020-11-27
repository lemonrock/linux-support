// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct OwnerNameToParsedRecordsValue<PR: ParsedRecord>
{
	cache_until: CacheUntil,
	records: Vec<(PR, PR::OrderPriorityAndWeight)>
}

impl<PR: ParsedRecord> OwnerNameToParsedRecordsValue<PR>
{
	#[inline(always)]
	pub(crate) fn solitary(self) -> PR
	{
		debug_assert_eq!(self.records.len(), 1);
		self.records.pop().unwrap().0
	}
	
	#[inline(always)]
	pub(crate) fn cache_until(&self) -> CacheUntil
	{
		self.cache_until
	}
	
	#[inline(always)]
	fn new_for_one(cache_until: CacheUntil, record: PR, order_priority_and_weight: PR::OrderPriorityAndWeight) -> Self
	{
		Self
		{
			cache_until,
			records: vec!
			[
				(record, order_priority_and_weight)
			]
		}
	}
	
	/// RFC 2181, Section 5.2 TTLs of RRs in an RRSet, Paragraph 2: "… the use of differing TTLs in an RRSet is hereby deprecated, the TTLs of all RRs in an RRSet must be the same".
	/// RFC 2181, Section 5.2 TTLs of RRs in an RRSet, Paragraph 3: "… Should an authoritative source send such a malformed RRSet, the client should treat the RRs for all purposes as if all TTLs in the RRSet had been set to the value of the lowest TTL in the RRSet".
	/// RFC 8499, Section 5. Resource Records clarifies that the above do not apply to `RRSIG` resource records (it does so by reference to RFC 4035).
	#[inline(always)]
	fn add(&mut self, cache_until: CacheUntil, record: PR, order_priority_and_weight: PR::OrderPriorityAndWeight)
	{
		self.cache_until.update(cache_until);
		self.records.push((record, order_priority_and_weight))
	}
	
	#[inline(always)]
	pub(crate) fn records(self) -> Vec<(PR, PR::OrderPriorityAndWeight)>
	{
		self.records
	}
}

impl<PR: ParsedRecord<OrderPriorityAndWeight=(Priority, Weight)>> OwnerNameToParsedRecordsValue<PR>
{
	#[inline(always)]
	fn sort(mut self)
	{
		self.records.sort_unstable_by(|left, right|
		{
			use self::Ordering::Equal;
			
			let (left_parsed_record, (left_priority, left_weight)) = left;
			let (right_parsed_record, (right_priority, right_weight)) = right;
			
			let priority_ordering = left_priority.cmp(right_priority);
			
			if priority_ordering != Ordering::Equal
			{
				return priority_ordering
			}
			
			match (left_weight, right_weight)
			{
			
			}
		})
	}
}

impl<PR: ParsedRecord<OrderPriorityAndWeight=Priority> + Ord> OwnerNameToParsedRecordsValue<PR>
{
	#[inline(always)]
	fn sort(mut self)
	{
		self.records.sort_unstable_by(|left, right|
		{
			use self::Ordering::Equal;
			
			let (left_parsed_record, left_priority) = left;
			let (right_parsed_record, right_priority) = right;
			
			let priority_ordering = left_priority.cmp(right_priority);
			
			if priority_ordering != Ordering::Equal
			{
				return priority_ordering
			}
			
			left_parsed_record.cmp(right_parsed_record)
		})
	}
}

impl<PR: ParsedRecord<OrderPriorityAndWeight=(Order, Priority)>> OwnerNameToParsedRecordsValue<PR>
{
	#[inline(always)]
	fn sort(mut self)
	{
		self.records.sort_unstable_by(|left, right|
		{
			let (left_parsed_record, (left_order, left_priority)) = left;
			let (right_parsed_record, (right_order, right_priority)) = right;
			
			let order_ordering = left_order.cmp(right_order);
			if order_ordering != Ordering::Equal
			{
				return order_ordering
			}
			
			left_priority.cmp(right_priority)
		})
	}
}

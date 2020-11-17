// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug)]
pub(crate) struct OwnerNameToRecords<'message, PR: ParsedRecord>(HashMap<EitherName<'message>, OwnerNameToRecordsValue<PR>>);

impl<'message, PR: ParsedRecord> OwnerNameToRecords<'message, PR>
{
	#[inline(always)]
	pub(crate) fn with_capacity(capacity: usize) -> Self
	{
		Self(HashMap::with_capacity(capacity))
	}
	
	#[inline(always)]
	pub(crate) fn add(self, owner_name: ParsedName<'message>, cache_until: CacheUntil, record: PR, order_priority_and_weight: PR::OrderPriorityAndWeight)
	{
		use self::FastSecureHashMapEntry::*;
		
		match self.0.entry(EitherName::Parsed(owner_name))
		{
			Vacant(vacant) =>
			{
				vacant.insert(OwnerNameToRecordsValue::new_for_one(cache_until, record, order_priority_and_weight));
			}
			
			Occupied(occupied) =>
			{
				occupied.get_mut().add(cache_until, record, order_priority_and_weight);
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn filter(self, most_canonical_name: &EfficientCaseFoldedName) -> Option<OwnerNameToRecordsValue<PR>>
	{
		self.0.remove(&EitherName::EfficientCaseFolded(most_canonical_name.clone()))
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct MultiplePrioritizedThenWeightedRecordsItem<OR: OwnedRecord>
{
	priority: Priority,
	
	priority_count: Option<NonZeroUsize>,
	
	weight: Weight,
	
	record: OR,
}

impl<OR: OwnedRecord> MultiplePrioritizedThenWeightedRecordsItem<OR>
{
	#[inline(always)]
	pub(crate) const fn new(priority: Priority, weight: Weight, record: OR) -> Self
	{
		Self
		{
			priority,
			priority_count: None,
			weight,
			record,
		}
	}
	
	#[inline(always)]
	pub(crate) fn set_priority_count(&mut self, priority_count: usize)
	{
		debug_assert!(self.priority_count.is_none());
		
		self.priority_count = Some(new_non_zero_usize(priority_count))
	}
}

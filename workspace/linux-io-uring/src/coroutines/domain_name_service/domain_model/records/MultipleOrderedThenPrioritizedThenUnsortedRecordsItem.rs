// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<OR: OwnedRecord>
{
	order: Order,
	
	order_count: Option<NonZeroUsize>,
	
	priority: Priority,
	
	priority_count: Option<NonZeroUsize>,
	
	record: OR,
}

impl<OR: OwnedRecord> MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<OR>
{
	#[inline(always)]
	pub(crate) const fn new(order: Order, priority: Priority, record: OR) -> Self
	{
		Self
		{
			order,
			order_count: None,
			priority,
			priority_count: None,
			record,
		}
	}
	
	#[inline(always)]
	pub(crate) fn set_order_count(&mut self, order_count: usize)
	{
		debug_assert!(self.order_count.is_none());
		
		self.order_count = Some(new_non_zero_usize(order_count))
	}
	
	#[inline(always)]
	pub(crate) fn set_priority_count(&mut self, priority_count: usize)
	{
		debug_assert!(self.priority_count.is_none());
		
		self.priority_count = Some(new_non_zero_usize(priority_count))
	}
}

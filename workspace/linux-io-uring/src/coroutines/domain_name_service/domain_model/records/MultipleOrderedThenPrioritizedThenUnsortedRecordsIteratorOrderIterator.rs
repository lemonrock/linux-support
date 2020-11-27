// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An iterator.
#[derive(Debug, Clone)]
pub struct MultipleOrderedThenPrioritizedThenUnsortedRecordsIteratorOrderIterator<'a, OR: OwnedRecord + 'a>
{
	source: &'a Vec<MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<OR>>,

	next_order_index: usize,

	exclusive_last_order_index: usize,
}

impl<'a, OR: OwnedRecord + 'a> Iterator for MultipleOrderedThenPrioritizedThenUnsortedRecordsIteratorOrderIterator<'a, OR>
{
	type Item = MultipleOrderedThenPrioritizedThenUnsortedRecordsIteratorOrderIteratorPriorityIterator<'a, OR>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let current_next_order_index = self.next_order_index;
		
		if unlikely!(current_next_order_index == self.exclusive_last_order_index)
		{
			None
		}
		else
		{
			let item = self.source.get_unchecked_safe(current_next_order_index);
			
			let priority_count = item.priority_count.unwrap().get();
			let new_index = current_next_order_index + priority_count;
			
			let iterator = MultipleOrderedThenPrioritizedThenUnsortedRecordsIteratorOrderIteratorPriorityIterator
			{
				source: self.source,
				
				next_index: current_next_order_index,
				
				exclusive_last_index: new_index
			};
			
			self.next_order_index = new_index;
			
			Some(iterator)
		}
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let remaining_size = self.exclusive_last_order_index - self.next_order_index;
		
		let lower = if unlikely!(remaining_size == 0)
		{
			0
		}
		else
		{
			1
		};
		
		(remaining_size, Some(remaining_size))
	}
}

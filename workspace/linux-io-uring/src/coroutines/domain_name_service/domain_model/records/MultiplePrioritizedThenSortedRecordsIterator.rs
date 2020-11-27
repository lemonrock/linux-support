// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An iterator by priority.
#[derive(Debug)]
pub struct MultiplePrioritizedThenSortedRecordsIterator<'a, OR: OwnedRecord + 'a>
{
	source: &'a Vec<MultiplePrioritizedThenSortedRecordsItem<OR>>,
	
	next_priority_starts_at_index: usize,
}

impl<'a, OR: OwnedRecord + 'a> Iterator for MultiplePrioritizedThenSortedRecordsIterator<'a, OR>
{
	type Item = MultiplePrioritizedThenSortedRecordsIteratorPriorityIterator<'a, OR>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let current_next_priority_starts_at_index = self.next_priority_starts_at_index;
		
		if unlikely!(current_next_priority_starts_at_index == self.source.len())
		{
			None
		}
		else
		{
			let item = self.source.get_unchecked_safe(current_next_priority_starts_at_index);
			
			let priority_count = item.priority_count.unwrap().get();
			let new_next_priority_starts_at_index = current_next_priority_starts_at_index + priority_count;
			
			let iterator = MultiplePrioritizedThenSortedRecordsIteratorPriorityIterator
			{
				source: self.source,
				
				next_index: current_next_priority_starts_at_index,
				
				exclusive_last_index: new_next_priority_starts_at_index,
			};
			
			self.next_priority_starts_at_index = new_next_priority_starts_at_index;
			
			Some(iterator)
		}
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let remaining_size = self.source.len() - self.next_priority_starts_at_index;
		
		let lower = if unlikely!(remaining_size == 0)
		{
			0
		}
		else
		{
			1
		};
		
		(lower, Some(remaining_size))
	}
}

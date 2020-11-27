// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An iterator.
#[derive(Debug, Clone)]
pub struct MultipleOrderedThenPrioritizedThenUnsortedRecordsIteratorOrderIteratorPriorityIterator<'a, OR: OwnedRecord + 'a>
{
	source: &'a Vec<MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<OR>>,
	
	next_index: usize,
	
	exclusive_last_index: usize,
}

impl<'a, OR: OwnedRecord + 'a> Iterator for MultipleOrderedThenPrioritizedThenUnsortedRecordsIteratorOrderIteratorPriorityIterator<'a, OR>
{
	type Item = &'a OR;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let next_index = self.next_index;
		if unlikely!(next_index == self.exclusive_last_index)
		{
			None
		}
		else
		{
			let item = self.source.get_unchecked_safe(next_index);
			
			self.next_index = next_index + 1;
			
			Some(&item.record)
		}
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let remaining_size = self.exclusive_last_index - self.next_index;
		
		(remaining_size, Some(remaining_size))
	}
}

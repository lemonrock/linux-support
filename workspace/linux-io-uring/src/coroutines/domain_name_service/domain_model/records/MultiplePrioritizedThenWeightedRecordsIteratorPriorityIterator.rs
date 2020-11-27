// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An iterator that randomly selects by weighting.
///
/// Approach designed to support RFC 2782, Page 3, "Weight", paragraph 2 to Page 4, paragraph 3.
#[derive(Debug)]
pub struct MultiplePrioritizedThenWeightedRecordsIteratorPriorityIterator<'a, OR: OwnedRecord + 'a>
{
	weightless: Vec<&'a OR>,
	
	weighted: Vec<(NonZeroU16, &'a OR)>,
	
	current_sum_of_all_weighted: u64,
}

impl<'a, OR: OwnedRecord + 'a> Iterator for MultiplePrioritizedThenWeightedRecordsIteratorPriorityIterator<'a, OR>
{
	type Item = &'a OR;
	
	/// Approach designed to support RFC 2782, Page 3, "Weight", paragraph 2 to Page 4, paragraph 3.
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let weighted_length = self.weighted.len();
		
		// (1) We have no weighted records, so we can only choose a record from the weightless list.
		if weighted_length == 0
		{
			debug_assert_eq!(self.current_sum_of_all_weighted, 0);
			return self.weightless.pop()
		}
		
		let random_running_sum_of_weight = self.random_running_sum_of_weight();
		
		// (2) We have weighted records but we need to first test the weightless list, as these sort before weighted.
		if random_running_sum_of_weight == 0
		{
			if let Some(record) = self.weightless.pop()
			{
				return Some(record)
			}
		}
		
		// (3) Loop over weighted records, finding the first record that has a `running_sum_of_weights` equal to or greater than the random `random_running_sum_of_weight`.
		
		// (3.1) Optimization to avoid loop if there is only one weighted record.
		if unlikely!(weighted_length == 1)
		{
			let (weight, record) = self.weighted.remove(0);
			debug_assert_eq!(weight.get() as u64, self.current_sum_of_all_weighted);
			self.current_sum_of_all_weighted = 0;
			return Some(record)
		}
		
		// (3.2) Loop.
		let mut running_sum_of_weights = 0;
		for index in 0 .. weighted_length
		{
			running_sum_of_weights += self.get_next_weight(index);
			
			if running_sum_of_weights >= random_running_sum_of_weight
			{
				let (weight, record) = self.weighted.remove(index);
				self.current_sum_of_all_weighted -= weight.get() as u64;
				return Some(record)
			}
		}
		
		// (4) The range possible for `random_running_sum_of_weight` can not exceed `self.current_sum_of_all_weighted`.
		// We can only get to here if we did not check for a length of `0` at (1).
		unreachable_code(format_args!(""));
	}
}

impl<'a, OR: OwnedRecord + 'a> MultiplePrioritizedThenWeightedRecordsIteratorPriorityIterator<'a, OR>
{
	fn new(source: &'a Vec<MultiplePrioritizedThenWeightedRecordsItem<OR>>, next_priority_index: usize, exclusive_last_priority_index: usize) -> Self
	{
		let maximum_size = exclusive_last_priority_index - next_priority_index;
		
		let mut weightless = Vec::with_capacity(maximum_size);
		let mut weighted = Vec::with_capacity(maximum_size);
		let mut current_sum_of_all_weighted = 0;
		
		for index in next_priority_index .. exclusive_last_priority_index
		{
			let item = source.get_unchecked_safe(index);
			let record = &item.record;
			if item.weight.is_weightless()
			{
				weightless.push(record)
			}
			else
			{
				let weight = weight.into_non_zero_u16();
				weighted.push((weight, record));
				current_sum_of_all_weighted += weight.get() as u64;
			}
		}
		
		Self
		{
			weightless,
			weighted,
			current_sum_of_all_weighted,
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn random_running_sum_of_weight(&self) -> u64
	{
		const inclusive_minimum: u64 = 0;
		
		let inclusive_maximum = self.current_sum_of_all_weighted;
		let random_value = fast_slightly_insecure_random_u64();
		
		// Technically, not biased, if `InclusiveMaximum - InclusiveMinimum + 1` is a power of 2.
		// See <https://ericlippert.com/2013/12/16/how-much-bias-is-introduced-by-the-remainder-technique/>
		let biased_uniform_distributed = inclusive_minimum + (random_value % (inclusive_maximum - inclusive_minimum + 1));
		
		biased_uniform_distributed
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn get_next_weight(&self, index: usize) -> u64
	{
		let (weight, _record) = self.weighted.get_unchecked_safe(index);
		(*weight).get() as u64
	}
}

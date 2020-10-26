// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Desgined to implement the `SRV` and `URI` prioritized, weighted records algorithm specified in RFC 2782, Page 3, "Weight", paragraph 2 to Page 4, paragraph 3, and, by extension, to support all other record kinds, including those that have a preference (eg `MX`) as well as those that don't (eg `A`).
///
/// This implementation of that algorithm fails if there are `(2^64 - 1) / (2^16 - 1)` or more `self.weighted` records, which is not possible for DNS.
#[derive(Debug, Clone)]
pub(crate) struct SortedWeightedRecords<Record: Sized>
{
	weightless: Vec<Rc<Record>>,
	
	weighted: Vec<(NonZeroU16, Rc<Record>)>,
	
	current_sum_of_all_weighted: u64,
}

impl<Record: Sized> Iterator for SortedWeightedRecords<Record>
{
	type Item = Rc<Record>;
	
	/// Approach designed to support RFC 2782, Page 3, "Weight", paragraph 2 to Page 4, paragraph 3.
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let weighted_length = self.weighted.len();
		
		// (1) We have no weighted records, so we can only choose a record from the weightless list.
		// In practice, only `SRV` and `URI` records will be weighted, so this is an effective optimization.
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
		if unlikely!(length == 1)
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
		unreachable!();
	}
}

impl<Record: Sized> SortedWeightedRecords<Record>
{
	#[inline(always)]
	fn new_for_one_record(weight: Weight, record: Record) -> Self
	{
		if likely!(weight.is_weightless())
		{
			Self
			{
				weightless: vec!
				[
					record
				],
				
				weighted: Vec::default(),
			
				current_sum_of_all_weighted: 0,
			}
		}
		else
		{
			let weight_non_zero_u16 = weight.into_non_zero_u16();
			Self
			{
				weightless: Vec::default(),
				
				weighted: vec!
				[
					(weight.into_non_zero_u16(), record)
				],
				
				current_sum_of_all_weighted: weight_non_zero_u16.get() as u64,
			}
		}
	}
	
	#[inline(always)]
	fn add(&mut self, weight: Weight, record: Record)
	{
		let record = Rc::new(record);
		if likely!(weight.is_weightless())
		{
			self.weightless.push(record)
		}
		else
		{
			let weight = weight.into_non_zero_u16();
			self.weighted.push((weight, record));
			self.current_sum_of_all_weighted += (weight.get() as u64);
		}
	}
	
	#[inline(always)]
	fn append(&mut self, append: &Self)
	{
		if self.is_empty()
		{
			*self = Clone::clone(append);
		}
		else
		{
			self.current_sum_of_all_weighted += append.current_sum_of_all_weighted;
			self.weightless.extend_from_slice(&append.weightless[..]);
			self.weighted.extend_from_slice(&append.weighted[..]);
		}
	}
	
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.weighted.is_empty() && self.weightless.is_empty()
	}
	
	#[inline(always)]
	fn record_count(&self) -> usize
	{
		self.weightless.len() + self.weighted.len()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn get_next_weight(&self, index: usize) -> u64
	{
		let (weight, _record) = unsafe { self.weighted.get_unchecked(index) };
		(*weight).get() as u64
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn random_running_sum_of_weight(&self) -> u64
	{
		const inclusive_minimum: u64 = 0;
		
		let inclusive_maximum = self.current_sum_of_all_weighted;
		let random_value = fast_slightly_insecure_random_u64().unwrap();
		
		// Technically, not biased, if `InclusiveMaximum - InclusiveMinimum + 1` is a power of 2.
		// See <https://ericlippert.com/2013/12/16/how-much-bias-is-introduced-by-the-remainder-technique/>
		let biased_uniform_distributed = inclusive_minimum + (random_value % (inclusive_maximum - inclusive_minimum + 1));
		
		biased_uniform_distributed
	}
}

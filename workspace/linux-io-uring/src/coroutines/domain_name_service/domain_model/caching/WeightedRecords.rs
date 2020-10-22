// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Already sorted, such that weight 0 occurs first.
struct WeightedRecords<Record: Sized + Clone>
{
	current_sum_of_all_weights: u64,
	sorted_records: Vec<(Weight, Record)>,
}

impl<Record: Sized + Clone> Iterator for WeightedRecords<Record>
{
	type Item = Record;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let length = match self.sorted_records.len()
		{
			0 => return None,
			
			1 => return Some(self.sorted_records.remove(0).1),
			
			length @ _ => length
		};
		
		let random_running_sum_of_weight = self.random_running_sum_of_weight();
		
		let mut current_sum_of_all_weights = 0;
		for index in 0 .. length
		{
			{
				let (weight, record) = unsafe { self.sorted_records.get_unchecked(index) };
				weight.add_to(&mut current_sum_of_all_weights);
			}
			
			if current_sum_of_all_weights >= random_running_sum_of_weight
			{
				let (weight, record) = self.sorted_records.remove(index);
				weight.subtract_from(&mut self.current_sum_of_all_weights);
				return Some(record)
			}
		}
		unreachable!();
	}
}

impl<Record: Sized + Clone> WeightedRecords<Record>
{
	#[inline(always)]
	fn new(mut unsorted_records: Vec<(Weight, Record)>) -> Self
	{
		// This only moves records that are weighted `0` relative to others.
		unsorted_records.sort_unstable_by(|left, right| left.0.zero_or_not_compare(&right.0));
		
		let mut current_sum_of_all_weights = 0;
		for (weight, _) in unsorted_records.iter()
		{
			weight.add_to(&mut current_sum_of_all_weights)
		}
		
		Self
		{
			current_sum_of_all_weights,
			sorted_records: unsorted_records,
		}
	}
	
	#[inline(always)]
	fn append(&mut self, unsorted_records: &Vec<(Weight, Record)>)
	{
		for (weight, record) in unsorted_records.iter()
		{
			weight.add_to(&mut self.current_sum_of_all_weights);
			
			let record = record.clone();
			if weight == Weight::Unassigned
			{
				self.sorted_records.insert(0, record)
			}
			else
			{
				self.sorted_records.push(record)
			}
		}
	}
	
	#[inline(always)]
	fn random_running_sum_of_weight(&self) -> u64
	{
		const inclusive_minimum: u64 = 0;
		
		let inclusive_maximum = self.current_sum_of_all_weights;
		let random_value = fast_slightly_insecure_random_u64().unwrap();
		
		// Technically, not biased, if `InclusiveMaximum - InclusiveMinimum + 1` is a power of 2.
		// See <https://ericlippert.com/2013/12/16/how-much-bias-is-introduced-by-the-remainder-technique/>
		let biased_uniform_distributed = inclusive_minimum + (random_value % (inclusive_maximum - inclusive_minimum + 1));
		
		biased_uniform_distributed
	}
}

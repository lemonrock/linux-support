// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AllPermutationsOfASet<'a, Element: 'a + Clone, V: 'a + Clone>
{
	inputs: &'a [Input<Element, V>],
	
	all_possible_combinations_for_length: Vec<Combination<Element, V>>,
	
	all_possible_permutations_for_length_at_combination: Vec<Permutations<Element>>,
}

impl<'a, Element: 'a + Clone, V: 'a + Clone> AllPermutationsOfASet<'a, Element, V>
{
	fn from_hash_map<V>(hash_map: &HashMap<Element, V>) -> (Vec<Combination<Element, V>>, Vec<Permutations<Element>>)
	{
		let inputs = Self::into_vec_from_hash_map(hash_map);
		Self::new(&inputs[..]).all()
	}
	
	fn into_vec_from_hash_map<V>(hash_map: &HashMap<Element, V>) -> Vec<Input<Element, V>>
	{
		let length = hash_map.len();
		let mut vec = Vec::with_capacity(length);
		for (element, value) in hash_map
		{
			let element = element.clone();
			let value = value.clone();
			vec.push((element, value));
		}
		vec
	}
	
	fn new(inputs: &'a [Input<Element, V>]) -> Self
	{
		let capacity = 2usize.pow(inputs.len() as u32);
		
		Self
		{
			inputs,
			
			all_possible_combinations_for_length: Vec::with_capacity(capacity),
			
			all_possible_permutations_for_length_at_combination: Vec::with_capacity(capacity),
		}
	}
	
	fn all(mut self) -> (Vec<Combination<Element, V>>, Vec<Permutations<Element>>)
	{
		let n = self.inputs.len();
		
		for index in 0 ..= n
		{
			if index == n
			{
				self.combinations_size_of_slice_len();
			}
			else
			{
				match index
				{
					0 => self.combinations_size_of_slice_0(),
	
					1 => self.combinations_size_of_slice_1(),
	
					2 => self.combinations_size_of_slice_2(),
	
					3 => self.combinations_size_of_slice_3(),
				
					_ => self.combinations_size_of_slice_k(index),
				}
			}
		}
		
		(self.all_possible_combinations_for_length, self.all_possible_permutations_for_length_at_combination)
	}
	
	#[inline(always)]
	fn factorial(n: usize) -> usize
	{
		if n <= 1
		{
			return 1
		}
		
		let mut factorial = 1;
		
		for index in 2 ..= n
		{
			factorial *= index
		}
		
		factorial
	}
	
	#[inline(always)]
	fn permute(&mut self, mut combination: Combination<Element, V>)
	{
		let k = combination.len();
		let mut result = Vec::with_capacity(Self::factorial(k));
		self.permute_using_heaps_algorithm(&mut combination, k, &mut result);
		self.all_possible_permutations_for_length_at_combination.push(result)
	}
	
	/// [Heap's Algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm).
	fn permute_using_heaps_algorithm(&mut self, values: &mut [Input<Element, V>], k: usize, result: &mut Permutations<Element>)
	{
		if k <= 1
		{
			let mut new_values = Vec::with_capacity(values.len());
			for (element, _) in values
			{
				new_values.push(element.clone());
			}
			result.push(new_values);
		}
		else
		{
			let new_k = k - 1;
			self.permute_using_heaps_algorithm(values, new_k, result);
			
			for index in 0 .. new_k
			{
				let is_even = k % 2 == 0;
				if is_even
				{
					values.swap(index, new_k);
				}
				else
				{
					values.swap(0, new_k);
				}
				
				self.permute_using_heaps_algorithm(values, new_k, result);
			}
		}
	}
	
	/// ⁿCₖ: where n == 4, k == 0 gives 1 combination.
	/// `n == inputs.len()`.
	/// * `[]`
	fn combinations_size_of_slice_0(&mut self)
	{
		let combination = vec![];
		self.record_combination(combination);
	}
	
	/// ⁿCₖ: where n == 4, k == 1 gives 4 combinations.
	/// `n == inputs.len()`.
	/// * `["A"]`
	/// * `["B"]`
	/// * `["C"]`
	/// * `["D"]`
	fn combinations_size_of_slice_1(&mut self)
	{
		for index in 0 .. self.n()
		{
			let combination = vec!
			[
				self.value(index)
			];
			self.record_combination(combination);
		}
	}
	
	/// ⁿCₖ: where n == 4, k == 2 gives 6 combinations.
	/// `n == inputs.len()`.
	/// * `["A", "B"]`
	/// * `["A", "C"]`
	/// * `["A", "D"]`
	/// * `["B", "C"]`
	/// * `["B", "D"]`
	/// * `["C", "D"]`
	fn combinations_size_of_slice_2(&mut self)
	{
		let n = self.n();
		
		for first_index in 0 .. (n - 1)
		{
			let first = self.value(first_index);
			for second_index in (first_index + 1) .. n
			{
				let second = self.value(second_index);
				let combination = vec![first, second];
				self.record_combination(combination);
			}
		}
	}
	
	/// ⁿCₖ: where n == 4, k == 3 gives 4 combinations.
	/// `n == inputs.len()`.
	/// * `["A", "B", "C"]`
	/// * `["A", "B", "D"]`
	/// * `["A", "C", "D"]`
	/// * `["B", "C", "D"]`
	fn combinations_size_of_slice_3(&mut self)
	{
		let n = self.n();
		
		for first_index in 0 .. (n - 2)
		{
			let first = self.value(first_index);
			for second_index in (first_index + 1) .. (n - 1)
			{
				let second = self.value(second_index);
				for third_index in (second_index + 1) .. n
				{
					let third = self.value(third_index);
					let combination = vec![first, second, third];
					self.record_combination(combination);
				}
			}
		}
	}
	
	fn combinations_size_of_slice_k(&mut self, k: usize)
	{
		let mut combination: Combination<Element, V> = Vec::with_capacity(k);
		unsafe { combination.set_len(k) };
		
		let n = self.n();
		let exclusive_end_index = n + 1 - k;
		
		self.recursive_loop(&mut combination, 0, exclusive_end_index, 0);
	}
	
	/// ⁿCₖ: where n == 4, k == 4 gives 1 combination.
	/// `n == inputs.len()`.
	/// * `["A", "B", "C", "D"]`
	fn combinations_size_of_slice_len(&mut self)
	{
		let combination = self.inputs.to_vec();
		self.record_combination(combination);
	}

	fn terminate_or_recursive_loop(&mut self, temporary_combination: &mut Combination<Element, V>, outer_index: usize, outer_exclusive_end_index: usize, depth: usize)
	{
		let start_index = outer_index + 1;
		let exclusive_end_index = outer_exclusive_end_index + 1;
		let terminal_loop = exclusive_end_index == self.inputs.len();
		
		if terminal_loop
		{
			for index in start_index .. exclusive_end_index
			{
				self.set_temporary_combination_value(temporary_combination, depth, index);
				self.record_combination(temporary_combination.clone());
			}
		}
		else
		{
			self.recursive_loop(temporary_combination, start_index, exclusive_end_index, depth)
		}
	}
	
	#[inline(always)]
	fn recursive_loop(&mut self, temporary_combination: &mut Combination<Element, V>, start_index: usize, exclusive_end_index: usize, depth: usize)
	{
		for index in start_index .. exclusive_end_index
		{
			self.set_temporary_combination_value(temporary_combination, depth, index);
			self.terminate_or_recursive_loop(temporary_combination, index, exclusive_end_index, depth + 1)
		}
	}
	
	#[inline(always)]
	fn set_temporary_combination_value(&self, combination: &mut Combination<Element, V>, depth: usize, index: usize)
	{
		unsafe { * combination.get_unchecked_mut(depth) = self.value(index) }
	}
	
	#[inline(always)]
	fn record_combination(&mut self, combination: Combination<Element, V>)
	{
		self.all_possible_combinations_for_length.push(combination.clone());
		self.permute(combination)
	}
	
	#[inline(always)]
	fn n(&self) -> usize
	{
		self.inputs.len()
	}
	
	#[inline(always)]
	fn value(&self, index: usize) -> Input<Element, V>
	{
		unsafe { self.inputs.get_unchecked(index).clone() }
	}
}

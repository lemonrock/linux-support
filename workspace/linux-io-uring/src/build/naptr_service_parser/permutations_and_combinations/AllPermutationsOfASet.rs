// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct AllPermutationsOfASet<'a, Element: 'a, V: 'a>
{
	inputs: &'a IndexMap<Element, V>,
	
	all_possible_combinations_and_permutations_for_combination_for_length: Vec<(Combination<'a, Element, V>, Permutations<'a, Element, V>)>,
}

impl<'a, Element: 'a, V: 'a> AllPermutationsOfASet<'a, Element, V>
{
	pub(super) fn from_index_map(inputs: &'a IndexMap<Element, V>, include_empty_combination: bool) -> Vec<(Combination<'a, Element, V>, Permutations<'a, Element, V>)>
	{
		Self::new(inputs).all(include_empty_combination)
	}
	
	fn new(inputs: &'a IndexMap<Element, V>) -> Self
	{
		eprintln!("XXX:{}", inputs.len() as u32);
		let capacity = 2usize.pow(inputs.len() as u32);
		
		Self
		{
			inputs,
			
			all_possible_combinations_and_permutations_for_combination_for_length: Vec::with_capacity(capacity),
		}
	}
	
	fn all(mut self, include_empty_combination: bool) -> Vec<(Combination<'a, Element, V>, Permutations<'a, Element, V>)>
	{
		let n = self.n();
		
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
					0 => if include_empty_combination
					{
						self.combinations_size_of_slice_0()
					},
	
					1 => self.combinations_size_of_slice_1(),
	
					2 => self.combinations_size_of_slice_2(),
	
					3 => self.combinations_size_of_slice_3(),
				
					_ => self.combinations_size_of_slice_k(index),
				}
			}
		}
		
		self.all_possible_combinations_and_permutations_for_combination_for_length
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
	fn record_combination(&mut self, combination: Combination<'a, Element, V>)
	{
		let permutations_for_combination = self.permute(&combination);
		self.all_possible_combinations_and_permutations_for_combination_for_length.push((combination, permutations_for_combination))
	}
	
	#[inline(always)]
	fn permute(&mut self, combination: &Combination<'a, Element, V>) -> Vec<Permutation<'a, Element, V>>
	{
		let mut current_permutation_for_combination = combination.clone();
		let k = current_permutation_for_combination.len();
		let mut permutations_for_combination = Vec::with_capacity(Self::factorial(k));
		Self::permute_using_heaps_algorithm(&mut current_permutation_for_combination, k, &mut permutations_for_combination);
		permutations_for_combination
	}
	
	/// [Heap's Algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm).
	fn permute_using_heaps_algorithm(current_permutation_for_combination: &mut [Input<'a, Element, V>], k: usize, permutations_for_combination: &mut Permutations<'a, Element, V>)
	{
		if k <= 1
		{
			let mut permutation = Vec::with_capacity(current_permutation_for_combination.len());
			for element_and_value in current_permutation_for_combination
			{
				permutation.push(*element_and_value);
			}
			permutations_for_combination.push(permutation);
		}
		else
		{
			let new_k = k - 1;
			Self::permute_using_heaps_algorithm(current_permutation_for_combination, new_k, permutations_for_combination);
			
			for index in 0 .. new_k
			{
				let is_even = k % 2 == 0;
				if is_even
				{
					current_permutation_for_combination.swap(index, new_k);
				}
				else
				{
					current_permutation_for_combination.swap(0, new_k);
				}
				
				Self::permute_using_heaps_algorithm(current_permutation_for_combination, new_k, permutations_for_combination);
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
		let mut combination: Combination<'a, Element, V> = Vec::with_capacity(k);
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
		let mut combination = Vec::with_capacity(self.n());
		for index in 0 .. self.n()
		{
			combination.push(self.value(index));
		}
		
		self.record_combination(combination);
	}

	fn terminate_or_recursive_loop(&mut self, temporary_combination: &mut Combination<'a, Element, V>, outer_index: usize, outer_exclusive_end_index: usize, depth: usize)
	{
		let start_index = outer_index + 1;
		let exclusive_end_index = outer_exclusive_end_index + 1;
		let terminal_loop = exclusive_end_index == self.n();
		
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
	fn recursive_loop(&mut self, temporary_combination: &mut Combination<'a, Element, V>, start_index: usize, exclusive_end_index: usize, depth: usize)
	{
		for index in start_index .. exclusive_end_index
		{
			self.set_temporary_combination_value(temporary_combination, depth, index);
			self.terminate_or_recursive_loop(temporary_combination, index, exclusive_end_index, depth + 1)
		}
	}
	
	#[inline(always)]
	fn set_temporary_combination_value(&self, combination: &mut Combination<'a, Element, V>, depth: usize, index: usize)
	{
		*(combination.get_mut(depth).unwrap()) = self.value(index)
	}
	
	#[inline(always)]
	fn n(&self) -> usize
	{
		self.inputs.len()
	}
	
	#[inline(always)]
	fn value(&self, index: usize) -> Input<'a, Element, V>
	{
		self.inputs.get_index(index).unwrap()
	}
}

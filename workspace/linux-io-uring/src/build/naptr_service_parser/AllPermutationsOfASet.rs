// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AllPermutationsOfASet<'a, Element: 'a + Copy>
{
	original_set: &'a [Element],
	
	all_possible_permutations_for_length: Vec<Vec<Element>>,
}

impl<'a, Element: 'a + Copy> AllPermutationsOfASet<'a, Element>
{
	#[inline(always)]
	fn permute(&mut self, mut values: Vec<Element>)
	{
		let k = values.len();
		self.permute_using_heaps_algorithm(&mut values, k);
	}
	
	fn permute_using_heaps_algorithm(&mut self, values: &mut [Element], k: usize)
	{
		if k <= 1
		{
			self.all_possible_permutations_for_length.push(values.to_vec());
		}
		else
		{
			let new_k = k - 1;
			self.permute_using_heaps_algorithm(values, new_k);
			
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
				
				self.permute_using_heaps_algorithm(values, new_k);
			}
		}
	}
	
	fn new(original_set: &'a [Element]) -> Self
	{
		let n = original_set.len();
		
		Self
		{
			original_set,
			
			all_possible_permutations_for_length: Vec::with_capacity(match n
			{
				0 => 1,
				1 => 2,
				2 => 5,
				3 => 16,
				4 => 65,
				5 => 326,
				other @ _ => 4usize.pow(other as u32)
			})
		}
	}
	
	fn all(mut self) -> Vec<Vec<Element>>
	{
		let n = self.original_set.len();
		
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
		
		self.all_possible_permutations_for_length
	}
	
	/// ⁿCₖ: where n == 4, k == 0 gives 1 combination.
	/// `n == original_set.len()`.
	/// * `[]`
	fn combinations_size_of_slice_0(&mut self)
	{
		self.all_possible_permutations_for_length.push(vec![])
	}
	
	/// ⁿCₖ: where n == 4, k == 1 gives 4 combinations.
	/// `n == original_set.len()`.
	/// * `["A"]`
	/// * `["B"]`
	/// * `["C"]`
	/// * `["D"]`
	fn combinations_size_of_slice_1(&mut self)
	{
		for index in 0 .. self.n()
		{
			let unordered_combination_of_elements = vec!
			[
				self.value(index)
			];
			self.all_possible_permutations_for_length.push(unordered_combination_of_elements);
		}
	}
	
	/// ⁿCₖ: where n == 4, k == 2 gives 6 combinations.
	/// `n == original_set.len()`.
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
				self.permute(vec![first, second])
			}
		}
	}
	
	/// ⁿCₖ: where n == 4, k == 3 gives 4 combinations.
	/// `n == original_set.len()`.
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
					self.permute(vec![first, second, third])
				}
			}
		}
	}
	
	fn combinations_size_of_slice_k(&mut self, k: usize)
	{
		let mut tuple: Vec<Element> = Vec::with_capacity(k);
		unsafe { tuple.set_len(k) };
		
		let n = self.n();
		let exclusive_end_index = n + 1 - k;
		
		self.recursive_loop(&mut tuple, 0, exclusive_end_index, 0);
	}
	
	/// ⁿCₖ: where n == 4, k == 4 gives 1 combination.
	/// `n == original_set.len()`.
	/// * `["A", "B", "C", "D"]`
	fn combinations_size_of_slice_len(&mut self)
	{
		self.permute(self.original_set.to_vec())
	}

	fn terminate_or_recursive_loop(&mut self, tuple: &mut Vec<Element>, outer_index: usize, outer_exclusive_end_index: usize, depth: usize)
	{
		let start_index = outer_index + 1;
		let exclusive_end_index = outer_exclusive_end_index + 1;
		let terminal_loop = exclusive_end_index == self.original_set.len();
		
		if terminal_loop
		{
			for index in start_index .. exclusive_end_index
			{
				self.set_value(tuple, depth, index);
				self.permute(tuple.clone())
			}
		}
		else
		{
			self.recursive_loop(tuple, start_index, exclusive_end_index, depth)
		}
	}
	
	#[inline(always)]
	fn recursive_loop(&mut self, tuple: &mut Vec<Element>, start_index: usize, exclusive_end_index: usize, depth: usize)
	{
		for index in start_index .. exclusive_end_index
		{
			self.set_value(tuple, depth, index);
			self.terminate_or_recursive_loop(tuple, index, exclusive_end_index, depth + 1)
		}
	}
	
	#[inline(always)]
	fn set_value(&self, tuple: &mut Vec<Element>, depth: usize, index: usize)
	{
		unsafe { * tuple.get_unchecked_mut(depth) = self.value(index) }
	}
	
	#[inline(always)]
	fn n(&self) -> usize
	{
		self.original_set.len()
	}
	
	#[inline(always)]
	fn value(&self, index: usize) -> Element
	{
		unsafe { * self.original_set.get_unchecked(index) }
	}
}

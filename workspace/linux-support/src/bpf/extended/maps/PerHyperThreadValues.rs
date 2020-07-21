// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a variably-sized two-dimensional array.
pub struct PerHyperThreadValues<V: Copy>
{
	number_of_validly_initialized_values: usize,
	number_of_possible_hyper_threads: NumberOfPossibleHyperThreads,
	data: Vec<u8>,
	marker: PhantomData<V>,
}

impl<V: Copy> PerHyperThreadValues<V>
{
	/// Number of keys that are validly initialized.
	#[inline(always)]
	pub fn number_of_validly_initialized_values(&self) -> usize
	{
		self.number_of_validly_initialized_values
	}
	
	#[inline(always)]
	pub(crate) fn set_number_of_validly_initialized_values(&mut self, number_of_validly_initialized_values: usize)
	{
		self.number_of_validly_initialized_values = number_of_validly_initialized_values
	}
	
	#[inline(always)]
	pub(crate) fn data(&mut self) -> &[u8]
	{
		&self.data[..]
	}
	
	#[inline(always)]
	pub(crate) fn mutable_data(&mut self) -> &mut [u8]
	{
		&mut self.data[..]
	}
	
	/// Values at row (`row_index` is the index of the key in a slice of keys, `&[K]`).
	#[inline(always)]
	pub fn values_at_row(&self, row_index: usize) -> &[PerHyperThreadValue<V>]
	{
		unsafe
		{
			let pointer = self.data.as_ptr().add(self.offset(row_index)) as *const PerHyperThreadValue<V>;
			from_raw_parts(pointer, self.values_length())
		}
	}
	
	/// Values at row (`row_index` is the index of the key in a slice of keys, `&[K]`).
	///
	/// Mutable.
	#[inline(always)]
	pub fn values_at_row_mut(&mut self, row_index: usize) -> &mut [PerHyperThreadValue<V>]
	{
		unsafe
		{
			let pointer = self.data.as_mut_ptr().add(self.offset(row_index)) as *mut PerHyperThreadValue<V>;
			from_raw_parts_mut(pointer, self.values_length())
		}
	}
	
	#[inline(always)]
	fn new(number_of_keys: usize, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Self
	{
		let bytes = number_of_keys * number_of_possible_hyper_threads.row_size_in_bytes::<V>();
		let mut data = Vec::with_capacity(bytes);
		unsafe { data.set_len(bytes) };
		Self
		{
			number_of_validly_initialized_values: 0,
			number_of_possible_hyper_threads,
			data,
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	fn values_length(&self) -> usize
	{
		self.number_of_possible_hyper_threads.length()
	}
	
	#[inline(always)]
	fn offset(&self, row_index: usize) -> usize
	{
		assert!(row_index < self.number_of_validly_initialized_values);
		
		row_index * self.row_size_in_bytes()
	}
	
	#[inline(always)]
	fn row_size_in_bytes(&self) -> usize
	{
		self.number_of_possible_hyper_threads.row_size_in_bytes::<V>()
	}
	
	#[inline(always)]
	fn size_of_per_hyper_thread_value() -> usize
	{
		size_of::<PerHyperThreadValue<V>>()
	}
}

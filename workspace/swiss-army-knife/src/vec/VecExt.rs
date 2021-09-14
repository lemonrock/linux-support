// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `Vec<T>` extension trait.
pub trait VecExt<T>
{
	/// Creates a new instance with the given capacity or fails.
	fn try_with_capacity(capacity: usize) -> Result<Vec<T>, AllocError>;
	
	/// Done this way instead of repeated `push()` or specialized `extend()` to minimize `if` checks for each `push()` and give LLVM's loop unrolling a chance to optimize.
	fn new_populated_by_copy(capacity: usize, copy: T) -> Result<Vec<T>, AllocError> where T: Copy;
	
	/// Done this way instead of repeated `push()` or specialized `extend()` to minimize `if` checks for each `push()` and give LLVM's loop unrolling a chance to optimize.
	fn new_populated_fallibly<Populator: FnOnce(usize) -> Result<T, AllocError> + Copy>(capacity: usize, populator: Populator) -> Result<Vec<T>, AllocError>;
	
	/// Push without checking capacity.
	fn push_unchecked(&mut self, value: T);
	
	#[doc(hidden)]
	fn set_length(&mut self, length: usize);
}

impl<T> VecExt<T> for Vec<T>
{
	#[inline(always)]
	fn try_with_capacity(capacity: usize) -> Result<Vec<T>, AllocError>
	{
		let mut new_vec = Vec::new();
		if new_vec.try_reserve_exact(capacity).is_err()
		{
			return Err(AllocError)
		}
		Ok(new_vec)
	}
	
	#[inline(always)]
	fn new_populated_by_copy(capacity: usize, copy: T) -> Result<Vec<T>, AllocError>
	where T: Copy
	{
		let mut vec = Self::try_with_capacity(capacity)?;
		let pointer = vec.as_mut_ptr();
		for index in 0 .. capacity
		{
			unsafe { pointer.add(index).write(copy) };
		}
		vec.set_length(capacity);
		Ok(vec)
	}
	
	#[inline(always)]
	fn new_populated_fallibly<Populator: FnOnce(usize) -> Result<T, AllocError> + Copy>(capacity: usize, populator: Populator) -> Result<Vec<T>, AllocError>
	{
		let mut vec = Self::try_with_capacity(capacity)?;
		let pointer = vec.as_mut_ptr();
		for index in 0 .. capacity
		{
			let element = match populator(index)
			{
				Err(AllocError) =>
				{
					vec.set_length(index);
					return Err(AllocError)
				}
				
				Ok(child_or_auxillary) => child_or_auxillary,
			};
			
			unsafe { pointer.add(index).write(element) };
		}
		vec.set_length(capacity);
		Ok(vec)
	}
	
	#[inline(always)]
	fn push_unchecked(&mut self, value: T)
	{
		let length = self.len();
		unsafe
		{
			let end = self.as_mut_ptr().add(length);
			ptr::write(end, value);
			self.set_len(length + 1);
		}
	}
	
	#[inline(always)]
	fn set_length(&mut self, length: usize)
	{
		debug_assert!(length <= self.capacity());
		unsafe { self.set_len(length) };
	}
}

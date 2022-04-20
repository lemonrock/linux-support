// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Heap<T>(*mut T, usize);

impl<T> const Default for Heap<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(null_mut(), 0)
	}
}

impl<T> Heap<T>
{
	#[inline(always)]
	fn from_vec(mut vec: Vec<T>) -> Self
	{
		let pointer = vec.as_mut_ptr();
		let length = vec.len();
		
		let _forget = ManuallyDrop::new(vec);
		Self(pointer, length)
	}
	
	#[inline(always)]
	fn from_pointer_and_length(pointer: NonNull<T>, length: usize) -> Self
	{
		Self(pointer.as_ptr(), length)
	}
	
	#[inline(always)]
	fn set_pointer(&mut self, pointer: NonNull<T>)
	{
		self.0 = pointer.as_ptr()
	}
	
	#[inline(always)]
	const fn slice<'a>(&self) -> &'a [T]
	{
		let data = self.non_null_pointer().as_ptr() as *const T;
		let length = self.length();
		unsafe { from_raw_parts(data, length) }
	}
	
	#[inline(always)]
	const fn slice_mut<'a>(&self) -> &'a mut [T]
	{
		let data = self.non_null_pointer().as_ptr();
		let length = self.length();
		unsafe { from_raw_parts_mut(data, length) }
	}
	
	#[inline(always)]
	const fn pointer_and_length(&self) -> (NonNull<T>, usize)
	{
		(self.non_null_pointer(), self.length())
	}
	
	#[inline(always)]
	const fn non_null_pointer(&self) -> NonNull<T>
	{
		if cfg!(debug_assertions)
		{
			if self.0.is_null()
			{
				panic!("Should never be null")
			}
		}
		new_non_null(self.0)
	}
	
	#[inline(always)]
	const fn length(&self) -> usize
	{
		self.1
	}
	
	#[inline(always)]
	fn length_ref_mut(&mut self) -> &mut usize
	{
		&mut self.1
	}
}

// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Heap<T>(*mut [T]);

impl<T> Copy for Heap<T>
{
}

impl<T> Clone for Heap<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0)
	}
}

impl<T> const Default for Heap<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(unsafe { from_raw_parts_mut(null_mut(), 0) })
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
		Self(unsafe { from_raw_parts_mut(pointer, length) })
	}
	
	#[inline(always)]
	fn from_pointer_and_length(pointer: NonNull<T>, length: usize) -> Self
	{
		Self(NonNull::slice_from_raw_parts(pointer, length).as_ptr())
	}
	
	#[inline(always)]
	fn set_pointer(&mut self, pointer: NonNull<T>)
	{
		let (was_pointer, _) = self.inner_ref_mut();
		
		*was_pointer = pointer;
	}
	
	#[inline(always)]
	const fn slice<'a>(&self) -> &'a [T]
	{
		unsafe { & * self.0 }
	}
	
	#[inline(always)]
	const fn slice_mut<'a>(&self) -> &'a mut [T]
	{
		unsafe { &mut * self.0 }
	}
	
	#[inline(always)]
	fn into_vec(&self, capacity: usize) -> Vec<T>
	{
		let (pointer, length) = self.pointer_and_length();
		unsafe { Vec::from_raw_parts(pointer, length, capacity) }
	}
	
	#[inline(always)]
	const fn pointer_and_length(&self) -> (*mut T, usize)
	{
		(self.pointer(), self.length())
	}
	
	#[inline(always)]
	const fn non_null_pointer(&self) -> NonNull<T>
	{
		new_non_null(self.pointer())
	}
	
	#[inline(always)]
	const fn pointer(&self) -> *mut T
	{
		self.0.as_mut_ptr()
	}
	
	#[inline(always)]
	fn length_ref_mut(&mut self) -> &mut usize
	{
		let (_, length) = self.inner_ref_mut();
		length
	}
	
	#[inline(always)]
	const fn length(&self) -> usize
	{
		self.0.len()
	}
	
	#[inline(always)]
	const fn inner_ref_mut(&mut self) -> &mut (NonNull<T>, usize)
	{
		let hack = (&mut self.0).cast::<(NonNull<T>, usize)>();
		unsafe { &mut * hack }
	}
}

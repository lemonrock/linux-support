// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


union StackWithoutLengthOrHeap<T, const N: usize>
{
	stack_without_length: StackWithoutLength<T, N>,
	
	heap: Heap<T>,
}

impl<T, const N: usize> Clone for StackWithoutLengthOrHeap<T, N>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		unsafe { transmute_copy(self) }
	}
}

impl<T, const N: usize> const Default for StackWithoutLengthOrHeap<T, N>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			stack_without_length: StackWithoutLength::default(),
		}
	}
}

impl<T, const N: usize> StackWithoutLengthOrHeap<T, N>
{
	#[inline(always)]
	fn set_heap(&mut self, heap: Heap<T>)
	{
		unsafe { self.heap = heap }
	}
	
	#[inline(always)]
	const fn take_array(&mut self) -> [T; N]
	{
		let stack_without_length = self.stack_without_length_mut();
		let inner = &stack_without_length.0;
		
		// Replacement for `ManuallyDrop::take()`, which isn't constant.
		let maybe_uninit_array =
		{
			if size_of::<ManuallyDrop<MaybeUninit<[T; N]>>>() != size_of::<MaybeUninit<[T; N]>>()
			{
				panic!("ManuallyDrop size has changed from that assumed")
			}
			// Pointer to the `ManuallyDrop::value` field.
			let pointer_to_inner_value = (inner as *const ManuallyDrop<MaybeUninit<[T; N]>>).cast::<MaybeUninit<[T; N]>>();
			unsafe { read(pointer_to_inner_value) }
		};
		
		unsafe { maybe_uninit_array.assume_init() }
	}
	
	#[inline(always)]
	const fn stack_without_length(&self) -> &StackWithoutLength<T, N>
	{
		unsafe { &self.stack_without_length }
	}
	
	#[inline(always)]
	const fn stack_without_length_mut(&mut self) -> &mut StackWithoutLength<T, N>
	{
		unsafe { &mut self.stack_without_length }
	}
	
	#[inline(always)]
	const fn heap(&self) -> &Heap<T>
	{
		unsafe { &self.heap }
	}
	
	#[inline(always)]
	const fn heap_mut(&mut self) -> &mut Heap<T>
	{
		unsafe { &mut self.heap }
	}
}

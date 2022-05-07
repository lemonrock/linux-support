// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct StackWithoutLength<T, const N: usize>(ManuallyDrop<MaybeUninit<[T; N]>>);

impl<T, const N: usize> const Default for StackWithoutLength<T, N>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(MaybeUninit::uninit())
	}
}

impl<T, const N: usize> const From<MaybeUninit<[T; N]>> for StackWithoutLength<T, N>
{
	#[inline(always)]
	fn from(stack_without_length: MaybeUninit<[T; N]>) -> Self
	{
		Self(ManuallyDrop::new(stack_without_length))
	}
}

impl<T, const N: usize> StackWithoutLength<T, N>
{
	#[inline(always)]
	const fn slice(&self, length_of_stack: usize) -> &[T]
	{
		let pointer = self.pointer();
		unsafe { from_raw_parts(pointer, length_of_stack) }
	}
	
	#[inline(always)]
	const fn slice_mut(&mut self, length_of_stack: usize) -> &mut [T]
	{
		let pointer = self.pointer_mut();
		unsafe { from_raw_parts_mut(pointer, length_of_stack) }
	}
	
	#[inline(always)]
	const fn non_null_pointer(&mut self) -> NonNull<T>
	{
		new_non_null(self.pointer_mut())
	}
	
	#[inline(always)]
	const fn pointer(&self) -> *const T
	{
		self.0.as_ptr().cast::<T>()
	}
	
	#[inline(always)]
	const fn pointer_mut(&mut self) -> *mut T
	{
		self.0.as_mut_ptr().cast::<T>()
	}
}

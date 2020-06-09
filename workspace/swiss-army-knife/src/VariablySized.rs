// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wrapper for variably sized data.
#[derive(Debug, Eq, PartialEq)]
pub struct VariablySized<T>(NonNull<T>, Layout);

impl<T> Drop for VariablySized<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pointer = self.0.as_ptr() as *mut u8;
		unsafe { dealloc(pointer, self.1) }
	}
}

impl<T> Deref for VariablySized<T>
{
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { & * self.0.as_ptr() }
	}
}

impl<T> DerefMut for VariablySized<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut * self.0.as_ptr() }
	}
}

impl<T> VariablySized<T>
{
	/// Allocate zeroed.
	#[inline(always)]
	pub fn allocate_zeroed(size: usize) -> Self
	{
		let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
		let pointer = unsafe { alloc_zeroed(layout.clone()) };
		Self(unsafe { NonNull::new_unchecked(pointer as *mut T) }, layout)
	}

	/// Zero (uninitialize).
	#[inline(always)]
	pub fn zero(&mut self)
	{
		let pointer = self.0.as_ptr();
		unsafe { write_bytes(pointer, 0x00, self.1.size()) }
	}
}

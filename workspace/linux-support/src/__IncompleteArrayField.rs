// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default)]
pub(crate) struct __IncompleteArrayField<T>(PhantomData<T>, [T; 0]);

impl<T> __IncompleteArrayField<T>
{
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		Self(PhantomData, [])
	}
	
	#[inline(always)]
	pub(crate) fn as_ptr(&self) -> *const T
	{
		self as *const _ as *const T
	}
	
	#[inline(always)]
	pub(crate) fn as_mut_ptr(&mut self) -> *mut T
	{
		self as *mut _ as *mut T
	}
	
	#[inline(always)]
	pub(crate) unsafe fn as_slice(&self, len: usize) -> &[T]
	{
		from_raw_parts(self.as_ptr(), len)
	}
	
	#[inline(always)]
	pub(crate) unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T]
	{
		from_raw_parts_mut(self.as_mut_ptr(), len)
	}
}

impl<T> Display for __IncompleteArrayField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result
	{
		fmt.write_str("__IncompleteArrayField")
	}
}

impl<T> Debug for __IncompleteArrayField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result
	{
		fmt.write_str("__IncompleteArrayField")
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait U8SliceMut<'a>: U8Slice<'a>
{
	#[inline(always)]
	fn cast_mut<T>(self, offset: usize) -> &'a mut T
	{
		unsafe { &mut * (self.get_mut_::<T>(offset)) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn get_mut_<T>(self, offset: usize) -> *mut T
	{
		(unsafe { self.slice_mut_().get_unchecked_mut(offset) }).unsafe_cast_mut::<T>()
	}

	#[doc(hidden)]
	fn slice_mut_(self) -> &'a mut [u8];
}

impl<'a> U8SliceMut<'a> for &'a mut [u8]
{
	#[inline(always)]
	fn slice_mut_(self) -> &'a mut [u8]
	{
		self
	}
}

impl<'a> U8SliceMut<'a> for &'a mut [u8; 2]
{
	#[inline(always)]
	fn slice_mut_(self) -> &'a mut [u8]
	{
		&mut self[..]
	}
}

impl<'a> U8SliceMut<'a> for &'a mut [u8; 4]
{
	#[inline(always)]
	fn slice_mut_(self) -> &'a mut [u8]
	{
		&mut self[..]
	}
}

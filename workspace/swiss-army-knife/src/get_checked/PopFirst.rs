// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Gets the next byte and pops it from the front of a slice.
pub trait PopFirst<T>
{
	/// Gets the next byte and pops it from the front of a slice.
	#[inline(always)]
	fn pop_first(&mut self) -> Option<T>
	{
		self.pop_first_internal(Some, None)
	}
	
	/// Gets the next byte and pops it from the front of a slice.
	#[inline(always)]
	fn pop_first_or_error<E: error::Error>(&mut self, error: E) -> Result<T, E>
	{
		self.pop_first_internal(Ok, Err(error))
	}
	
	#[doc(hidden)]
	fn pop_first_internal<R, Ok: FnOnce(T) -> R>(&mut self, ok: Ok, error: R) -> R;
}

impl<'a, GU: GetUnchecked<T>, T: Copy> PopFirst<T> for &'a GU
{
	#[inline(always)]
	fn pop_first_internal<R, Ok: FnOnce(T) -> R>(&mut self, ok: Ok, error: R) -> R
	{
		let slice = *self;
		if slice.as_ref().is_empty()
		{
			error
		}
		else
		{
			let value = slice.get_unchecked_value_safe(0usize);
			*self = slice.get_unchecked_range_safe(1usize .. );
			ok(value)
		}
	}
}

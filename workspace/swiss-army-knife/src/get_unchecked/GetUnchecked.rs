// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
pub trait GetUnchecked<T>
{
	/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T;
	
	/// Like `self[index] = value` but unchecked unless debug assertions are configured.
	#[inline(always)]
	fn set_unchecked_mut_safe<AUI: AsUsizeIndex>(&mut self, index: AUI, value: T)
	{
		*self.get_unchecked_mut_safe(index) = value
	}
	
	/// Like `get_unchecked_mut()`, but, if debug assertions are configured, checks and panics.
	fn get_unchecked_mut_safe<AUI: AsUsizeIndex>(&mut self, index: AUI) -> &mut T;
	
	/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
	#[inline(always)]
	fn get_unchecked_value_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> T
	where T: Copy
	{
		* self.get_unchecked_safe(index)
	}
}

impl<T> GetUnchecked<T> for [T]
{
	#[inline(always)]
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T
	{
		let index = index.as_usize();
		debug_assert!(index < self.len());
		unsafe { self.get_unchecked(index) }
	}
	
	#[inline(always)]
	fn get_unchecked_mut_safe<AUI: AsUsizeIndex>(&mut self, index: AUI) -> &mut T
	{
		let index = index.as_usize();
		debug_assert!(index < self.len());
		unsafe { self.get_unchecked_mut(index) }
	}
}

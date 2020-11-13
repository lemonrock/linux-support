// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
pub trait GetUnchecked<T>
{
	/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T;
}

impl<'a, T: 'a> GetUnchecked<T> for &'a [T]
{
	#[inline(always)]
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T
	{
		let index = index.as_usize();
		debug_assert!(index < self.len());
		unsafe { self.get_unchecked(index) }
	}
}

impl<'a, T: 'a> GetUnchecked<T> for &'a mut [T]
{
	#[inline(always)]
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T
	{
		let index = index.as_usize();
		debug_assert!(index < self.len());
		unsafe { self.get_unchecked(index) }
	}
}

impl<T> GetUnchecked<T> for Vec<T>
{
	#[inline(always)]
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T
	{
		let index = index.as_usize();
		debug_assert!(index < self.len());
		unsafe { self.get_unchecked(index) }
	}
}

impl<T> GetUnchecked<T> for Box<[T]>
{
	#[inline(always)]
	fn get_unchecked_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> &T
	{
		let index = index.as_usize();
		debug_assert!(index < self.len());
		unsafe { self.get_unchecked(index) }
	}
}

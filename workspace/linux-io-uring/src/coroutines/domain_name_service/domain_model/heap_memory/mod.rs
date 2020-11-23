// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use context_allocator::allocators::Allocator;
use std::mem::align_of;
use std::borrow::BorrowMut;
use std::alloc::Layout;


pub struct OwnedBox<T, A: Allocator>
{
	pointer_to_t: NonNull<T>,
	allocator: Rc<A>,
}

impl<T, A: Allocator> Drop for OwnedBox<T, A>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.allocator.AllocRef_dealloc(self.pointer_to_t.cast(), Self::layout()) };
	}
}

impl<T, A: Allocator> OwnedBox<T, A>
{
	#[inline(always)]
	fn layout() -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(size_of::<T>(), align_of::<T>()) }
	}
	
	/// New instance.
	#[inline(always)]
	pub fn new(t: T, allocator: &Rc<A>) -> Result<Self, AllocError>
	{
		let layout = Self::layout();
		let pointer_to_x = allocator.AllocRef_alloc(Self::layout())?.as_non_null_ptr().cast();
		
		unsafe { copy_nonoverlapping(&t, pointer_to_x.as_ptr(), layout.size()) }
		
		let allocator = allocator.clone();
		Ok
		(
			Self
			{
				pointer_to_t: pointer_to_x,
				allocator,
			}
		)
	}
}

impl<T, A: Allocator> Deref for OwnedBox<T, A>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T
	{
		unsafe { &* self.pointer_to_t.as_ptr() }
	}
}

impl<T, A: Allocator> DerefMut for OwnedBox<T, A>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T
	{
		unsafe { &mut * self.pointer_to_t.as_ptr() }
	}
}

impl<T, A: Allocator> Borrow<T> for OwnedBox<T, A>
{
    fn borrow(&self) -> &T
	{
		self.deref()
    }
}

impl<T, A: Allocator> BorrowMut<T> for OwnedBox<T, A>
{
	#[inline(always)]
    fn borrow_mut(&mut self) -> &mut T
	{
		self.deref_mut()
    }
}

impl<T, A: Allocator> AsRef<T> for OwnedBox<T, A>
{
	#[inline(always)]
    fn as_ref(&self) -> &T
	{
        self.deref()
    }
}

impl<T, A: Allocator> AsMut<T> for OwnedBox<T, A>
{
	#[inline(always)]
    fn as_mut(&mut self) -> &mut T
	{
		self.deref_mut()
    }
}

impl<T: Clone, A: Allocator> Clone for OwnedBox<T, A>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new((**self).clone(), &self.allocator).unwrap()
	}
	
	#[inline(always)]
	fn clone_from(&mut self, source: &Self)
	{
		(**self).clone_from(&(**source));
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for bounded range slices.
pub trait AsUsizeRange<T>
{
	#[cfg(debug_assertions)]
	#[doc(hidden)]
	fn get_checked_range_ref(self, slice: &[T]) -> Option<&[T]>;
	
	#[cfg(debug_assertions)]
	#[doc(hidden)]
	fn get_checked_range_mut(self, slice: &mut [T]) -> Option<&mut [T]>;
	
	#[doc(hidden)]
	unsafe fn get_unchecked_range_ref(self, slice: *const [T]) -> *const [T];
	
	#[doc(hidden)]
	unsafe fn get_unchecked_range_mut(self, slice: *mut [T]) -> *mut [T];
}

impl<T, AUI: AsUsizeIndex> AsUsizeRange<T> for Range<AUI>
{
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_ref(self, slice: &[T]) -> Option<&[T]>
	{
		let start = self.start.as_usize();
		let end = self.end.as_usize();
		(start .. end).get(slice)
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_mut(self, slice: &mut [T]) -> Option<&mut [T]>
	{
		let start = self.start.as_usize();
		let end = self.end.as_usize();
		(start .. end).get_mut(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_ref(self, slice: *const [T]) -> *const [T]
	{
		let start = self.start.as_usize();
		let end = self.end.as_usize();
		(start .. end).get_unchecked(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_mut(self, slice: *mut [T]) -> *mut [T]
	{
		let start = self.start.as_usize();
		let end = self.end.as_usize();
		(start .. end).get_unchecked_mut(slice)
	}
}

impl<T, AUI: AsUsizeIndex> AsUsizeRange<T> for RangeFrom<AUI>
{
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_ref(self, slice: &[T]) -> Option<&[T]>
	{
		(self.start.as_usize() ..).get(slice)
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_mut(self, slice: &mut [T]) -> Option<&mut [T]>
	{
		(self.start.as_usize() ..).get_mut(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_ref(self, slice: *const [T]) -> *const [T]
	{
		(self.start.as_usize() ..).get_unchecked(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_mut(self, slice: *mut [T]) -> *mut [T]
	{
		(self.start.as_usize() ..).get_unchecked_mut(slice)
	}
}

impl<T> AsUsizeRange<T> for RangeFull
{
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_ref(self, slice: &[T]) -> Option<&[T]>
	{
		Some(slice)
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_mut(self, slice: &mut [T]) -> Option<&mut [T]>
	{
		Some(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_ref(self, slice: *const [T]) -> *const [T]
	{
		slice
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_mut(self, slice: *mut [T]) -> *mut [T]
	{
		slice
	}
}

impl<T, AUI: AsUsizeIndex + Copy> AsUsizeRange<T> for RangeInclusive<AUI>
{
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_ref(self, slice: &[T]) -> Option<&[T]>
	{
		let start = self.start().as_usize();
		let end = self.end().as_usize();
		(start ..= end).get(slice)
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_mut(self, slice: &mut [T]) -> Option<&mut [T]>
	{
		let start = self.start().as_usize();
		let end = self.end().as_usize();
		(start ..= end).get_mut(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_ref(self, slice: *const [T]) -> *const [T]
	{
		let start = self.start().as_usize();
		let end = self.end().as_usize();
		(start ..= end).get_unchecked(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_mut(self, slice: *mut [T]) -> *mut [T]
	{
		let start = self.start().as_usize();
		let end = self.end().as_usize();
		(start ..= end).get_unchecked_mut(slice)
	}
}

impl<T, AUI: AsUsizeIndex> AsUsizeRange<T> for RangeToInclusive<AUI>
{
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_ref(self, slice: &[T]) -> Option<&[T]>
	{
		(..= self.end.as_usize()).get(slice)
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	fn get_checked_range_mut(self, slice: &mut [T]) -> Option<&mut [T]>
	{
		(..= self.end.as_usize()).get_mut(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_ref(self, slice: *const [T]) -> *const [T]
	{
		(..= self.end.as_usize()).get_unchecked(slice)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_range_mut(self, slice: *mut [T]) -> *mut [T]
	{
		(..= self.end.as_usize()).get_unchecked_mut(slice)
	}
}

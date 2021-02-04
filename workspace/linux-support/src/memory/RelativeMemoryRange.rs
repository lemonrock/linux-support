// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a relative range of memory.
pub trait RelativeMemoryRange
{
	/// Computes start and length, checking in debug builds that the values are page aligned.
	#[inline(always)]
	fn compute_and_debug_assert_page_aligned(&self, absolute_inclusive_start: VirtualAddress, original_length: usize) -> (VirtualAddress, usize)
	{
		let computed_start = self.compute_inclusive_absolute_start(absolute_inclusive_start);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(computed_start.into()), "computed_start `{:?}` is not page aligned", computed_start);
		
		let computed_length = self.compute_length(original_length);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(computed_length), "computed_length `{}` is not page aligned", computed_length);
		
		(computed_start, computed_length)
	}
	
	/// Computes the absolute inclusize start.
	fn compute_inclusive_absolute_start(&self, absolute_inclusive_start: VirtualAddress) -> VirtualAddress;
	
	/// Computes the length.
	fn compute_length(&self, original_length: usize) -> usize;
}

impl<T: AsPrimitive<usize> + Unsigned> RelativeMemoryRange for Range<T>
where VirtualAddress: Add<T, Output=VirtualAddress>
{
	#[inline(always)]
	fn compute_inclusive_absolute_start(&self, original_absolute_inclusive_start: VirtualAddress) -> VirtualAddress
	{
		original_absolute_inclusive_start + self.start
	}
	
	#[inline(always)]
	fn compute_length(&self, original_length: usize) -> usize
	{
		compute_length_exclusive(self.start.as_(), self.end.as_(), original_length)
	}
}

impl<T: AsPrimitive<usize> + Unsigned + Copy> RelativeMemoryRange for RangeInclusive<T>
where VirtualAddress: Add<T, Output=VirtualAddress>
{
	#[inline(always)]
	fn compute_inclusive_absolute_start(&self, original_absolute_inclusive_start: VirtualAddress) -> VirtualAddress
	{
		original_absolute_inclusive_start + *self.start()
	}
	
	#[inline(always)]
	fn compute_length(&self, original_length: usize) -> usize
	{
		compute_length_inclusive((*self.start()).as_(), (*self.end()).as_(), original_length)
	}
}

impl RelativeMemoryRange for RangeFull
{
	#[inline(always)]
	fn compute_inclusive_absolute_start(&self, original_absolute_inclusive_start: VirtualAddress) -> VirtualAddress
	{
		original_absolute_inclusive_start
	}
	
	#[inline(always)]
	fn compute_length(&self, original_length: usize) -> usize
	{
		original_length
	}
}

impl<T: AsPrimitive<usize> + Unsigned> RelativeMemoryRange for RangeFrom<T>
where VirtualAddress: Add<T, Output=VirtualAddress>
{
	#[inline(always)]
	fn compute_inclusive_absolute_start(&self, original_absolute_inclusive_start: VirtualAddress) -> VirtualAddress
	{
		original_absolute_inclusive_start + self.start
	}
	
	#[inline(always)]
	fn compute_length(&self, original_length: usize) -> usize
	{
		original_length
	}
}

impl<T: AsPrimitive<usize> + Unsigned> RelativeMemoryRange for RangeTo<T>
where VirtualAddress: Add<T, Output=VirtualAddress>
{
	#[inline(always)]
	fn compute_inclusive_absolute_start(&self, original_absolute_inclusive_start: VirtualAddress) -> VirtualAddress
	{
		original_absolute_inclusive_start
	}
	
	#[inline(always)]
	fn compute_length(&self, original_length: usize) -> usize
	{
		compute_length_exclusive(0, self.end.as_(), original_length)
	}
}

impl<T: AsPrimitive<usize> + Unsigned> RelativeMemoryRange for RangeToInclusive<T>
where VirtualAddress: Add<T, Output=VirtualAddress>
{
	#[inline(always)]
	fn compute_inclusive_absolute_start(&self, original_absolute_inclusive_start: VirtualAddress) -> VirtualAddress
	{
		original_absolute_inclusive_start
	}
	
	#[inline(always)]
	fn compute_length(&self, original_length: usize) -> usize
	{
		compute_length_inclusive(0, self.end.as_(), original_length)
	}
}

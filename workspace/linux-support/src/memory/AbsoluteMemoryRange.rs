// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents an absolute range of memory.
///
/// Note that this is implemented for `(&MappedMemory, impl RelativeMemoryRange)` and `MappedMemorySubrange` for convenience, but use of these is best avoided as they are inefficient to use repeatedly on the same data (ie they don't memoize).
pub trait AbsoluteMemoryRange: Sized
{
	/// Convenience method.
	///
	/// Result must be page-aligned.
	///
	/// If the memory range represents huge-page backed memory, result must be huge-page aligned.
	#[inline(always)]
	fn inclusive_absolute_start_and_length_u64(self) -> (u64, u64)
	{
		let (start, length) = self.inclusive_absolute_start_and_length();
		(start.into(), length as u64)
	}
	
	/// Convenience method that can be optimized for some implementations.
	///
	/// Result must be page-aligned.
	///
	/// If the memory range represents huge-page backed memory, result must be huge-page aligned.
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize);
	
	/// Start.
	///
	/// Must be page-aligned.
	///
	/// If the memory range represents huge-page backed memory, must be huge-page aligned.
	///
	/// Calling this more than once may be inefficient; prefer `inclusive_absolute_start_and_length()`.
	fn inclusive_absolute_start(self) -> VirtualAddress;
	
	/// Length.
	///
	/// Must be page-aligned.
	///
	/// If the memory range represents huge-page backed memory, must be huge-page aligned.
	///
	/// Calling this more than once may be inefficient; prefer `inclusive_absolute_start_and_length()`.
	fn length(self) -> usize;
}

impl<'a, Subrange: RelativeMemoryRange> AbsoluteMemoryRange for (&'a MappedMemory, Subrange)
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		self.0.sub_range_inner(&self.1)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.0.sub_range_inner(&self.1).0
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.0.sub_range_inner(&self.1).1
	}
}

impl<T: Sized> AbsoluteMemoryRange for NonNull<[T]>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		(
			self.inclusive_absolute_start(),
			self.length()
		)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		VirtualAddress::from(self.as_mut_ptr())
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.len()
	}
}

impl<'a, T: 'a + Sized> AbsoluteMemoryRange for &'a [T]
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		(
			self.inclusive_absolute_start(),
			self.length()
		)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		VirtualAddress::from(self.as_ptr())
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.len()
	}
}

impl<'a, T: 'a + Sized> AbsoluteMemoryRange for &'a mut [T]
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		(
			self.inclusive_absolute_start(),
			self.length()
		)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		VirtualAddress::from(self.as_ptr())
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.len()
	}
}

impl<T: Sized> AbsoluteMemoryRange for *const [T]
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		(
			self.inclusive_absolute_start(),
			self.length()
		)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		VirtualAddress::from(self)
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.len()
	}
}

impl<T: Sized> AbsoluteMemoryRange for *mut [T]
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		(
			self.inclusive_absolute_start(),
			self.length()
		)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		VirtualAddress::from(self)
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.len()
	}
}

impl<T: Into<VirtualAddress>> AbsoluteMemoryRange for Range<T>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		let Range { start, end } = self;
		let inclusive_absolute_start = start.into();
		let exclusive_end = end.into();
		let length = exclusive_end.saturating_sub(inclusive_absolute_start);
		(inclusive_absolute_start, length)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.start.into()
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		let Range { start, end } = self;
		let inclusive_absolute_start = start.into();
		let exclusive_end = end.into();
		exclusive_end.saturating_sub(inclusive_absolute_start)
	}
}

impl<'a, T: 'a + Into<VirtualAddress> + Clone> AbsoluteMemoryRange for &'a Range<T>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		let Range { start, end } = self.clone();
		let inclusive_absolute_start = start.into();
		let exclusive_end = end.into();
		let length = exclusive_end.saturating_sub(inclusive_absolute_start);
		(inclusive_absolute_start, length)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.start.clone().into()
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		let exclusive_end = self.end.clone().into();
		exclusive_end.saturating_sub(self.inclusive_absolute_start())
	}
}

impl<T: Into<VirtualAddress>> AbsoluteMemoryRange for RangeInclusive<T>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		let inclusive_absolute_start = self.start().into();
		let end: VirtualAddress = self.end().into();
		let length = end.saturating_increment() - inclusive_absolute_start;
		(inclusive_absolute_start, length)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.start().into()
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		let end: VirtualAddress = self.end().into();
		end.saturating_increment() - self.inclusive_absolute_start()
	}
}

impl<'a, T: 'a + Into<VirtualAddress>> AbsoluteMemoryRange for &'a RangeInclusive<T>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		let inclusive_absolute_start = self.start().into();
		let end: VirtualAddress = self.end().into();
		let length = end.saturating_increment() - inclusive_absolute_start;
		(inclusive_absolute_start, length)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.start().into()
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		let end: VirtualAddress = self.end().into();
		end.saturating_increment() - self.inclusive_absolute_start()
	}
}

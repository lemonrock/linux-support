// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory range that requires no calculations for `self.inclusive_start()` and `self.length()` and is `Copy`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FastAbsoluteMemoryRange
{
	inclusive_absolute_start: VirtualAddress,
	length: usize,
}

impl<'a, Subrange: RelativeMemoryRange> From<(&'a MappedMemory, Subrange)> for FastAbsoluteMemoryRange
{
	#[inline(always)]
	fn from(from: (&'a MappedMemory, Subrange)) -> Self
	{
		let (inclusive_absolute_start, length) = from.inclusive_absolute_start_and_length();
		Self::new(inclusive_absolute_start, length)
	}
}

impl<'a, Subrange: RelativeMemoryRange> From<MappedMemorySubrange<'a, Subrange>> for FastAbsoluteMemoryRange
{
	#[inline(always)]
	fn from(from: MappedMemorySubrange<'a, Subrange>) -> Self
	{
		let (inclusive_absolute_start, length) = from.inclusive_absolute_start_and_length();
		Self::new(inclusive_absolute_start, length)
	}
}

impl<T: Into<VirtualAddress>> From<Range<T>> for FastAbsoluteMemoryRange
{
	#[inline(always)]
	fn from(from: Range<T>) -> Self
	{
		let (inclusive_absolute_start, length) = from.inclusive_absolute_start_and_length();
		Self::new(inclusive_absolute_start, length)
	}
}

impl<'a, T: 'a + Into<VirtualAddress> + Clone> From<&'a Range<T>> for FastAbsoluteMemoryRange
{
	#[inline(always)]
	fn from(from: &'a Range<T>) -> Self
	{
		let (inclusive_absolute_start, length) = from.inclusive_absolute_start_and_length();
		Self::new(inclusive_absolute_start, length)
	}
}

impl<T: Into<VirtualAddress>> From<RangeInclusive<T>> for FastAbsoluteMemoryRange
{
	#[inline(always)]
	fn from(from: RangeInclusive<T>) -> Self
	{
		Self::from(&from)
	}
}

impl<'a, T: 'a + Into<VirtualAddress>> From<&'a RangeInclusive<T>> for FastAbsoluteMemoryRange
{
	#[inline(always)]
	fn from(from: &'a RangeInclusive<T>) -> Self
	{
		let (inclusive_absolute_start, length) = from.inclusive_absolute_start_and_length();
		Self::new(inclusive_absolute_start, length)
	}
}

impl AbsoluteMemoryRange for FastAbsoluteMemoryRange
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
		self.inclusive_absolute_start
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.length
	}
}

impl FastAbsoluteMemoryRange
{
	/// Create a new instance.
	pub const fn new(inclusive_absolute_start: VirtualAddress, length: usize) -> Self
	{
		Self
		{
			inclusive_absolute_start,
			length,
		}
	}
}

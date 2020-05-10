// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Array index data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ArrayIndexing
{
	offset: usize,

	step: usize,

	range: Range<usize>,
}

impl ArrayIndexing
{
	/// New instance.
	#[inline(always)]
	pub(crate) const fn new(offset: usize, step: usize, inclusive_minimum: usize, inclusive_maximum: usize) -> Self
	{
		cfn_assert_eq!(step % 4, 0);
		cfn_assert!(inclusive_maximum > inclusive_minimum);
		cfn_assert!(inclusive_maximum < usize::MAX);

		Self
		{
			offset,
			step,
			range: inclusive_minimum .. (inclusive_maximum + 1),
		}
	}

	#[inline(always)]
	pub(crate) const fn capacity(&self) -> usize
	{
		self.range.end - self.range.start
	}

	#[inline(always)]
	pub(crate) fn iterate(&self, mut offset_user: impl FnMut(usize))
	{
		for index in self.range.clone()
		{
			offset_user(self.offset + (self.step * index))
		}
	}
}

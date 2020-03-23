// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Indexing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Indexing
{
	Singleton
	{
		offset: usize,
	},

	Array(ArrayIndexing),

	SplitArray(ArrayIndexing, ArrayIndexing)
}

impl Indexing
{
	#[inline(always)]
	pub(crate) const fn singleton(offset: usize) -> Self
	{
		Indexing::Singleton
		{
			offset
		}
	}

	#[inline(always)]
	pub(crate) const fn array(offset: usize, step: usize, inclusive_minimum: usize, inclusive_maximum: usize) -> Self
	{
		Indexing::Array(ArrayIndexing::new(offset, step, inclusive_minimum, inclusive_maximum))
	}

	#[inline(always)]
	pub(crate) const fn split_array(offset0: usize, step0: usize, inclusive_minimum0: usize, inclusive_maximum0: usize, offset1: usize, step1: usize, inclusive_minimum1: usize, inclusive_maximum1: usize) -> Self
	{
		Indexing::SplitArray
		(
			ArrayIndexing::new(offset0, step0, inclusive_minimum0, inclusive_maximum0),
			ArrayIndexing::new(offset1, step1, inclusive_minimum1, inclusive_maximum1),
		)
	}
}

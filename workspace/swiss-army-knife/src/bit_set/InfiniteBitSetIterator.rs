// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Infinite Iterator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InfiniteBitSetIterator<'a, BSA: BitSetAware>
{
	iterator: BitSetIterator<'a, BSA>,
}

impl<'a, BSA: BitSetAware> Iterator for InfiniteBitSetIterator<'a, BSA>
{
	type Item = BSA;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if let Some(item) = self.iterator.next()
		{
			return Some(item)
		}
		self.iterator.reset();
		self.iterator.next()
	}
}

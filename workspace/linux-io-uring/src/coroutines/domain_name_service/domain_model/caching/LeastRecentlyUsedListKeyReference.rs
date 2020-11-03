// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct LeastRecentlyUsedListKeyReference<'cache>
{
	key: NonNull<EfficientCaseFoldedName>,
}

impl<'cache> PartialEq for LeastRecentlyUsedListKeyReference<'cache>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		unsafe
		{
			self.key.as_ref().eq(rhs.key.as_ref())
		}
	}
}

impl<'cache> Eq for LeastRecentlyUsedListKeyReference<'cache>
{
}

impl<'cache> Hash for LeastRecentlyUsedListKeyReference<'cache>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		unsafe { self.key.as_ref().hash(state) }
	}
}

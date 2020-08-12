// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct CachedDesiredControllersAndOurDepth(RefCell<Option<(Controllers, usize)>>);

impl CachedDesiredControllersAndOurDepth
{
	fn get(&self, constructor: impl FnOnce() -> (Controllers, usize)) -> (&Controllers, usize)
	{
		let is_uncached = self.0.borrow().is_none();
		if is_uncached
		{
			*self.0.borrow_mut() = Some(constructor())
		}
		self.get_cached()
	}
	
	#[inline(always)]
	fn get_cached(&self) -> (&Controllers, usize)
	{
		let borrow = self.0.borrow();
		let (left, right) = Ref::map_split(borrow, |option| match option
		{
			&None => panic!(),
			&Some((ref left, ref right)) => (left, right)
		});
		
		let left = Ref::leak(left);
		let right = *right;
		(left, right)
	}
}

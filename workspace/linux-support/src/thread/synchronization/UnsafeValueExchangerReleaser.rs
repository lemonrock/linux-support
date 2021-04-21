// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct UnsafeValueExchangerReleaser<V>
{
	parent: usize,
	marker: PhantomData<V>,
	waiting_thread: Thread,
}

unsafe impl<V> Send for UnsafeValueExchangerReleaser<V>
{
}

impl<V> UnsafeValueExchangerReleaser<V>
{
	#[inline(always)]
	pub(super) fn release(self, value: V)
	{
		let parent = unsafe { & * (self.parent as *const UnsafeValueExchanger<V>)};
		parent.store_value(value);
		self.waiting_thread.unpark();
	}
}

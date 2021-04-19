// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(super) struct SimpleTwoThreadBarrierReleaser<V: Debug + Send + Sync + 'static>
{
	inner: SimpleTwoThreadBarrier<V>,
}

impl<V: Debug + Send + Sync + 'static> SimpleTwoThreadBarrierReleaser<V>
{
	#[inline(always)]
	pub(super) fn store_and_release(self, value_to_transfer: V, waiter_thread: &Thread)
	{
		self.inner.value_to_transfer.set(value_to_transfer).unwrap();
		// NOTE: This `drop()` must occur before `store(Release)`, otherwise there may be more than once Arc reference to `value_to_transfer`, and `Arc::try_unwrap()` in `SimpleTwoThreadBarrierWaiter` will panic.
		drop(self.inner.value_to_transfer);
		
		self.inner.lock.store(true, Release);
		waiter_thread.unpark()
	}
}

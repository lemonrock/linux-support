// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(super) struct SimpleTwoThreadBarrierWaiter<V: Debug + Send + Sync + 'static, T: Terminate + 'static>
{
	inner: SimpleTwoThreadBarrier<V>,
	
	terminate: Arc<T>,
}

impl<V: Debug + Send + Sync + 'static, T: Terminate + 'static> SimpleTwoThreadBarrierWaiter<V, T>
{
	#[inline(always)]
	pub(super) fn wait(self) -> Result<V, ()>
	{
		while self.inner.lock.load(Acquire) == SimpleTwoThreadBarrier::<V>::BarrierNotYetReached
		{
			if unlikely!(self.terminate.should_finish())
			{
				return Err(())
			}
			
			park()
		}
		
		let mut value_to_transfer = Arc::try_unwrap(self.inner.value_to_transfer).unwrap();
		Ok(value_to_transfer.take().unwrap())
	}
}

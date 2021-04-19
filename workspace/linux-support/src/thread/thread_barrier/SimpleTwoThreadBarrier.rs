// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(super) struct SimpleTwoThreadBarrier<V: Debug + Send + Sync + 'static>
{
	lock: Arc<AtomicBool>,

	value_to_transfer: Arc<SyncOnceCell<V>>
}

impl<V: Debug + Send + Sync + 'static> SimpleTwoThreadBarrier<V>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			lock: self.lock.clone(),
			value_to_transfer: self.value_to_transfer.clone(),
		}
	}
}

impl<V: Debug + Send + Sync + 'static> SimpleTwoThreadBarrier<V>
{
	const BarrierNotYetReached: bool = false;
	
	#[inline(always)]
	pub(super) fn new<T: Terminate + 'static>(terminate: &Arc<T>) -> (SimpleTwoThreadBarrierWaiter<V, T>, SimpleTwoThreadBarrierReleaser<V>)
	{
		let inner = Self
		{
			lock: Arc::new(AtomicBool::new(Self::BarrierNotYetReached)),
		
			value_to_transfer: Arc::new(SyncOnceCell::new()),
		};
		
		(
			SimpleTwoThreadBarrierWaiter
			{
				inner: inner.clone(),
				
				terminate: terminate.clone(),
			},
			
			SimpleTwoThreadBarrierReleaser
			{
				inner,
			}
		)
	}
}

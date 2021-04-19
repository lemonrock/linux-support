// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(super) struct WaitForSecurityLockDownWaiter<T: Terminate + 'static>
{
	lock: Arc<AtomicBool>,
	
	terminate: Arc<T>,
}

impl<T: Terminate + 'static> WaitForSecurityLockDownWaiter<T>
{
	#[inline(always)]
	pub(super) fn wait(&self) -> Result<(), ()>
	{
		while self.lock.load(Acquire) == WaitForSecurityLockDown::NotYetLockedDown
		{
			if unlikely!(self.terminate.should_finish())
			{
				return Err(())
			}
			park()
		}
		Ok(())
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An efficient, CAS-free unfair spin lock that uses an atomic exchange and only requires one byte of memory.
///
/// Uses Intel TSX Hardware Lock Elision (which should be backwards compatible) to create a spin lock.
/// Inspiration from <https://github.com/cyfdecyf/spinlock>, <https://software.intel.com/en-us/articles/tsx-anti-patterns-in-lock-elision-code> and <http://locklessinc.com/articles/locks/>.
#[derive(Debug)]
pub struct IntelHardwareOptimizedLockSpinLock(UnsafeCell<u8>);

unsafe impl Sync for IntelHardwareOptimizedLockSpinLock
{
}

impl Default for IntelHardwareOptimizedLockSpinLock
{
	#[inline(always)]
	fn default() -> Self
	{
		IntelHardwareOptimizedLockSpinLock(UnsafeCell::new(Self::Unlocked))
	}
}

impl SpinLock for IntelHardwareOptimizedLockSpinLock
{
	#[inline(always)]
	fn acquire_spin_lock(&self)
	{
		while !self.try_to_acquire_spin_lock()
		{
			while self.is_locked()
			{
				spin_loop_hint();
			}
		}
	}
	
	#[inline(always)]
	fn try_to_acquire_spin_lock(&self) -> bool
	{
		// We can either exchange a previous value of Unlocked for Locked or a previous value of Locked for Locked.
		let was_previously = unsafe { __hle_acquire_exchange_n1(self.lock(), Self::Locked) };
		was_previously == Self::Unlocked
	}
	
	#[inline(always)]
	fn unlock_spin_lock(&self)
	{
		debug_assert!(self.is_locked(), "Does not have spin lock");
		
		self.forcibly_unlock_spin_lock()
	}
	
	#[inline(always)]
	fn is_locked(&self) -> bool
	{
		unsafe { *self.lock() == Self::Locked }
	}
	
	#[inline(always)]
	fn is_unlocked(&self) -> bool
	{
		unsafe { *self.lock() == Self::Unlocked }
	}
	
	#[inline(always)]
	fn forcibly_unlock_spin_lock(&self)
	{
		unsafe { __hle_release_store_n1(self.lock(), Self::Unlocked as u32) }
	}
}

impl IntelHardwareOptimizedLockSpinLock
{
	const Unlocked: u8 = 0;
	
	const Locked: u8 = 1;
	
	#[inline(always)]
	fn lock(&self) -> *mut u8
	{
		self.0.get()
	}
}

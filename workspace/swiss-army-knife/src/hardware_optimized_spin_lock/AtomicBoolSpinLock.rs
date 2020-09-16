// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An efficient, CAS-free unfair spin lock that uses an atomic fetch OR and only requires one byte of memory.
#[derive(Debug)]
pub struct AtomicBoolSpinLock(AtomicBool);

unsafe impl Sync for AtomicBoolSpinLock
{
}

impl Default for AtomicBoolSpinLock
{
	#[inline(always)]
	fn default() -> Self
	{
		AtomicBoolSpinLock(AtomicBool::new(Self::Unlocked))
	}
}

impl SpinLock for AtomicBoolSpinLock
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
		// We can either OR a previous value of Unlocked with Locked to get Locked or a previous value of Locked with Locked to get Locked.
		let previously_was = self.0.fetch_or(Self::Locked, Acquire);
		previously_was == Self::Unlocked
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
		self.0.load(Relaxed) == Self::Locked
	}
	
	#[inline(always)]
	fn is_unlocked(&self) -> bool
	{
		self.0.load(Relaxed) == Self::Unlocked
	}
	
	#[inline(always)]
	fn forcibly_unlock_spin_lock(&self)
	{
		self.0.store(Self::Unlocked, Release)
	}
}

#[allow(non_upper_case_globals)]
impl AtomicBoolSpinLock
{
	const Unlocked: bool = false;
	
	const Locked: bool = true;
}

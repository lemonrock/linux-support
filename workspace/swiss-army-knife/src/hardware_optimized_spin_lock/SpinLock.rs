// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A spin-lock API.
pub trait SpinLock: Default + Debug + Sync
{
	/// Returns true if acquired spin lock.
	#[inline(always)]
	fn acquire_spin_lock(&self);
	
	/// Returns true if acquired spin lock.
	#[inline(always)]
	fn try_to_acquire_spin_lock(&self) -> bool;
	
	/// Unlocks the spin lock.
	#[inline(always)]
	fn unlock_spin_lock(&self);
	
	/// Returns true if locked (but not necessarily by this thread).
	#[inline(always)]
	fn is_locked(&self) -> bool;
	
	/// Returns true if unlocked.
	#[inline(always)]
	fn is_unlocked(&self) -> bool;
	
	/// Forcibly unlocks the spin lock, even if it is currently unlocked.
	///
	/// Useful for working with persistent memory.
	#[inline(always)]
	fn forcibly_unlock_spin_lock(&self);
}

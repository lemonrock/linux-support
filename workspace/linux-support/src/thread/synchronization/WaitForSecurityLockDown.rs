// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Synchronizes all threads so they can not begin their main loops until a security lock-down has occurred:-
///
/// * A program-wide seccomp filter is applied (threads are free to apply their own in `ThreadFunction::initialize()`;
/// * A `setuid`-like change of users, groups and supplemental groups has occurred;
/// * Dumpable bits and the like are locked down.
#[derive(Debug)]
pub struct WaitForSecurityLockDown
{
	lock: Arc<AtomicBool>,
	
	waiting_threads: Vec<Thread>,
}

impl WaitForSecurityLockDown
{
	const NotYetLockedDown: bool = false;
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			lock: Arc::new(AtomicBool::new(Self::NotYetLockedDown)),
			
			waiting_threads: vec![],
		}
	}
	
	#[inline(always)]
	pub(super) fn waiter<T: Terminate + 'static>(&self, terminate: &Arc<T>) -> WaitForSecurityLockDownWaiter<T>
	{
		WaitForSecurityLockDownWaiter
		{
			lock: self.lock.clone(),
			terminate: terminate.clone(),
		}
	}
	
	#[inline(always)]
	pub(super) fn register(&mut self, spawned_thread: &SpawnedThread<()>)
	{
		self.waiting_threads.push(spawned_thread.thread().clone())
	}
	
	/// Now locked down.
	#[inline(always)]
	pub fn now_locked_down(self)
	{
		self.lock.store(true, Release);
		
		for waiting_thread in self.waiting_threads
		{
			waiting_thread.unpark()
		}
	}
}

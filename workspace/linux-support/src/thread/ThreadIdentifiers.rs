// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct ThreadIdentifiers
{
	thread_identifier_and_pthread_t_available_barrier: SimpleBarrier,
	thread_identifier: Arc<AtomicI32>,
	pthread_t: Arc<AtomicU64>,
	current_thread: Thread,
}

impl Clone for ThreadIdentifiers
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			thread_identifier_and_pthread_t_available_barrier: self.thread_identifier_and_pthread_t_available_barrier.clone(),
			thread_identifier: Arc::clone(&self.thread_identifier),
			pthread_t: Arc::clone(&self.pthread_t),
			current_thread: self.current_thread.clone()
		}
	}
}

impl ThreadIdentifiers
{
	#[allow(deprecated)]
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		Self
		{
			thread_identifier_and_pthread_t_available_barrier: SimpleBarrier::new(),
			thread_identifier: Arc::new(AtomicI32::new(unsafe { uninitialized() })),
			pthread_t: Arc::new(AtomicU64::new(unsafe { uninitialized() })),
			current_thread: thread::current(),
		}
	}

	#[inline(always)]
	pub(crate) fn set(self)
	{
		let thread_identifier: i32 = ThreadIdentifier::default().into();
		let pthread_t: u64 = unsafe { transmute(pthread_self()) };
		self.thread_identifier.store(thread_identifier, Release);
		self.pthread_t.store(pthread_t, Release);
		self.thread_identifier_and_pthread_t_available_barrier.release_one(self.current_thread);
	}

	#[inline(always)]
	pub(crate) fn get_and_reuse(&self) -> (ThreadIdentifier, pthread_t)
	{
		self.thread_identifier_and_pthread_t_available_barrier.wait_on_parked();
		let values =
		(
			unsafe { transmute(self.thread_identifier.load(Acquire)) },
			unsafe { transmute(self.pthread_t.load(Acquire)) }
		);
		self.reuse();
		values
	}

	#[allow(deprecated)]
	#[inline(always)]
	fn reuse(&self)
	{
		self.thread_identifier_and_pthread_t_available_barrier.reuse();
		self.thread_identifier.store(unsafe { uninitialized() }, Release);
		self.pthread_t.store(unsafe { uninitialized() }, Release);
	}
}

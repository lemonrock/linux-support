// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct SimpleBarrier(Arc<AtomicBool>);

impl Clone for SimpleBarrier
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(Arc::clone(&self.0))
	}
}

impl SimpleBarrier
{
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		Self(Arc::new(AtomicBool::new(false)))
	}

	#[inline(always)]
	pub(crate) fn wait_on_parked(&self)
	{
		while !self.0.load(Acquire)
		{
			// park() can spuriously wake up.
			park();
		}
	}

	#[inline(always)]
	pub(crate) fn release<'a, T: 'a>(&self, join_handles: impl Iterator<Item=&'a JoinHandle<T>>)
	{
		self.0.store(true, Release);

		for join_handle in join_handles
		{
			join_handle.thread().unpark()
		}
	}

	#[inline(always)]
	pub(crate) fn release_one(&self, thread: Thread)
	{
		self.0.store(true, Release);

		thread.unpark()
	}

	#[inline(always)]
	pub(crate) fn reuse(&self)
	{
		self.0.store(false, Release);
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread identifiers for a spawned thread.
#[derive(Debug)]
pub struct SpawnedThread<T>
{
	join_handle: JoinHandle<T>,
	thread_identifier: ThreadIdentifier,
}

impl<T> SpawnedThread<T>
{
	/// Join Handle.
	#[inline(always)]
	pub fn join_handle(self) -> JoinHandle<T>
	{
		self.join_handle
	}
	
	/// Rust thread.
	#[inline(always)]
	pub fn thread(&self) -> &Thread
	{
		self.join_handle.thread()
	}
	
	/// Rust thread identifier.
	#[inline(always)]
	pub fn rust_thread_identifier(&self) -> ThreadId
	{
		self.thread().id()
	}
	
	/// `pthread_t`; also known as `RawPthread`.
	#[inline(always)]
	pub fn pthread_t(&self) -> pthread_t
	{
		self.join_handle.as_pthread_t() as *mut _
	}
	
	/// `tid`.
	#[inline(always)]
	pub fn thread_identifier(&self) -> ThreadIdentifier
	{
		self.thread_identifier
	}
	
	#[inline(always)]
	fn unpark(self)
	{
		self.thread().unpark()
	}
}

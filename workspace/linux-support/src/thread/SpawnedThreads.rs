// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub struct SpawnedThreads<T: Terminate + 'static>
{
	spawned_threads: Vec<SpawnedThread<()>>,
	
	terminate: Arc<T>,
}

impl<T: Terminate + 'static> SpawnedThreads<T>
{
	#[inline(always)]
	fn new(number_of_spawned_threads: usize, terminate: Arc<T>) -> Self
	{
		Self
		{
			spawned_threads: Vec::with_capacity(number_of_spawned_threads),
			
			terminate,
		}
	}
	
	#[inline(always)]
	fn push(&mut self, spawned_thread: SpawnedThread<()>)
	{
		self.spawned_threads.push(spawned_thread)
	}
	
	#[inline(always)]
	pub(crate) fn terminate_ref(&self) -> &Arc<T>
	{
		&self.terminate
	}
	
	/// Spawn threads but configure them from the main (spawning) thread.
	///
	/// On error any threads created are told to run to stop as soon as possible and `terminate` becomes true.
	pub(crate) fn main_thread_spawn_child_threads<PTMAI: PerThreadMemoryAllocatorInstantiator + 'static, TF: ThreadFunction>(mut child_threads: Vec<ThreadSettings<TF>>, terminate: Arc<T>, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>, start_logging: bool, wait_for_security_lock_down: &mut WaitForSecurityLockDown) -> Result<Self, SpawnedThreadError>
		where PTMAI::InstantiationArguments: 'static
	{
		let mut spawned_threads = Self::new(child_threads.len(), terminate);

		for thread_settings in child_threads.drain(..)
		{
			spawned_threads = thread_settings.spawn::<PTMAI, T>(spawned_threads, instantiation_arguments, start_logging, wait_for_security_lock_down)?;
		}
		
		Ok(spawned_threads)
	}
	
	/// Continue or terminate?
	#[inline(always)]
	pub fn continue_or_terminate<V, E>(self, result: Result<V, E>,) -> Result<(Self, V), E>
	{
		match result
		{
			Ok(ok) =>
			{
				Ok((self, ok))
			}
			
			Err(error) =>
			{
				self.terminate();
				Err(error)
			}
		}
	}
	
	/// Terminate.
	#[inline(always)]
	pub fn terminate(self) -> Arc<T>
	{
		self.terminate.begin_termination();
		
		for spawned_thread in self.spawned_threads
		{
			spawned_thread.unpark()
		}
		
		self.terminate
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread settings.
///
/// Effectively a named tuple of values need to create a thread.
#[derive(Debug)]
pub struct ThreadSettings<'a, TF: ThreadFunction>
{
	/// Thread configuration; typically shared amongst many threads.
	pub thread_configuration: &'a ThreadConfiguration,
	
	/// Should normally be for just one HyperThread.
	pub affinity: HyperThreads,
	
	/// Thread function.
	pub thread_function: TF,
}

impl<'a, TF: ThreadFunction> ThreadSettings<'a, TF>
{
	#[inline(always)]
	fn spawn<PTMAI: PerThreadMemoryAllocatorInstantiator + 'static, T: Terminate + 'static>(self, spawned_threads: SpawnedThreads<T>, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>, start_logging: bool, wait_for_security_lock_down: &mut WaitForSecurityLockDown) -> Result<SpawnedThreads<T>, SpawnedThreadError>
	{
		let ThreadSettings { thread_configuration, ref affinity, thread_function } = self;
		
		let terminate = spawned_threads.terminate_ref();
		let result = thread_configuration.spawn::<PTMAI, TF, T>(instantiation_arguments, thread_function, affinity, start_logging, terminate, wait_for_security_lock_down);
		let (mut spawned_threads, spawned_thread) = spawned_threads.continue_or_terminate(result)?;
		spawned_threads.push(spawned_thread);
		Ok(spawned_threads)
	}
	
	#[inline(always)]
	pub(crate) fn configure_main_thread<PTMAI: PerThreadMemoryAllocatorInstantiator, T: Terminate + 'static>(self, spawned_threads: SpawnedThreads<T>, proc_path: &ProcPath, instantiation_arguments: Arc<PTMAI::InstantiationArguments>) -> Result<(SpawnedThreads<T>, TF::TLBF, PTMAI::ThreadDropGuard), MainThreadConfigurationError>
	{
		let ThreadSettings { thread_configuration, ref affinity, thread_function } = self;
		
		let result = thread_configuration.configure_main_thread::<PTMAI>(instantiation_arguments, proc_path, affinity);
		let (spawned_threads, thread_local_allocator_drop_guard) = spawned_threads.continue_or_terminate(result)?;
		
		let result = thread_function.initialize();
		let (spawned_threads, main_thread_loop_function) = spawned_threads.continue_or_terminate(result).map_err(ThreadConfigurationError::ThreadFunctionInitializationFailed)?;
		
		Ok((spawned_threads, main_thread_loop_function, thread_local_allocator_drop_guard))
	}
}

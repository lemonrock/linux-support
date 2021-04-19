// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process executor.
///
/// Starts threads and locks down security using seccomp, capabilities and setuid et al in an intricate dance once a process is secured using `ProcessConfiguration`.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct ProcessExecutor
{
	/// User and group settings.
	pub user_and_group_settings: UserAndGroupSettings,

	/// Seccomp configuration.
	///
	/// SecComp filtering adds a 5% to 10% overhead.
	pub seccomp: Option<PermittedSyscalls>,
	
	/// Parent death signal.
	pub parent_death_signal: Option<Option<Signal>>,
	
	/// Child subreaper process.
	pub child_subreaper_process: Option<Option<ProcessIdentifier>>,
}

impl ProcessExecutor
{
	/// Execute the process.
	///
	/// Creates and configures threads, pauses them, then applies a security lock down, then releases all threads.
	/// The entry thread is considered the main thread that, when it exits, the process would exit.
	/// Always returns an outcome.
	///
	/// Should *ALWAYS* be called after `ProcessConfiguration.configure*()`.
	///
	/// `terminate` should be the value returned from `ProcessConfiguration.configure*()`.
	pub fn execute_securely<T: Terminate + 'static, MainThreadFunction: ThreadFunction, ChildThreadFunction: ThreadFunction, PTMAI: PerThreadMemoryAllocatorInstantiator + 'static>(&self, file_system_layout: &FileSystemLayout, terminate: Arc<T>, main_thread: ThreadSettings<MainThreadFunction>, child_threads: Vec<ThreadSettings<ChildThreadFunction>>, instantiation_arguments: Arc<PTMAI::InstantiationArguments>) -> Result<(), ProcessExecutorError>
		where PTMAI::InstantiationArguments: 'static
	{
		let (spawned_threads, main_thread_loop_body_function, main_thread_local_allocator_drop_guard) = self.prepare_and_secure_threads::<T, MainThreadFunction, ChildThreadFunction, PTMAI>(file_system_layout, terminate, main_thread, child_threads, instantiation_arguments)?;
		
		let (spawned_threads, ()) = spawned_threads.continue_or_terminate(self.apply_after_changing_user_or_group_identifiers_and_initializing_main_thread())?;
		
		Self::execute::<T, MainThreadFunction::TLBF, PTMAI>(spawned_threads, main_thread_loop_body_function, main_thread_local_allocator_drop_guard)
	}

	#[inline(always)]
	fn prepare_and_secure_threads<T: Terminate + 'static, MainThreadFunction: ThreadFunction, ChildThreadFunction: ThreadFunction, PTMAI: PerThreadMemoryAllocatorInstantiator + 'static>(&self, file_system_layout: &FileSystemLayout, terminate: Arc<T>, main_thread: ThreadSettings<MainThreadFunction>, child_threads: Vec<ThreadSettings<ChildThreadFunction>>, instantiation_arguments: Arc<PTMAI::InstantiationArguments>) -> Result<(SpawnedThreads<T>, MainThreadFunction::TLBF, PTMAI::ThreadDropGuard), ProcessExecutorError>
		where PTMAI::InstantiationArguments: 'static
	{
		let (_, proc_path, _, etc_path) = file_system_layout.paths();
		
		let mut wait_for_security_lock_down = WaitForSecurityLockDown::new();
		
		let spawned_threads = SpawnedThreads::main_thread_spawn_child_threads::<PTMAI, ChildThreadFunction>(child_threads, terminate, &instantiation_arguments, false, &mut wait_for_security_lock_down)?;
		
		let (spawned_threads, main_thread_loop_function, thread_local_allocator_drop_guard) = main_thread.configure_main_thread::<PTMAI, T>(spawned_threads, proc_path, instantiation_arguments)?;
		
		let (spawned_threads, ()) = spawned_threads.continue_or_terminate(self.apply_security(etc_path))?;
		wait_for_security_lock_down.now_locked_down();

		Ok((spawned_threads, main_thread_loop_function, thread_local_allocator_drop_guard))
	}

	#[inline(always)]
	fn execute<T: Terminate + 'static, MainThreadLoopBodyFunction: ThreadLoopBodyFunction, PTMAI: PerThreadMemoryAllocatorInstantiator>(spawned_threads: SpawnedThreads<T>, main_thread_loop_body_function: MainThreadLoopBodyFunction, main_thread_local_allocator_drop_guard: PTMAI::ThreadDropGuard) -> Result<(), ProcessExecutorError>
	{
		main_thread_loop_body_function.execute_loop(spawned_threads.terminate_ref());
		
		let terminate = spawned_threads.terminate();

		drop(main_thread_local_allocator_drop_guard);
		
		if terminate.terminated_due_to_panic_or_irrecoverable_error()
		{
			Err(ProcessExecutorError::TerminatedDueToPanicOrIrrecoverableError)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn apply_security(&self, etc_path: &EtcPath) -> Result<(), ProcessExecutorError>
	{
		self.seccomp_for_all_threads()?;
		self.user_and_group_settings.change_user_and_groups(etc_path)?;
		Self::protect_access_to_proc_self_and_disable_core_dumps_needs_to_be_called_as_changing_user_identifiers_resets_process_dumpable_bit()
	}
	
	#[inline(always)]
	fn apply_after_changing_user_or_group_identifiers_and_initializing_main_thread(&self) -> Result<(), ProcessExecutorError>
	{
		use self::ProcessExecutorError::*;
		
		// Must be changed *after* changing effective and file system user and group identifiers.
		if let Some(parent_death_signal) = self.parent_death_signal
		{
			Signal::set_current_process_parent_death_signal(parent_death_signal).map_err(CouldNotSetParentDeathSignal)?;
		}
		
		if let Some(child_subreaper_process) = self.child_subreaper_process
		{
			ProcessIdentifier::set_current_process_child_subreaper_process(child_subreaper_process).map_err(CouldNotSetChildSubreaper)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn seccomp_for_all_threads(&self) -> Result<(), ProcessExecutorError>
	{
		use self::ProcessExecutorError::*;

		if let Some(ref seccomp) = self.seccomp
		{
			seccomp.seccomp_program().load_and_synchronize_all_threads(true, false).map_err(CouldNotLoadSeccompFilters)?.map_err(CouldNotSynchronizeSeccompFiltersOnThread)
		}
		else
		{
			Ok(())
		}
	}

	// This needs to be called after any changes to the process' user identifiers: the process' dumpable bit is reset after the effective user changes.
	#[inline(always)]
	fn protect_access_to_proc_self_and_disable_core_dumps_needs_to_be_called_as_changing_user_identifiers_resets_process_dumpable_bit() -> Result<(), ProcessExecutorError>
	{
		change_dumpable(false).map_err(ProcessExecutorError::CouldNotDisableDumpable)
	}
}

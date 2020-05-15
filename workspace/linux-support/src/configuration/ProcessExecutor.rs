// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process executor.
///
/// Starts threads and locks down security using seccomp, capabilities and setuid et al in an intricate dance once a process is secured using `ProcessConfiguration`.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessExecutor
{
	/// User and group settings.
	#[serde(default)] pub user_and_group_settings: UserAndGroupSettings,

	/// Seccomp configuration.
	///
	/// SecComp filtering adds a 5% to 10% overhead.
	#[serde(default)] pub seccomp: Option<PermittedSyscalls>,
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
	pub fn execute_securely<T: Terminate + 'static, MTF: ThreadFunction, CTF: ThreadFunction>(&self, proc_path: &ProcPath, etc_path: &EtcPath, terminate: &Arc<T>, main_thread: (&ThreadConfiguration, MTF), child_threads: Vec<(&ThreadConfiguration, CTF)>) -> Result<(), ProcessExecutorError>
	{
		let (join_handles, main_thread_loop_function) = self.prepare_and_secure_threads(proc_path, etc_path, terminate, main_thread, child_threads)?;

		Self::execute(join_handles, main_thread_loop_function, terminate)
	}

	#[inline(always)]
	fn execute(join_handles: JoinHandles, mut main_thread_loop_function: impl ThreadLoopBodyFunction, terminate: &Arc<impl Terminate + 'static>) -> Result<(), ProcessExecutorError>
	{
		let terminate = terminate.clone();

		while likely!(terminate.should_continue())
		{
			main_thread_loop_function.invoke(&terminate);
			spin_loop_hint()
		}

		drop(main_thread_loop_function);
		join_handles.join();

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
	fn prepare_and_secure_threads<T: Terminate + 'static, MTF: ThreadFunction, CTF: ThreadFunction>(&self, proc_path: &ProcPath, etc_path: &EtcPath, terminate: &Arc<T>, (main_thread_configuration, main_thread_function): (&ThreadConfiguration, MTF), child_threads: Vec<(&ThreadConfiguration, CTF)>) -> Result<(JoinHandles, MTF::TLBF), ProcessExecutorError>
	{
		#[inline(always)]
		fn ok_or<T: Terminate + 'static, Error>(result: Result<(), Error>, terminate: &Arc<T>, join_handles: JoinHandles, map_err: impl FnOnce(Error) -> ProcessExecutorError) -> Result<JoinHandles, ProcessExecutorError>
		{
			match result
			{
				Ok(()) => Ok(join_handles),
				Err(error) =>
				{
					terminate.clone().begin_termination();
					join_handles.error_join();
					return Err(map_err(error))
				}
			}
		}

		use self::ProcessExecutorError::*;

		let (join_handles, result) = JoinHandles::main_thread_spawn_child_threads(child_threads, terminate, proc_path);
		let join_handles = ok_or(result, terminate, join_handles, CouldNotConfigureChildThreads)?;

		let mut join_handles = ok_or(main_thread_configuration.configure_main_thread(), terminate, join_handles, CouldNotConfigureMainThread)?;
		join_handles.release_configured();

		let main_thread_loop_function = main_thread_function.initialize();
		let mut join_handles = ok_or(self.apply_security(etc_path), terminate, join_handles, |error| error)?;
		join_handles.release_seccomp_applied_and_setuid_et_al_done();

		Ok((join_handles, main_thread_loop_function))
	}

	#[inline(always)]
	fn apply_security(&self, etc_path: &EtcPath) -> Result<(), ProcessExecutorError>
	{
		self.seccomp_for_all_threads()?;
		self.user_and_group_settings.change_user_and_groups(etc_path)?;
		Self::protect_access_to_proc_self_and_disable_core_dumps_needs_to_be_called_as_changing_user_identifiers_resets_process_dumpable_bit()
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
		disable_dumpable().map_err(ProcessExecutorError::CouldNotDisableDumpable)
	}
}

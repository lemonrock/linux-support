// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub struct JoinHandles
{
	wait_until_configured: SimpleBarrier,
	wait_until_seccomp_applied_and_setuid_et_al_done: SimpleBarrier,
	join_handles: Vec<(JoinHandle<()>, ThreadIdentifier, pthread_t)>,
}

impl JoinHandles
{
	/// Spawn threads but configure them from the main (spawning) thread.
	///
	/// On error any threads created are told to run to stop as soon as possible and `terminate` becomes true.
	pub fn main_thread_spawn_child_threads<T: Terminate + 'static, CTF: ThreadFunction, PTMAI: PerThreadMemoryAllocatorInstantiator>(mut child_threads: Vec<(&ThreadConfiguration<PTMAI>, CTF)>, terminate: &Arc<T>, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>, proc_path: &ProcPath) -> (Self, Result<(), ThreadConfigurationError>)
	{
		let mut this = Self
		{
			wait_until_configured: SimpleBarrier::new(),
			wait_until_seccomp_applied_and_setuid_et_al_done: SimpleBarrier::new(),
			join_handles: Vec::with_capacity(child_threads.len()),
		};

		let thread_identifiers = ThreadIdentifiers::new();
		for (thread_configuration, thread_function) in child_threads.drain(..)
		{
			if let Err(error) = this.add_thread(&thread_identifiers, terminate, thread_configuration, thread_function, instantiation_arguments, proc_path)
			{
				return (this, Err(error))
			}
		}

		(this, Ok(()))
	}

	fn add_thread<T: Terminate + 'static, CTF: ThreadFunction, PTMAI: PerThreadMemoryAllocatorInstantiator>(&mut self, thread_identifiers: &ThreadIdentifiers, terminate: &Arc<T>, thread_configuration: &ThreadConfiguration<PTMAI>, thread_function: CTF, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>, proc_path: &ProcPath) -> Result<(), ThreadConfigurationError>
	{
		let join_handle = self.spawn(thread_identifiers, terminate, thread_configuration, thread_function, instantiation_arguments).map_err(ThreadConfigurationError::CouldNotCreateThread)?;
		let (thread_identifier, pthread_t) = thread_identifiers.get_and_reuse();
		self.join_handles.push((join_handle, thread_identifier, pthread_t));

		// Now configure the thread.
		thread_configuration.configure_from_main_thread(thread_identifier, pthread_t, proc_path)?;

		Ok(())
	}

	fn spawn<T: Terminate + 'static, CTF: ThreadFunction, PTMAI: PerThreadMemoryAllocatorInstantiator>(&self, thread_identifiers: &ThreadIdentifiers, terminate: &Arc<T>, thread_configuration: &ThreadConfiguration<PTMAI>, thread_function: CTF, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>) -> io::Result<JoinHandle<()>>
	{
		let thread_identifiers = thread_identifiers.clone();
		let (wait_until_configured, wait_until_seccomp_applied_and_setuid_et_al_done) = self.clone_barriers();
		let terminate_catch_unwind = terminate.clone();
		let terminate_error = terminate.clone();
		let thread_capabilities = thread_configuration.capabilities.clone();
		thread_configuration.spawn
		(
			instantiation_arguments,
			move ||
			{
				let result = catch_unwind
				(
					AssertUnwindSafe
					(
						move ||
						{
							thread_identifiers.set();

							if unlikely!(terminate_catch_unwind.should_finish())
							{
								return
							}

							wait_until_configured.wait_on_parked();

							if unlikely!(terminate_catch_unwind.should_finish())
							{
								return
							}

							let mut thread_loop_body_function = thread_function.initialize();

							if unlikely!(terminate_catch_unwind.should_finish())
							{
								return
							}

							if let Some(capabilities) = thread_capabilities
							{
								capabilities.configure_if_wanted()
							}
							else
							{
								ThreadCapabilitiesConfiguration::configure_if_unwanted()
							}.unwrap();

							if unlikely!(terminate_catch_unwind.should_finish())
							{
								return
							}

							wait_until_seccomp_applied_and_setuid_et_al_done.wait_on_parked();

							while likely!(terminate_catch_unwind.should_continue())
							{
								thread_loop_body_function.invoke(&terminate_catch_unwind);
								spin_loop_hint()
							}
						}
					)
				);

				if let Err(payload) = result
				{
					terminate_error.begin_termination_due_to_irrecoverable_error(&payload, None);
					resume_unwind(payload)
				}
			}
		)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn thread_identifiers_for_thread(&self, relative_thread_identifier: usize) -> Option<(ThreadId, ThreadIdentifier, pthread_t)>
	{
		self.join_handles.get(relative_thread_identifier).map(|&(ref join_handle, thread_identifier, pthread_t)| (join_handle.thread().id(), thread_identifier, pthread_t))
	}

	/// Join on join handles.
	#[inline(always)]
	pub fn error_join(mut self)
	{
		// Belt-and-braces
		self.release_all();

		self.join()
	}

	/// Join on join handles.
	#[inline(always)]
	pub fn join(mut self)
	{
		for (join_handle, _, _) in self.join_handles.drain(..)
		{
			let _ = join_handle.join();
		}
	}

	#[inline(always)]
	pub(crate) fn release_configured(&mut self)
	{
		self.wait_until_configured.set_release();

		SimpleBarrier::unpark_all(self.join_handles.iter().map(|(join_handle, _, _)| join_handle))
	}

	#[inline(always)]
	pub(crate) fn release_seccomp_applied_and_setuid_et_al_done(&mut self)
	{
		self.wait_until_seccomp_applied_and_setuid_et_al_done.set_release();

		SimpleBarrier::unpark_all(self.join_handles.iter().map(|(join_handle, _, _)| join_handle))
	}

	#[inline(always)]
	fn release_all(&mut self)
	{
		// Done in reverse order in case of spurious wake ups.
		self.wait_until_seccomp_applied_and_setuid_et_al_done.set_release();
		self.wait_until_configured.set_release();

		SimpleBarrier::unpark_all(self.join_handles.iter().map(|(join_handle, _, _)| join_handle))
	}

	#[inline(always)]
	fn clone_barriers(&self) -> (SimpleBarrier, SimpleBarrier)
	{
		(self.wait_until_configured.clone(), self.wait_until_seccomp_applied_and_setuid_et_al_done.clone())
	}
}

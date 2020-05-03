// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct JoinHandles
{
	barrier: SimpleBarrier,
	join_handles: Vec<(JoinHandle<()>, ThreadIdentifier, pthread_t)>,
}

impl Drop for JoinHandles
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.join();
	}
}

impl JoinHandles
{
	/// Spawn threads but configure them from the main (spawning) thread.
	///
	/// On error any threads created are told to run to stop as soon as possible and `terminate` becomes true.
	pub fn main_thread_spawn_configured_child_threads<TLBF: ThreadLoopBodyFunction, T: Terminate>(thread_configurations: &[(ThreadConfiguration, TLBF)], terminate: &Arc<T>, proc_path: &ProcPath) -> (Self, Result<(), ThreadConfigurationError>)
	{
		let mut this = Self
		{
			barrier: SimpleBarrier::new(),
			join_handles: Vec::with_capacity(thread_configurations.len()),
		};

		let thread_identifiers = ThreadIdentifiers::new();
		for (thread_configuration, thread_loop_body_function) in thread_configurations.iter()
		{
			if let Err(error) = this.add_thread(&thread_identifiers, terminate, thread_configuration, thread_loop_body_function, proc_path)
			{
				return (this, Err(error))
			}
		}

		(this, Ok(()))
	}

	fn add_thread(&mut self, thread_identifiers: &ThreadIdentifiers, terminate: &Arc<impl Terminate>, thread_configuration: &ThreadConfiguration, thread_loop_body_function: &impl ThreadLoopBodyFunction, proc_path: &ProcPath) -> Result<(), ThreadConfigurationError>
	{
		let join_handle = self.spawn(thread_identifiers, terminate, thread_configuration, *thread_loop_body_function).map_err(ThreadConfigurationError::CouldNotCreateThread)?;
		let (thread_identifier, pthread_t) = thread_identifiers.get_and_reuse();
		self.join_handles.push((join_handle, thread_identifier, pthread_t));

		// Now configure the thread.
		thread_configuration.configure_from_main_thread(thread_identifier, pthread_t, proc_path)?;

		Ok(())
	}

	fn spawn(&self, thread_identifiers: &ThreadIdentifiers, terminate: &Arc<impl Terminate + 'static>, thread_configuration: &ThreadConfiguration, thread_loop_body_function: impl ThreadLoopBodyFunction) -> io::Result<JoinHandle<()>>
	{
		let thread_identifiers = thread_identifiers.clone();
		let wait_until_configured = self.clone_barrier();
		let terminate = terminate.clone();
		thread_configuration.spawn
		(
			||
			{
				let result = catch_unwind
				(
					AssertUnwindSafe
					(
						move ||
						{
							thread_identifiers.set();

							wait_until_configured.wait_on_parked();

							while likely!(terminate.should_continue())
							{
								thread_loop_body_function.invoke()
							}
						}
					)
				);

				if let Err(payload) = result
				{
					terminate.begin_termination_due_to_irrecoverable_error(&payload);
					resume_unwind(payload)
				}
			}
		)
	}

	#[inline(always)]
	pub fn thread_identifiers_for_thread(&self, relative_thread_identifier: usize) -> Option<(ThreadId, ThreadIdentifier, pthread_t)>
	{
		self.join_handles.get(relative_thread_identifier).map(|&(ref join_handle, thread_identifier, pthread_t)| (join_handle.thread().id(), thread_identifier, pthread_t))
	}

	/// Join on join handles.
	#[inline(always)]
	pub fn join(mut self)
	{
		// Belt-and-braces.
		self.release();

		for (join_handle, _, _) in self.join_handles.drain(..)
		{
			let _ = join_handle.join();
		}
	}

	#[inline(always)]
	pub(crate) fn release(&mut self)
	{
		self.barrier.release(self.join_handles.iter().map(|(join_handle, _, _)| join_handle))
	}

	#[inline(always)]
	fn clone_barrier(&self) -> SimpleBarrier
	{
		self.barrier.clone()
	}
}

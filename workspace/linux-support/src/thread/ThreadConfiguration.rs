// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread configuration.
///
/// Used to create a thread and within a thread.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThreadConfiguration
{
	/// Thread name.
	#[serde(default)] pub name: ThreadName,

	/// Thread stack size.
	#[serde(default = "ThreadConfiguration::stack_size_default")] pub stack_size: NonZeroNumberOfPages,

	/// Thread HyperThread affinity.
	#[serde(default)] pub affinity: BitSet<HyperThread>,

	/// Sets the scheduler policy for the thread.
	#[serde(default)] pub thread_scheduler: PerThreadSchedulerPolicyAndFlags,

	#[allow(missing_docs)]
	#[serde(default)] pub disable_transparent_huge_pages: bool,
}

impl Default for ThreadConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			name: Default::default(),
			stack_size: Self::stack_size_default(),
			affinity: Default::default(),
			thread_scheduler: Default::default(),
			disable_transparent_huge_pages: false
		}
	}
}

impl ThreadConfiguration
{
	/// Spawns and configures a new thread.
	///
	/// The new thread inherits copies of the calling thread's capability sets and CPU affinity mask (see `sched_setaffinity()`).
	#[inline(always)]
	pub fn spawn<F, T>(&self, f: F) -> io::Result<JoinHandle<T>>
	where
		F: FnOnce() -> T,
		F: std::marker::Send + 'static,
		T: std::marker::Send + 'static,
	{
		let stack_size = self.stack_size.get() * PageSize::current().size_in_bytes().get();
		Builder::new().name(self.name.to_string()).stack_size(stack_size as usize).spawn(move ||
		{
			adjust_transparent_huge_pages(!self.disable_transparent_huge_pages);
			f()
		})
	}

	/// Configure.
	#[inline(always)]
	fn configure_from_main_thread(&self, thread_identifier: ThreadIdentifier, pthread_t: pthread_t, proc_path: &ProcPath) -> Result<(), ThreadConfigurationError>
	{
		use self::ThreadConfigurationError::*;

		self.name.set_thread_name(ProcessIdentifierChoice::Current, thread_identifier, proc_path).map_err(CouldNotSetThreadName)?;

		self.affinity.set_thread_affinity(pthread_t).map_err(CouldNotSetThreadAffinity)?;

		self.thread_scheduler.set_for_thread(ThreadIdentifierChoice::Other(thread_identifier)).map_err(CouldNotSetSchedulerPolicyAndFlags)?;

		Ok(())
	}

	/// Configure.
	#[inline(always)]
	pub fn configure_main_thread(&self) -> Result<(), ThreadConfigurationError>
	{
		use self::ThreadConfigurationError::*;

		adjust_transparent_huge_pages(!self.disable_transparent_huge_pages);

		self.name.set_current_thread_name().map_err(CouldNotSetThreadName)?;

		self.affinity.set_current_thread_affinity().map_err(CouldNotSetThreadAffinity)?;

		self.thread_scheduler.set_for_thread(ThreadIdentifierChoice::Current).map_err(CouldNotSetSchedulerPolicyAndFlags)?;

		Ok(())
	}

	#[inline(always)]
	fn stack_size_default() -> NonZeroNumberOfPages
	{
		unsafe { NonZeroNumberOfPages::new_unchecked(2_048 * 1_024 / PageSize::current().size_in_bytes()) }
	}
}

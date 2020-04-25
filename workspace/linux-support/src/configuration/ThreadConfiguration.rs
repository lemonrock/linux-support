// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread configuration.
///
/// Used to create a thread and within a thread.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ThreadConfiguration
{
	/// Thread name.
	pub name: ThreadName,

	/// Thread stack size.
	pub stack_size: NumberOfPages,

	/// Sets the scheduler policy for the thread.
	pub thread_scheduler: PerThreadSchedulerPolicyAndFlags,
}

impl ThreadConfiguration
{
	/// Spawns and configures a new thread.
	#[inline(always)]
	pub fn spawn<F: FnOnce() -> T + Send + 'static, T: Send + 'static>(&self, proc_path: &ProcPath, thread_function: F) -> io::Result<JoinHandle<Result<T, ThreadConfigurationError>>>
	{
		const page_size: usize = (PageSize::current() as u64 as usize);
		Builder::new().name(name.to_string()).stack_size(self.stack_size as usize * page_size).spawn(move ||
		{
			self.configure(proc_path)?;
			thread_function
		})
	}

	/// Configure.
	#[inline(always)]
	fn configure(&self, proc_path: &ProcPath) -> Result<(), ThreadConfigurationError>
	{
		use self::ThreadConfigurationError::*;

		self.name.set_current_thread_name()?;

		self.thread_scheduler.set_for_thread(ProcessIdentifierChoice::Current.thread_identifier()).map_err(|reason| CouldNotSetSchedulerPolicyAndFlags(reason))?;

		no_new_privileges();

		Ok(())
	}
}

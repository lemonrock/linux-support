// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread configuration.
///
/// Used to create a thread and within a thread.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
	pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
	where
		F: FnOnce() -> T,
		F: std::marker::Send + 'static,
		T: std::marker::Send + 'static,
	{
		// TODO: Alternative design: make the thread block on a lock, grab its thread-id then configure it from main thread; can release it whenever we are ready to.
		// TODO: Change the main thread, too.
		// TODO: Set cpuset.
		// TODO: Set NUMA.
		// TODO: kill all threads on panic.

		const page_size: usize = PageSize::current() as u64 as usize;
		Builder::new().name(self.name.to_string()).stack_size(self.stack_size as usize * page_size).spawn(move ||
		{
			Self::configure_panic_hook();
			let t =
			{
				self.configure().unwrap();
				f()
			};
			Self::unconfigure_panic_hook();
			t
		})
	}

	/// Configure.
	#[inline(always)]
	fn configure(&self) -> Result<(), ThreadConfigurationError>
	{
		use self::ThreadConfigurationError::*;

		self.name.set_current_thread_name().map_err(|cause| CouldNotSetThreadName(cause))?;

		self.thread_scheduler.set_for_thread(ThreadIdentifierChoice::Current).map_err(|reason| CouldNotSetSchedulerPolicyAndFlags(reason))?;

		no_new_privileges();

		Ok(())
	}

	#[inline(always)]
	fn configure_panic_hook()
	{
		set_hook(Box::new(|panic_info|
		{
			let (source_file, line_number, column_number) = match panic_info.location()
			{
				None => ("(unknown source file)", 0, 0),
				Some(location) => (location.file(), location.line(), location.column())
			};

			let cause = panic_payload_to_cause(panic_info.payload());

			ProcessLoggingConfiguration::caught_panic(source_file, line_number, column_number, &cause)
		}));
	}

	#[inline(always)]
	fn unconfigure_panic_hook()
	{
		drop(take_hook());
	}
}

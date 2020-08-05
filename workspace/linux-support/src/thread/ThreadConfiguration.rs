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
	///
	/// This can not be changed for the main thread.
	#[serde(default = "ThreadConfiguration::stack_size_default")] pub stack_size: NonZeroNumberOfPages,
	
	/// Sets the nice value for the thread.
	#[serde(default)] pub nice: Option<Nice>,

	/// Sets the IO priority (ionice or ioprio) for the thread.
	#[serde(default)] pub io_priority: Option<IoPriority>,

	/// Sets the scheduler policy for the thread.
	#[serde(default)] pub thread_scheduler: PerThreadSchedulerPolicyAndFlags,

	/// NUMA memory policy.
	#[serde(default)] numa_memory_policy: Option<SetMemoryPolicy>,

	#[allow(missing_docs)]
	#[serde(default)] pub disable_transparent_huge_pages: bool,

	/// Capabilities to apply after configuring the thread but before executing the thread loop.
	///
	/// If capabilities are used the `keep capabilities` securebit is also set and locked.
	#[serde(default)] pub capabilities: Option<Arc<ThreadCapabilitiesConfiguration>>,
	
	/// Memory choices for thread-local allocator.
	#[serde(default)] pub thread_local_allocator_configuration: Arc<ThreadLocalAllocatorConfiguration>,
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
			nice: None,
			io_priority: None,
			thread_scheduler: Default::default(),
			numa_memory_policy: None,
			disable_transparent_huge_pages: false,
			capabilities: None,
			thread_local_allocator_configuration: Default::default()
		}
	}
}

impl ThreadConfiguration
{
	/// Spawns and configures a new thread.
	///
	/// The new thread inherits copies of the calling thread's capability sets and CPU affinity mask (see `sched_setaffinity()`).
	#[inline(always)]
	pub(crate) fn spawn<PTMAI: PerThreadMemoryAllocatorInstantiator, F, T>(&self, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>, f: F) -> io::Result<JoinHandle<T>>
	where
		F: FnOnce() -> T,
		F: std::marker::Send + 'static,
		T: std::marker::Send + 'static,
		PTMAI::InstantiationArguments: 'static,
	{
		let stack_size = self.stack_size.get() * PageSize::current().size_in_bytes().get();
		let numa_memory_policy = self.numa_memory_policy.clone();
		let disable_transparent_huge_pages = self.disable_transparent_huge_pages;
		let thread_local_allocator_configuration = self.thread_local_allocator_configuration.clone();
		let instantiation_arguments = instantiation_arguments.clone();
		Builder::new().name(self.name.to_string()).stack_size(stack_size as usize).spawn(move ||
		{
			if let Some(ref numa_memory_policy) = numa_memory_policy
			{
				numa_memory_policy.set_thread_policy()
			}

			adjust_transparent_huge_pages(!disable_transparent_huge_pages);
			
			let thread_local_allocator_drop_guard = PTMAI::instantiate(thread_local_allocator_configuration, instantiation_arguments).expect("Could not allocate a thread-local allocator");
			
			(unsafe { LocalSyslogSocket::configure_per_thread_local_syslog_socket() }).expect("Could not start logging");
			
			let t = f();
			
			drop(thread_local_allocator_drop_guard);
			
			t
		})
	}

	/// Configure.
	#[inline(always)]
	pub(crate) fn configure_from_main_thread(&self, thread_identifier: ThreadIdentifier, pthread_t: pthread_t, proc_path: &ProcPath, affinity: &HyperThreads) -> Result<(), ThreadConfigurationError>
	{
		self.configure_thread(pthread_t, thread_identifier, proc_path, affinity)
	}
	
	/// Configure.
	#[inline(always)]
	pub fn configure_main_thread<PTMAI: PerThreadMemoryAllocatorInstantiator>(&self, instantiation_arguments: Arc<PTMAI::InstantiationArguments>, proc_path: &ProcPath, affinity: &HyperThreads) -> Result<PTMAI::ThreadDropGuard, ThreadConfigurationError>
	{
		if let Some(ref numa_memory_policy) = self.numa_memory_policy
		{
			numa_memory_policy.set_thread_policy()
		}
		
		adjust_transparent_huge_pages(!self.disable_transparent_huge_pages);
		
		let thread_local_allocator_drop_guard = PTMAI::instantiate(self.thread_local_allocator_configuration.clone(), instantiation_arguments).expect("Could not allocate a thread-local allocator");
		
		// `(unsafe { LocalSyslogSocket::configure_per_thread_local_syslog_socket() }).expect("Could not start logging");` is not required as it is done in `LocalSyslogSocketConfiguration::configure()`.
		
		self.configure_thread(unsafe { pthread_self() }, ThreadIdentifier::default(), proc_path, affinity)?;
		
		Ok(thread_local_allocator_drop_guard)
	}
	
	#[inline(always)]
	fn configure_thread(&self, pthread_t: pthread_t, thread_identifier: ThreadIdentifier, proc_path: &ProcPath, affinity: &HyperThreads) -> Result<(), ThreadConfigurationError>
	{
		use self::ThreadConfigurationError::*;
		
		self.name.set_thread_name(ProcessIdentifierChoice::Current, thread_identifier, proc_path).map_err(CouldNotSetThreadName)?;
		
		affinity.set_thread_affinity(pthread_t).map_err(CouldNotSetThreadAffinity)?;
		
		if let Some(nice) = self.nice
		{
			nice.set_thread_priority(ThreadIdentifier::default()).map_err(|_: ()| CouldNotSetNice)?
		}
		
		if let Some(io_priority) = self.io_priority
		{
			io_priority.set_for_thread(ThreadIdentifier::default()).map_err(CouldNotSetIoPriority)?
		}
		
		self.thread_scheduler.set_for_thread(ThreadIdentifierChoice::Other(thread_identifier)).map_err(ThreadConfigurationError::CouldNotSetSchedulerPolicyAndFlags)?;
		
		Ok(())
	}
	
	#[inline(always)]
	fn stack_size_default() -> NonZeroNumberOfPages
	{
		const _2MB: u64 = 2_048 * 1_024;
		unsafe { NonZeroNumberOfPages::new_unchecked(_2MB / PageSize::current().size_in_bytes().get()) }
	}
}

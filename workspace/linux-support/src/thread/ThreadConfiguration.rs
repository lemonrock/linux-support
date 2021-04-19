// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread configuration that can only be set (configured) from a running thread.
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
	
	/// NUMA memory policy.
	#[serde(default)] pub numa_memory_policy: Option<SetMemoryPolicy>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub disable_transparent_huge_pages: Option<bool>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub current_timer_slack: Option<Option<CurrentTimerSlackNanoseconds>>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub store_bypass_speculation_mitigation_control: Option<StoreBypassSpeculationMitigationControlChangeOperation>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub indirect_branch_speculation_mitigation_control: Option<IndirectBranchSpeculationMitigationControlChangeOperation>,
	
	/// Memory choices for thread-local allocator.
	#[serde(default)] pub thread_local_allocator_configuration: Arc<ThreadLocalAllocatorConfiguration>,
	
	/// Sets the nice value for the thread.
	#[serde(default)] pub nice: Option<Nice>,
	
	/// Sets the IO priority (ionice or ioprio) for the thread.
	#[serde(default)] pub io_priority: Option<IoPriority>,
	
	/// Sets the scheduler policy for the thread.
	#[serde(default)] pub thread_scheduler: Arc<PerThreadSchedulerPolicyAndFlags>,
	
	/// Capabilities to apply after configuring the thread but before executing the thread loop.
	///
	/// If capabilities are used the `keep capabilities` securebit is also set and locked.
	///
	/// This can not be changed for the main thread.
	#[serde(default)] pub capabilities: Option<Arc<ThreadCapabilitiesConfiguration>>,
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
			numa_memory_policy: None,
			disable_transparent_huge_pages: None,
			current_timer_slack: None,
			store_bypass_speculation_mitigation_control: None,
			indirect_branch_speculation_mitigation_control: None,
			thread_local_allocator_configuration: Default::default(),
			nice: None,
			io_priority: None,
			thread_scheduler: Default::default(),
			capabilities: None,
		}
	}
}

impl ThreadConfiguration
{
	/// Configures the main thread.
	///
	/// `stack_size` is ignored and not changed.
	#[inline(always)]
	pub fn configure_main_thread<PTMAI: PerThreadMemoryAllocatorInstantiator>(&self, instantiation_arguments: Arc<PTMAI::InstantiationArguments>, proc_path: &ProcPath, affinity: &HyperThreads) -> Result<PTMAI::ThreadDropGuard, MainThreadConfigurationError>
	{
		self.name.set_thread_name(ProcessIdentifierChoice::Current, ThreadIdentifier::default(), proc_path).map_err(MainThreadConfigurationError::CouldNotSetThreadName)?;
		
		// No need to `start_logging` as this is done in `LocalSyslogSocketConfiguration::configure()`.
		let thread_local_allocator_drop_guard = Self::most_thread_configuration::<PTMAI>(self.numa_memory_policy.as_ref(), self.disable_transparent_huge_pages, self.current_timer_slack, self.store_bypass_speculation_mitigation_control, self.indirect_branch_speculation_mitigation_control, self.thread_local_allocator_configuration.clone(), instantiation_arguments, false, affinity, self.nice, self.io_priority, &self.thread_scheduler)?;
		
		Ok(thread_local_allocator_drop_guard)
	}
	
	/// Spawns and configures a new thread.
	///
	/// The new thread inherits copies of the calling thread's capability sets and CPU affinity mask (see `sched_setaffinity()`).
	/// Method returns once the spawned thread has been configured.
	///
	/// Configuration is complex:-
	///
	/// * because there is no way to find the ThreadIdentifier (`tid`) except from the running thread;
	/// * because some thread specific values are best set from the thread creating the spawned thread in order to better receive panics and avoid Arcs or clones of data structures otherwise passed to the spawned thread.
	/// * Because setting capabilities and seccomp can only occur once a thread has done some initialization work (eg opening a network connection, after which it needs to drop privileged port access).
	#[inline(always)]
	pub fn spawn<PTMAI: PerThreadMemoryAllocatorInstantiator, TF: ThreadFunction, T: Terminate + 'static>(&self, instantiation_arguments: &Arc<PTMAI::InstantiationArguments>, thread_function: TF, affinity: &HyperThreads, start_logging: bool, terminate: &Arc<T>, wait_for_security_lock_down: &mut WaitForSecurityLockDown) -> Result<SpawnedThread<()>, SpawnedThreadError>
	where PTMAI::InstantiationArguments: 'static,
	{
		let wait_for_security_lock_down_waiter = wait_for_security_lock_down.waiter(terminate);
		let (spawned_thread_identifier_has_been_set_waiter, spawned_thread_identifier_has_been_set_releaser) = SimpleTwoThreadBarrier::new(terminate);
		
		let thread_code =
		{
			let numa_memory_policy = self.numa_memory_policy.clone();
			let disable_transparent_huge_pages = self.disable_transparent_huge_pages;
			let current_timer_slack = self.current_timer_slack;
			let store_bypass_speculation_mitigation_control = self.store_bypass_speculation_mitigation_control;
			let indirect_branch_speculation_mitigation_control = self.indirect_branch_speculation_mitigation_control;
			let thread_local_allocator_configuration = self.thread_local_allocator_configuration.clone();
			let instantiation_arguments = instantiation_arguments.clone();
			let affinity_between_threads_hack = affinity as *const HyperThreads as usize;
			let nice = self.nice;
			let io_priority = self.io_priority;
			let thread_scheduler = self.thread_scheduler.clone();
			let capabilities = self.capabilities.clone();
			let terminate = terminate.clone();
			let spawning_thread = current();
			move ||
			{
				let (thread_loop_body_function, thread_local_allocator_drop_guard) = match Self::configure_and_initialized_spawned_thread::<PTMAI, TF, T>(numa_memory_policy, disable_transparent_huge_pages, current_timer_slack, store_bypass_speculation_mitigation_control, indirect_branch_speculation_mitigation_control, thread_local_allocator_configuration, instantiation_arguments, start_logging, affinity_between_threads_hack, nice, io_priority, thread_scheduler, capabilities, thread_function)
				{
					Ok((thread_loop_body_function, thread_local_allocator_drop_guard)) =>
					{
						spawned_thread_identifier_has_been_set_releaser.store_and_release(Ok(ThreadIdentifier::default()), &spawning_thread);
						(thread_loop_body_function, thread_local_allocator_drop_guard)
					},
					
					Err(error) =>
					{
						spawned_thread_identifier_has_been_set_releaser.store_and_release(Err(error), &spawning_thread);
						return
					}
				};
				
				let _ = wait_for_security_lock_down_waiter.wait();
				thread_loop_body_function.execute_loop(&terminate);
				
				drop(thread_local_allocator_drop_guard);
			}
		};
		
		let join_handle = self.spawn_catching_panics(terminate.clone(), thread_code)?;
		yield_now();
		
		let thread_identifier = spawned_thread_identifier_has_been_set_waiter.wait().map_err(|()| SpawnedThreadError::SpawnedThreadTerminatedBeforeSettingThreadIdentifier)??;
		let spawned_thread = SpawnedThread
		{
			join_handle,
			thread_identifier
		};
		wait_for_security_lock_down.register(&spawned_thread);
		
		drop(affinity);
		
		Ok(spawned_thread)
	}
	
	fn spawn_catching_panics<T: Terminate + Send + Sync + 'static>(&self, terminate: Arc<T>, thread_code: impl FnOnce() + Send + Sync + 'static) -> Result<JoinHandle<()>, SpawnedThreadError>
	{
		self.builder().spawn
		(
			move ||
			{
				if let Err(payload) = catch_unwind(AssertUnwindSafe(thread_code))
				{
					terminate.begin_termination_due_to_irrecoverable_error(&payload, None);
					resume_unwind(payload)
				}
			}
		).map_err(SpawnedThreadError::CouldNotSpawnThread)
	}
	
	#[inline(always)]
	fn configure_and_initialized_spawned_thread<PTMAI: PerThreadMemoryAllocatorInstantiator, TF: ThreadFunction, T: Terminate + 'static>(numa_memory_policy: Option<SetMemoryPolicy>, disable_transparent_huge_pages: Option<bool>, current_timer_slack: Option<Option<CurrentTimerSlackNanoseconds>>, store_bypass_speculation_mitigation_control: Option<StoreBypassSpeculationMitigationControlChangeOperation>, indirect_branch_speculation_mitigation_control: Option<IndirectBranchSpeculationMitigationControlChangeOperation>, thread_local_allocator_configuration: Arc<ThreadLocalAllocatorConfiguration>, instantiation_arguments: Arc<PTMAI::InstantiationArguments>, start_logging: bool, affinity_between_threads_hack: usize, nice: Option<Nice>, io_priority: Option<IoPriority>, thread_scheduler: Arc<PerThreadSchedulerPolicyAndFlags>, capabilities: Option<Arc<ThreadCapabilitiesConfiguration>>, thread_function: TF) -> Result<(TF::TLBF, PTMAI::ThreadDropGuard), ThreadConfigurationError>
	{
		let thread_local_allocator_drop_guard = Self::most_thread_configuration::<PTMAI>(numa_memory_policy.as_ref(), disable_transparent_huge_pages, current_timer_slack, store_bypass_speculation_mitigation_control, indirect_branch_speculation_mitigation_control, thread_local_allocator_configuration, instantiation_arguments, start_logging, unsafe { & * (affinity_between_threads_hack as *const HyperThreads) }, nice, io_priority, &thread_scheduler)?;
		
		let thread_loop_body_function = thread_function.initialize().map_err(ThreadConfigurationError::ThreadFunctionInitializationFailed)?;
		
		Self::configure_thread_capabilities_after_initializing_thread_loop_body_function(capabilities)?;
		
		Ok((thread_loop_body_function, thread_local_allocator_drop_guard))
	}
	
	#[inline(always)]
	fn most_thread_configuration<PTMAI: PerThreadMemoryAllocatorInstantiator>(numa_memory_policy: Option<&SetMemoryPolicy>, disable_transparent_huge_pages: Option<bool>, current_timer_slack: Option<Option<CurrentTimerSlackNanoseconds>>, store_bypass_speculation_mitigation_control: Option<StoreBypassSpeculationMitigationControlChangeOperation>, indirect_branch_speculation_mitigation_control: Option<IndirectBranchSpeculationMitigationControlChangeOperation>, thread_local_allocator_configuration: Arc<ThreadLocalAllocatorConfiguration>, instantiation_arguments: Arc<PTMAI::InstantiationArguments>, start_logging: bool, affinity: &HyperThreads, nice: Option<Nice>, io_priority: Option<IoPriority>, thread_scheduler: &PerThreadSchedulerPolicyAndFlags) -> Result<PTMAI::ThreadDropGuard, ThreadConfigurationError>
	{
		use self::ThreadConfigurationError::*;
		
		if let Some(numa_memory_policy) = numa_memory_policy
		{
			numa_memory_policy.set_thread_policy()
		}
		
		if let Some(disable_transparent_huge_pages) = disable_transparent_huge_pages
		{
			change_transparent_huge_pages(!disable_transparent_huge_pages).map_err(DisableTransparentHugePages)?;
		}
		
		match current_timer_slack
		{
			None => (),
			
			Some(None) => CurrentTimerSlackNanoseconds::reset_to_default().expect("Could not reset current timer slack to default"),
			
			Some(Some(current_time_slack)) => current_time_slack.set().expect("Could not change current timer slack"),
		}
		
		if let Some(store_bypass_speculation_mitigation_control) = store_bypass_speculation_mitigation_control
		{
			store_bypass_speculation_mitigation_control.change_for_current_thread().map_err(StoreBypassSpeculationMitigationControl)?;
		}
		
		if let Some(indirect_branch_speculation_mitigation_control) = indirect_branch_speculation_mitigation_control
		{
			indirect_branch_speculation_mitigation_control.change_for_current_thread().map_err(IndirectBranchSpeculationMitigationControl)?;
		}
		
		let thread_local_allocator_drop_guard = PTMAI::instantiate(thread_local_allocator_configuration, instantiation_arguments).map_err(PerThreadMemoryAllocatorInstantiation)?;
		
		if start_logging
		{
			(unsafe { LocalSyslogSocket::configure_per_thread_local_syslog_socket() }).map_err(CouldNotStartThreadLocalLogging)?;
		}
		
		let thread_identifier = ThreadIdentifier::default();
		
		{
			let pthread_t = unsafe { pthread_self() };
			affinity.set_thread_affinity(pthread_t).map_err(CouldNotSetThreadAffinity)?;
		}
		
		if let Some(nice) = nice
		{
			nice.set_thread_priority(thread_identifier).map_err(|_: ()| CouldNotSetNice)?
		}
		
		if let Some(io_priority) = io_priority
		{
			io_priority.set_for_thread(thread_identifier).map_err(CouldNotSetIoPriority)?
		}
		
		thread_scheduler.set_for_thread(ThreadIdentifierChoice::Other(thread_identifier)).map_err(CouldNotSetSchedulerPolicyAndFlags)?;
		
		Ok(thread_local_allocator_drop_guard)
	}
	
	#[inline(always)]
	fn configure_thread_capabilities_after_initializing_thread_loop_body_function(capabilities: Option<Arc<ThreadCapabilitiesConfiguration>>) -> Result<(), ThreadConfigurationError>
	{
		if let Some(capabilities) = capabilities
		{
			capabilities.configure_if_wanted()?
		}
		else
		{
			ThreadCapabilitiesConfiguration::configure_if_unwanted()?
		}
		Ok(())
	}
	
	#[inline(always)]
	fn builder(&self) -> Builder
	{
		Builder::new().name(self.name()).stack_size(self.stack_size())
	}
	
	#[inline(always)]
	fn name(&self) -> String
	{
		self.name.to_string()
	}
	
	#[inline(always)]
	fn stack_size(&self) -> usize
	{
		(self.stack_size.get() * PageSize::default().size_in_bytes().get()) as usize
	}
	
	#[inline(always)]
	fn stack_size_default() -> NonZeroNumberOfPages
	{
		const _2MB: u64 = 2_048 * 1_024;
		new_non_zero_u64(_2MB / PageSize::default().size_in_bytes().get())
	}
}

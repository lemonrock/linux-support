// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process configuration.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessConfiguration
{
	/// Inclusive minimum.
	pub minimum_linux_kernel_version: LinuxKernelVersionNumber,

	/// Change global (root) settings to maximize use of machine resources and security.
	#[serde(default)] pub global: Option<GlobalConfiguration>,

	/// Mandatory CPU feature checks to suppress.
	#[serde(default)] pub mandatory_cpu_feature_checks_to_suppress: HashSet<MandatoryCpuFeatureCheck>,

	/// Optional CPU feature checks to suppress.
	#[serde(default = "ProcessConfiguration::optional_cpu_feature_checks_to_suppress_default")] pub optional_cpu_feature_checks_to_suppress: HashSet<OptionalCpuFeatureCheck>,

	/// Process name.
	#[serde(default)] pub name: ProcessName,

	/// Locale.
	#[serde(default)] pub locale: LocaleName,

	/// Permissions bit mask for new files.
	#[serde(default)] pub umask: AccessPermissions,

	/// Resource limits.
	#[serde(default = "ProcessConfiguration::resource_limits_default")] pub resource_limits: ResourceLimitsSet,

	/// Pevent all memory in the process' virtual address space from being swapped.
	///
	/// Memory locking has two main applications: real-time algorithms and high-security data processing.
	/// Real-time applications require deterministic timing, and, like scheduling, paging is one major cause of unexpected program execution delays.
	///
	/// Memory locks are not inherited by a child created via `fork()` and are automatically removed (unlocked) during an `execve()`.
	#[serde(default)] pub lock_all_memory: Option<LockAllMemory>,

	/// Process HyperThread affinity.
	#[serde(default)] pub affinity: Option<BitSet<HyperThread>>,

	/// Process scheduling.
	#[serde(default)] pub process_scheduling_configuration: ProcessSchedulingConfiguration,

	/// Mostly for things like block device daemons and FUSE daemons.
	///
	/// Since Linux 5.6.
	#[serde(default)] pub enable_io_flusher: Option<bool>,

	/// Logging configuration.
	#[serde(default)] pub logging_configuration: ProcessLoggingConfiguration,

	/// User and group settings.
	#[serde(default)] pub user_and_group_settings: UserAndGroupSettings,

	/// Paths to use for `PATH`.
	#[serde(default = "ProcessConfiguration::binary_paths_default")] pub binary_paths: BTreeSet<PathBuf>,

	/// The folder path to use as the 'current working directory' (CWD).
	///
	/// Equivalent functionality to the shell command `chdir`.
	///
	/// Defaults to `/`, which is appropriate for a daemon to allow for unmounts to happen.
	#[serde(default = "ProcessConfiguration::working_directory_default")] pub working_directory: PathBuf,

	/// Seccomp configuration.
	///
	/// SecComp filtering adds a 5% to 10% overhead.
	#[serde(default)] pub seccomp: Option<PermittedSyscalls>,

	#[serde(default)] pub capabilities: Option<ProcessCapabilitiesConfiguration>,

	/// Main thread configuration.
	#[serde(default)] pub main_thread_configuration: ThreadConfiguration,

	/// Main thread sleeps for this long before checking for events.
	///
	/// Default is 1ms.
	#[serde(default = "ProcessConfiguration::main_thread_sleep_default")] pub main_thread_sleep: Duration,
}

impl ProcessConfiguration
{
	/// Configure, then run the process.
	#[inline(always)]
	pub fn configure_and_run_assume_linux_standard_base(&self, run_as_daemon: bool, terminate: &Arc<impl Terminate>) -> Result<(), ProcessConfigurationError>
	{
		let sys_path = SysPath::default();
		let proc_path = ProcPath::default();
		let dev_path = DevPath::default();
		let etc_path = EtcPath::default();
		self.configure_then_run(&sys_path, &proc_path, &dev_path, &etc_path, run_as_daemon, terminate)
	}

	/// Configure, then run the process.
	#[inline(always)]
	pub fn configure_then_run(&self, sys_path: &SysPath, proc_path: &ProcPath, dev_path: &DevPath, etc_path: &EtcPath, run_as_daemon: bool, terminate: &Arc<impl Terminate>) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		// This *MUST* be called before daemonizing.
		Self::block_all_signals_on_current_thread();

		// This *MUST* be called before daemonizing.
		reset_all_signal_handlers_to_default();

		Self::validate_not_running_setuid_or_setgid()?;

		Self::protect_access_to_proc_self_and_disable_core_dumps()?;

		self.cpu_feature_checks()?;

		Self::validate_current_personality(proc_path)?;

		self.validate_linux_kernel_version_is_recent_enough(proc_path)?;

		// This *MUST* be called before daemonizing.
		close_all_open_file_descriptors_apart_from_standard(proc_path).map_err(CouldNotCloseAllOpenFileDescriptorsApartFromStandard)?;

		// This *MUST* be called before creating PID files (eg when daemonizing) or log files not managed by syslog.
		self.umask.set_umask();

		self.set_global_configuration(sys_path, proc_path)?;

		set_current_dir(&self.working_directory).map_err(CouldNotChangeWorkingDirectory)?;

		// This *SHOULD* be called before enabling logging.
		self.set_locale()?;

		if run_as_daemon
		{
			daemonize(dev_path)
		}

		// This *SHOULD* be called before enabling logging.
		self.set_process_name(proc_path)?;

		// This *COULD* be called before daemonizing.
		// This *MUST* be called before locking memory.
		// This *MUST* be called before creating new threads.
		// This *MUST* be called before changing process scheduling.
		// This *MUST* be called opening file descriptors.
		self.resource_limits.change().map_err(CouldNotChangeResourceLimit)?;

		// This *MUST* be called after changing resource limits.
		self.lock_all_memory();

		// This *MUST* be called before creating new threads.
		self.configure_process_affinity(proc_path)?;

		// This *MUST* be called before creating new threads.
		self.process_scheduling_configuration.configure(proc_path)?;

		self.set_io_flusher()?;

		// This *SHOULD* be configured before configuring logging.
		// This *MUST* be called before `configure_global_panic_hook()` which uses backtraces depedant on environment variable settings.
		self.set_environment_variables_to_minimum_required(etc_path)?;

		// This *MUST* be called before `configure_global_panic_hook()`.
		self.logging_configuration.start_logging(!run_as_daemon, &self.name);
		configure_global_panic_hook(terminate);

		// This *MUST* be called before creating new threads.
		// This *MUST* be called before dropping root.
		if let Some(capabilities) = self.capabilities.as_ref()
		{
			capabilities.configure_if_wanted()?
		}
		else
		{
			ProcessCapabilitiesConfiguration::configure_if_unwanted()?
		}

		// This *MUST* be called before creating new threads.
		Self::secure_io_ports();

		// This *MUST* be called before creating new threads.
		// This *MUST* be called before executing programs that might be setuid/setgid or have file capabilities.
		// This prevents `execve()` granting additional capabilities.
		no_new_privileges().map_err(CouldNotPreventTheGrantingOfNoNewPrivileges)?;

		self.threads_exist_from_now_on(thread_configurations, terminate, proc_path)
	}

	#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))]
	fn secure_io_ports()
	{
		remove_ioport_permissions();
		remove_ioport_privileges();
	}

	fn threads_exist_from_now_on(&self, thread_configurations: &[(ThreadConfiguration, XXX)], terminate: &Arc<impl Terminate>, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		let (join_handles, result) = JoinHandles::main_thread_spawn_configured_child_threads(thread_configurations, terminate, proc_path);
		if let Err(error) = result
		{
			terminate.clone().begin_termination();
			join_handles.release();
			return Err(error)?
		}

		let result = self.main_thread_configuration.configure_main_thread();
		if let Err(error) = result
		{
			terminate.clone().begin_termination();
			join_handles.release();
			return Err(error)?
		}

		// TODO: Let threads initialize before applying seccomp (eg to open sockets, etc).
		// TODO: Then strip capabilities and set up syscalls.
		// TODO: A NUMA-aware malloc.

		self.seccomp_for_all_threads()?;

		// Capabilities are dropped when an user transitions to a non-root user unless we set the securebit to keep capabilities.
		self.user_and_group_settings.change_user_and_groups(etc_path)?;

		Self::protect_access_to_proc_self_and_disable_core_dumps()?;

		join_handles.release();

		let terminate = terminate.clone();
		while terminate.should_continue()
		{
			// TODO: Enter the main loop; need to run a signal handler.
			sleep(self.main_thread_sleep)
		}

		if terminate.terminated_due_to_panic_or_irrecoverable_error()
		{
			Err(ProcessConfigurationError::TerminatedDueToPanicOrIrrecoverableError)
		}
		else
		{
			Ok(())
		}
	}

	/// This is a security defence to prevent propagation of unknown environment variables to potential child processes.
	#[inline(always)]
	fn set_environment_variables_to_minimum_required(&self, etc_path: &EtcPath) -> Result<(), JoinPathsError>
	{
		populate_clean_environment(&self.binary_paths, UserIdentifier::current_real().user_name_home_directory_and_shell(etc_path))
	}

	// This needs to be called after any changes to the process' user identifiers: the process' dumpable bit is reset after the effective user changes.
	#[inline(always)]
	fn protect_access_to_proc_self_and_disable_core_dumps() -> Result<(), ProcessConfigurationError>
	{
		disable_dumpable().map_err(ProcessConfigurationError::CouldNotDisableDumpable)
	}

	#[inline(always)]
	fn set_process_name(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		self.name.set_process_name(ProcessIdentifierChoice::Current, proc_path).map_err(ProcessConfigurationError::CouldNotSetProcessName)
	}

	#[inline(always)]
	fn configure_process_affinity(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		set_value(proc_path, |_proc_path, value| value.set_current_process_affinity(), self.affinity.as_ref(), ProcessConfigurationError::CouldNotChangeProcessAffinity)
	}

	#[inline(always)]
	fn set_locale(&self) -> Result<LocaleName, ProcessConfigurationError>
	{
		self.locale.set_all().map_err(|_: ()| ProcessConfigurationError::CouldNotSetLocale(self.locale.clone()))
	}

	#[inline(always)]
	fn lock_all_memory(&self)
	{
		if let Some(lock_all_memory) = self.lock_all_memory
		{
			lock_all_memory.lock()
		}
	}

	#[inline(always)]
	fn set_global_configuration(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		set_value(proc_path, |proc_path, global| global.configure(sys_path, proc_path), self.global.as_ref(), ProcessConfigurationError::CouldNotChangeGlobalConfiguration)
	}

	#[inline(always)]
	fn set_io_flusher(&self) -> Result<(), ProcessConfigurationError>
	{
		if let Some(enable_io_flusher) = self.enable_io_flusher
		{
			set_io_flusher(enable_io_flusher).map_err(ProcessConfigurationError::IoFlusher)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn block_all_signals_on_current_thread()
	{
		BitSet::<Signal>::block_all_signals_on_current_thread();
	}

	#[inline(always)]
	fn validate_not_running_setuid_or_setgid() -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		if unlikely!(UserIdentifier::running_setuid())
		{
			return Err(RunningSetUid)
		}

		if unlikely!(GroupIdentifier::running_setuid())
		{
			return Err(RunningSetGid)
		}

		Ok(())
	}

	#[inline(always)]
	fn validate_current_personality(proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		PersonalityFlags::validate_only_one_execution_domain(proc_path);
		let current_personality = PersonalityFlags::current().map_err(|_ :()| CouldNotObtainPersonality)?;
		if likely!(current_personality.is_standard_linux())
		{
			Ok(())
		}
		else
		{
			Err(CurrentPersonalityIsNotLinux(current_personality))
		}
	}

	#[inline(always)]
	fn validate_linux_kernel_version_is_recent_enough(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;
		if LinuxKernelVersion::parse(proc_path).map_err(CouldNotParseLinuxKernelVersion)?.major_minor_revision() < self.minimum_linux_kernel_version
		{
			Err(LinuxKernelVersionIsTooOld)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn seccomp_for_all_threads(&self) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		if let Some(ref seccomp) = self.seccomp
		{
			seccomp.seccomp_program().load_and_synchronize_all_threads(true, false).map_err(|cause| CouldNotLoadSeccompFilters(cause))?.map_err(|thread_identifier| CouldNotSynchronizeSeccompFiltersOnThread(thread_identifier))
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn cpu_feature_checks(&self) -> Result<(), ProcessConfigurationError>
	{
		let check_arguments;
		#[cfg(target_arch = "x86_64")]
		{
			let cpu_id = CpuId::new();
			check_arguments =
			(
				cpu_id.get_feature_info().unwrap(),
				cpu_id.get_extended_function_info().unwrap(),
				cpu_id.get_extended_feature_info().unwrap(),
			);
		}
		#[cfg(not(target_arch = "x86_64"))]
		{
			check_arguments = ();
		}

		use self::ProcessConfigurationError::*;

		let empty: HashSet<CompiledCpuFeatureCheck> = HashSet::new();
		Self::cpu_feature_check(&empty, &check_arguments, CompiledCpuFeatureChecksFailed)?;
		Self::cpu_feature_check(&self.mandatory_cpu_feature_checks_to_suppress, &check_arguments, MandatoryCpuFeatureChecksFailed)?;
		Self::cpu_feature_check(&self.optional_cpu_feature_checks_to_suppress, &check_arguments, OptionalCpuFeatureChecksFailed)
	}

	#[inline(always)]
	fn cpu_feature_check<C: Check>(cpu_feature_checks_to_suppress: &HashSet<C>, check_arguments: &C::CheckArguments, error: impl FnOnce(FailedChecks<C>) -> ProcessConfigurationError) -> Result<(), ProcessConfigurationError>
	{
		C::run_all_checks(cpu_feature_checks_to_suppress, check_arguments).map_err(error)
	}

	#[inline(always)]
	fn optional_cpu_feature_checks_to_suppress_default() -> HashSet<OptionalCpuFeatureCheck>
	{
		#[cfg(target_arch = "x86_64")] use self::OptionalCpuFeatureCheck::*;
		#[cfg(target_arch = "x86_64")] return hashset!
		{
			has_rep_movsb_stosb,
			has_prefetchw,
			has_ss,
			has_working_xsave,
			has_tsc_adjust_msr,
			has_invpcid,
			has_smap,
		};

		#[cfg(not(target_arch = "x86_64"))] return hashset!
		{
		};
	}

	#[inline(always)]
	fn resource_limits_default() -> ResourceLimitsSet
	{
		ResourceLimitsSet::defaultish(ResourceLimit::maximum_number_of_open_file_descriptors(&ProcPath::default()).expect("Could not read maximum number of file descriptors"))
	}

	#[inline(always)]
	fn binary_paths_default() -> BTreeSet<PathBuf>
	{
		btreeset!
		{
			PathBuf::from("/usr/bin"),
			PathBuf::from("/bin"),
		}
	}

	#[inline(always)]
	fn working_directory_default() -> PathBuf
	{
		PathBuf::from("/")
	}

	#[inline(always)]
	fn main_thread_sleep_default() -> Duration
	{
		Duration::new(0, 1_000_000)
	}
}

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

	/// Capabilities to retain in the process before opening network sockets.
	/// This allows an application to, say, use `CAP_SYS_NICE` and open raw I/O network sockets without running the application with root-like vulnerabilities or require file capabilities.
	#[serde(default)] pub initial_capabilities: Option<ThreadCapabilitiesConfiguration>,

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

	/// Process nice.
	#[serde(default)] pub process_nice_configuration: Option<ProcessNiceConfiguration>,

	/// IO Priority (ionice / ioprio).
	#[serde(default)] pub process_io_priority_configuration: Option<ProcessIoPriorityConfiguration>,

	/// Mostly for things like block device daemons and FUSE daemons.
	///
	/// Since Linux 5.6.
	#[serde(default)] pub enable_io_flusher: Option<bool>,

	/// Logging configuration.
	#[serde(default)] pub logging_configuration: ProcessLoggingConfiguration,

	/// Paths to use for `PATH`.
	#[serde(default = "ProcessConfiguration::binary_paths_default")] pub binary_paths: BTreeSet<PathBuf>,

	/// The folder path to use as the 'current working directory' (CWD).
	///
	/// Equivalent functionality to the shell command `chdir`.
	///
	/// Defaults to `/`, which is appropriate for a daemon to allow for unmounts to happen.
	#[serde(default = "ProcessConfiguration::working_directory_default")] pub working_directory: PathBuf,
}

impl ProcessConfiguration
{
	/// Configure.
	///
	/// Assumes a file hierarchy standard (FHS) file system layout of `/sys`, `/proc`, `/dev` and `/etc`
	///
	/// Use `ProcessExecutor::execute_securely()` after this.
	/// Until this is used, the returned `SimpleTerminate` does not affect any thread behaviour.
	#[inline(always)]
	pub fn configure_assuming_file_hierarchy_standard_file_system_layout(&self, run_as_daemon: bool) -> Result<Arc<impl Terminate>, ProcessConfigurationError>
	{
		let sys_path = SysPath::default();
		let proc_path = ProcPath::default();
		let dev_path = DevPath::default();
		let etc_path = EtcPath::default();
		self.configure(run_as_daemon, &sys_path, &proc_path, &dev_path, &etc_path)
	}

	/// Configure.
	///
	/// Use `ProcessExecutor::execute_securely()` after this.
	/// Until this is used, the returned `SimpleTerminate` does not affect any thread behaviour.
	#[inline(always)]
	pub fn configure(&self, run_as_daemon: bool, sys_path: &SysPath, proc_path: &ProcPath, dev_path: &DevPath, etc_path: &EtcPath) -> Result<Arc<impl Terminate>, ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		// This *MUST* be called before daemonizing.
		Self::block_all_signals_on_current_thread();

		// This *MUST* be called before daemonizing.
		reset_all_signal_handlers_to_default();

		Self::validate_not_running_setuid_or_setgid()?;

		Self::protect_access_to_proc_self_and_disable_core_dumps()?;

		self.cpu_feature_checks()?;

		// This *MUST* be called before `set_global_configuration()` otherwise we will try to configure files in `/proc` that don't exist.
		self.validate_linux_kernel_version_is_recent_enough(proc_path)?;

		// This *MUST* be called after `validate_linux_kernel_version_is_recent_enough()`.
		self.set_global_configuration(sys_path, proc_path)?;

		// This *SHOULD* be called as soon as possible so that when threads open network sockets, say, we are already running with as few capabilities as possible.
		// This *SHOULD* be called after `set_global_configuration()` so that the former can load Linux kernel modules if needed.
		self.reduce_initial_capabilities_to_minimum_set()?;

		Self::validate_current_personality(proc_path)?;

		// This *MUST* be called before daemonizing.
		close_all_open_file_descriptors_apart_from_standard(proc_path).map_err(CouldNotCloseAllOpenFileDescriptorsApartFromStandard)?;

		// This *MUST* be called before creating PID files (eg when daemonizing) or log files not managed by syslog.
		self.umask.set_umask();

		set_current_dir(&self.working_directory).map_err(CouldNotChangeWorkingDirectory)?;

		// This *SHOULD* be called before enabling logging.
		// This *SHOULD* be called before using any libc functions that format strings.
		self.set_locale()?;

		// This *SHOULD* be called before enabling logging.
		self.set_process_name(proc_path)?;

		// This *SHOULD* be called before daemonizing (forking).
		// This *MUST* be called before locking memory.
		// This *MUST* be called before creating new threads.
		// This *MUST* be called before changing process scheduling.
		// This *MUST* be called opening any significant number of file descriptors.
		self.resource_limits.change().map_err(CouldNotChangeResourceLimit)?;

		// This *SHOULD* be configured before configuring logging.
		// This *MUST* be called before `configure_global_panic_hook()` which uses backtraces depedant on environment variable settings.
		self.set_environment_variables_to_minimum_required_and_force_time_zone_to_utc(etc_path)?;

		if run_as_daemon
		{
			daemonize(dev_path)
		}

		// This *MUST* be called before creating `terminate`.
		self.logging_configuration.start_logging(!run_as_daemon, &self.name);
		let terminate = SimpleTerminate::new(ParsedPanicErrorLoggerProcessLoggingConfiguration);

		configure_global_panic_hook(&terminate);

		// This *MUST* be called after changing resource limits.
		// This *MUST* be called after daemonizing (forking) s memory locks aren't preserved across a fork.
		self.lock_all_memory();

		// This *MUST* be called before creating new threads.
		self.configure_process_affinity(proc_path)?;

		// This *MUST* be called before creating new threads.
		set_value(proc_path, |proc_path, process_nice_configuration| process_nice_configuration.configure(proc_path),self.process_nice_configuration.as_ref(), ProcessNiceConfiguration)?;

		// This *MUST* be called before creating new threads.
		set_value(proc_path, |_, process_io_priority_configuration| process_io_priority_configuration.configure(), self.process_io_priority_configuration.as_ref(), ProcessIoPriorityConfiguration)?;

		self.set_io_flusher()?;

		// This *MUST* be called after daemonizing (forking).
		// This *MUST* be called before creating new threads.
		#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))] Self::secure_io_ports();

		// This *MUST* be called before creating new threads.
		// This *MUST* be called before loading Seccomp filters for uprivileged processes.
		// This *MUST* be called before executing programs that might be setuid/setgid or have file capabilities.
		// This prevents `execve()` granting additional capabilities.
		no_new_privileges().map_err(CouldNotPreventTheGrantingOfNoNewPrivileges)?;

		Ok(terminate)
	}

	#[inline(always)]
	fn reduce_initial_capabilities_to_minimum_set(&self) -> Result<(), ProcessConfigurationError>
	{
		if let Some(ref initial_capabilities) = self.initial_capabilities.as_ref()
		{
			initial_capabilities.configure_just_capabilities().map_err(ProcessConfigurationError::CouldNotChangeInitialCapabilities)
		}
		else
		{
			Ok(())
		}
	}

	#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))]
	#[inline(always)]
	fn secure_io_ports()
	{
		remove_ioport_permissions();
		remove_ioport_privileges();
	}

	/// This is a security defence to prevent propagation of unknown environment variables to potential child processes.
	///
	/// This also correctly sets:-
	///
	/// * rust backtrace logging;
	/// * ensures that the UTC time zone exists and is readable;
	/// * ensures that the libc time zone global static fields are correctly set (not doing this was a bug in rsyslog, for instance).
	#[inline(always)]
	fn set_environment_variables_to_minimum_required_and_force_time_zone_to_utc(&self, etc_path: &EtcPath) -> Result<(), ProcessConfigurationError>
	{
		let utc_file_path = etc_path.zoneinfo("UTC");
		utc_file_path.read_raw().map_err(ProcessConfigurationError::UtcFilePathDoesNotExistOrIsNotReadable)?;

		populate_clean_environment(&self.binary_paths, UserIdentifier::current_real().user_name_home_directory_and_shell(etc_path), utc_file_path)?;

		unsafe { tzset() };

		Ok(())
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
}

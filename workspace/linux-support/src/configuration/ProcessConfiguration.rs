// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process configuration.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessConfiguration
{
	/// Inclusive minimum.
	#[serde(default ="ProcessConfiguration::minimum_linux_kernel_version_default" )] pub minimum_linux_kernel_version: LinuxKernelVersionNumber,

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

	/// Legacy adjustment to out-of-memory score.
	///
	/// Use `OutOfMemoryAdjustment::Exempt` to disable Out-of-Memory killing.
	///
	/// Making less likely or disabling requires root.
	#[serde(default)] pub out_of_memory_adjustment: Option<OutOfMemoryAdjustment>,

	/// Modern adjustment to out-of-memory score.
	///
	/// Use `OutOfMemoryScoreAdjustment::LessLikely(OutOfMemoryScoreAdjustmentValue::InclusiveMaximum))` to disable Out-of-Memory killing.
	///
	/// Making less likely or disabling requires root.
	#[serde(default)] pub out_of_memory_score_adjustment: Option<OutOfMemoryScoreAdjustment>,
	
	/// Synchronize (flush all data and meta data to disks) and drop caches, once, on start-up.
	///
	/// This can help later memory large allocations to succeed in conjunction with `compact_memory`.
	///
	/// See also `GlobalMemoryConfiguration.compact_unevictable_allowed`.
	///
	/// Requires root if `true`.
	#[serde(default)] pub synchronize_and_drop_caches: bool,
	
	/// Compact memory, once, on start-up.
	///
	/// This can help later memory large allocations to succeed in conjunction with `synchronize_and_drop_caches`.
	///
	/// See also `GlobalMemoryConfiguration.compact_unevictable_allowed`.
	///
	/// Requires root if `true`.
	#[serde(default)] pub compact_memory: bool,
	
	/// Prevent all memory in the process' virtual address space from being swapped.
	///
	/// Memory locking has two main applications: real-time algorithms and high-security data processing.
	/// Real-time applications require deterministic timing, and, like scheduling, paging is one major cause of unexpected program execution delays.
	///
	/// Memory locks are not inherited by a child created via `fork()` and are automatically removed (unlocked) during an `execve()`.
	#[serde(default)] pub lock_all_memory: Option<LockAllMemory>,

	/// Process nice.
	#[serde(default)] pub process_nice_configuration: Option<ProcessNiceConfiguration>,

	/// IO Priority (ionice / ioprio).
	#[serde(default)] pub process_io_priority_configuration: Option<ProcessIoPriorityConfiguration>,

	/// Mostly for things like block device daemons and FUSE daemons.
	///
	/// Since Linux 5.6.
	#[serde(default)] pub enable_or_disable_io_flusher: Option<bool>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub machine_check_exception_kill_policy: Option<Option<MachineCheckExceptionKillPolicy>>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub timestamp_counter_setting: Option<TimestampCounterSetting>,
	
	#[allow(missing_docs)]
	#[serde(default)] pub enable_or_disable_process_performance_counters: Option<bool>,

	/// Logging configuration.
	#[serde(default)] pub logging_configuration: ProcessLoggingConfiguration,
	
	/// When a panic occurs that isn't caught try to capture a full stack back trace.
	#[serde(default = "ProcessConfiguration::enable_full_rust_stack_back_traces_default")] pub enable_full_rust_stack_back_traces: bool,
	
	/// Paths to use for `PATH`.
	#[serde(default = "ProcessConfiguration::binary_paths_default")] pub binary_paths: BTreeSet<PathBuf>,

	/// The folder path to use as the 'current working directory' (CWD).
	///
	/// Equivalent functionality to the shell command `chdir`.
	///
	/// Defaults to `/`, which is appropriate for a daemon to allow for unmounts to happen.
	#[serde(default = "ProcessConfiguration::working_directory_default")] pub working_directory: PathBuf,

	/// Flags to restrict data emitted during core dumps.
	///
	/// Defaults to `empty()`, which *does not* prevent core dumps but does restrict the data dumped.
	#[serde(default = "ProcessConfiguration::core_dump_filter_default")] pub core_dump_filter: CoreDumpFilterFlags,
}

impl Default for ProcessConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			minimum_linux_kernel_version: Self::minimum_linux_kernel_version_default(),
			global: None,
			mandatory_cpu_feature_checks_to_suppress: Default::default(),
			optional_cpu_feature_checks_to_suppress: Self::optional_cpu_feature_checks_to_suppress_default(),
			name: Default::default(),
			locale: Default::default(),
			umask: Default::default(),
			initial_capabilities: None,
			resource_limits: Self::resource_limits_default(),
			out_of_memory_adjustment: None,
			out_of_memory_score_adjustment: None,
			synchronize_and_drop_caches: false,
			compact_memory: false,
			lock_all_memory: None,
			process_nice_configuration: None,
			process_io_priority_configuration: None,
			enable_or_disable_io_flusher: None,
			machine_check_exception_kill_policy: None,
			timestamp_counter_setting: None,
			enable_or_disable_process_performance_counters: None,
			logging_configuration: Default::default(),
			enable_full_rust_stack_back_traces: Self::enable_full_rust_stack_back_traces_default(),
			binary_paths: Self::binary_paths_default(),
			working_directory: Self::working_directory_default(),
			core_dump_filter: Self::core_dump_filter_default()
		}
	}
}

impl ProcessConfiguration
{
	/// Configure.
	///
	/// Use `ProcessExecutor::execute_securely()` after this.
	/// Until this is used, the returned `SimpleTerminate` does not affect any thread behaviour.
	///
	/// `process_affinity` should be calculated, but, ideally, should be the isolated CPUs on the system.
	#[inline(always)]
	pub fn configure(&self, run_as_daemon: bool, file_system_layout: &FileSystemLayout, defaults: &DefaultPageSizeAndHugePageSizes, additional_logging_configuration: &mut impl AdditionalLoggingConfiguration, global_computed_scheduling_affinity: Option<&GlobalComputedSchedulingConfiguration>, process_affinity: Option<&HyperThreads>) -> Result<Arc<impl Terminate>, ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;
		
		let (sys_path, proc_path, dev_path, etc_path) = file_system_layout.paths();
		
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
		self.set_global_configuration(sys_path, proc_path, defaults)?;
		
		// This *MUST* be called after `validate_linux_kernel_version_is_recent_enough()`.
		Self::set_global_computed_configuration(sys_path, proc_path, global_computed_scheduling_affinity)?;

		// This *SHOULD* be called as soon as possible so that when threads open network sockets, say, we are already running with as few capabilities as possible.
		//	Thus is *SHOULD be called before configuring logging which *DOES* open a network connection to a log (or syslog) socket.
		// This *SHOULD* be called after `set_global_configuration()` so that the former can load Linux kernel modules if needed.
		self.reduce_initial_capabilities_to_minimum_set()?;

		self.apply_core_dump_filter(proc_path)?;
		
		Self::validate_current_personality(proc_path)?;

		// This *MUST* be called before daemonizing.
		close_all_open_file_descriptors_apart_from_standard(proc_path).map_err(CouldNotCloseAllOpenFileDescriptorsApartFromStandard)?;

		// This *MUST* be called before creating PID files (eg when daemonizing) or log files not managed by syslog.
		self.umask.set_umask();

		set_current_dir(&self.working_directory).map_err(CouldNotChangeWorkingDirectory)?;

		// This *SHOULD* be called before using any libc functions that format strings.
		self.set_locale()?;

		self.set_process_name(proc_path)?;

		// This *SHOULD* be called before daemonizing (forking).
		// This *MUST* be called before locking memory.
		// This *MUST* be called before creating new threads.
		// This *MUST* be called before changing process scheduling.
		// This *MUST* be called opening any significant number of file descriptors.
		self.resource_limits.change().map_err(CouldNotChangeResourceLimit)?;
		
		// This should be called before making large memory allocations.
		self.set_out_of_memory_adjustment(proc_path)?;
		
		self.synchronize_and_drop_caches(proc_path)?;
		
		self.compact_memory(proc_path)?;
		
		// This *SHOULD* be configured before configuring logging.
		// This *MUST* be called before `configure_global_panic_hook()` which uses backtraces depedant on environment variable settings.
		self.set_environment_variables_to_minimum_required_and_force_time_zone_to_utc(etc_path)?;
		
		// This *MUST* be called before `configure_logging` as `configure_logging` opens sockets (eg to `/dev/log`) that are closed-on-exec and refers to process identifiers (pids).
		if run_as_daemon
		{
			daemonize(dev_path)
		}
		
		// This *MUST* be called before creating `ParsedPanicErrorLoggerProcessLoggingConfiguration` and thus `SimpleTerminate`.
		self.configure_logging(dev_path, proc_path, run_as_daemon, additional_logging_configuration)?;
		
		let terminate = SimpleTerminate::new(ParsedPanicErrorLoggerProcessLoggingConfiguration);

		configure_global_panic_hook(&terminate);

		// This *MUST* be called after changing resource limits.
		// This *MUST* be called after daemonizing (forking) as memory locks aren't preserved across a fork.
		self.lock_all_memory();

		// This *MUST* be called before creating new threads.
		self.configure_process_affinity(proc_path, process_affinity)?;

		// This *MUST* be called before creating new threads.
		set_value(proc_path, |proc_path, process_nice_configuration| process_nice_configuration.configure(proc_path),self.process_nice_configuration.as_ref(), ProcessNiceConfiguration)?;

		// This *MUST* be called before creating new threads.
		set_value(proc_path, |_, process_io_priority_configuration| process_io_priority_configuration.configure(), self.process_io_priority_configuration.as_ref(), ProcessIoPriorityConfiguration)?;

		self.enable_or_disable_io_flusher()?;
		
		self.change_machine_check_exception_kill_policy().map_err(CouldNotChangeMachineCheckExceptionKillPolicy)?;
		
		self.set_timestamp_counter_setting()?;
		
		self.enable_or_disable_process_performance_counters()?;

		// This *MUST* be called after daemonizing (forking).
		// This *MUST* be called before creating new threads.
		#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))] Self::secure_io_ports();

		// This *MUST* be called before creating new threads.
		// This *MUST* be called before loading Seccomp filters for uprivileged processes.
		// This *MUST* be called before executing programs that might be setuid/setgid or have file capabilities.
		// This prevents `execve()` granting additional capabilities.
		change_no_new_privileges(true).map_err(CouldNotPreventTheGrantingOfNoNewPrivileges)?;
		
		Ok(terminate)
	}
	
	#[inline(always)]
	fn set_out_of_memory_adjustment(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		if let Some(out_of_memory_adjustment) = self.out_of_memory_adjustment
		{
			out_of_memory_adjustment.set(proc_path, ProcessIdentifierChoice::Current).map_err(ProcessConfigurationError::CouldNotChangeOutOfMemoryAdjustment)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn synchronize_and_drop_caches(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		if self.synchronize_and_drop_caches
		{
			File::synchronize_everything();
			
			assert_effective_user_id_is_root("write /proc/sys/vm/drop_caches");
			
			const DropPageCaches: u8 = 1;
			const DropSlabObjects: u8 = 2;
			const DisableInformationalLogging: u8 = 4;
			
			set_proc_sys_vm_value(proc_path, "drop_caches", Some(UnpaddedDecimalInteger(DropPageCaches | DropSlabObjects | DisableInformationalLogging)), ProcessConfigurationError::CouldNotDropCaches)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn compact_memory(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		if self.compact_memory
		{
			assert_effective_user_id_is_root("write /proc/sys/vm/compact_memory");
			
			set_proc_sys_vm_value(proc_path, "compact_memory", Some(true), ProcessConfigurationError::CouldNotCompactMemory)
		}
		else
		{
			Ok(())
		}
	}
	
	fn configure_logging(&self, dev_path: &DevPath, proc_path: &ProcPath, run_as_daemon: bool, additional_logging_configuration: &mut impl AdditionalLoggingConfiguration) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;
		
		let internet_protocol_addresses = Self::get_internet_protocol_addresses_using_netlink().map_err(|cause| CouldNotGetInternetProtocolAddressesUsingNetlink(cause))?;
		let host_name = LinuxKernelHostName::new(proc_path).map_err(CouldNotParseLinuxKernelHostName)?;
		let domain_name = LinuxKernelDomainName::new(proc_path).map_err(CouldNotParseLinuxKernelDomainName)?;
		self.logging_configuration.configure_logging(additional_logging_configuration, dev_path, !run_as_daemon, &internet_protocol_addresses, host_name.as_ref(), domain_name.as_ref(), &self.name)?;
		
		if run_as_daemon
		{
			self.logging_configuration.redirect_FILE_standard_out_and_file_standard_error_to_log(host_name.as_ref(), &self.name);
		}
		
		Ok(())
	}
	
	fn get_internet_protocol_addresses_using_netlink() -> Result<Vec<IpAddr>, String>
	{
		let mut internet_protocol_addresses = Vec::new();
		
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(|cause| cause.to_string())?;
		
		for entry in RouteNetlinkProtocol::get_internet_protocol_version_4_addresses(&mut netlink_socket_file_descriptor)?
		{
			match entry.local_address_and_destination_address_for_point_to_point()
			{
				None => (),
				Some((Left(local_address), _destination_address_for_point_to_point)) => internet_protocol_addresses.push((*local_address).into()),
				Some((Right(local_address), _destination_address_for_point_to_point)) => internet_protocol_addresses.push(local_address.internet_protocol_address_moved().into()),
			}
		}
		
		for entry in RouteNetlinkProtocol::get_internet_protocol_version_6_addresses(&mut netlink_socket_file_descriptor)?
		{
			match entry.local_address_and_destination_address_for_point_to_point()
			{
				None => (),
				Some((Left(local_address), _destination_address_for_point_to_point)) => internet_protocol_addresses.push((*local_address).into()),
				Some((Right(local_address), _destination_address_for_point_to_point)) => internet_protocol_addresses.push(local_address.internet_protocol_address_moved().into()),
			}
		}
		
		Ok(internet_protocol_addresses)
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
	
	#[inline(always)]
	fn apply_core_dump_filter(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		self.core_dump_filter.change_core_dump_filter(proc_path, ProcessIdentifierChoice::Current).map_err(ProcessConfigurationError::CouldNotChangeCoredumpFilter)
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

		populate_clean_environment(self.enable_full_rust_stack_back_traces, &self.binary_paths, UserIdentifier::current_real().user_name_home_directory_and_shell(etc_path), utc_file_path)?;

		unsafe { tzset() };

		Ok(())
	}

	// This needs to be called after any changes to the process' user identifiers: the process' dumpable bit is reset after the effective user changes.
	#[inline(always)]
	fn protect_access_to_proc_self_and_disable_core_dumps() -> Result<(), ProcessConfigurationError>
	{
		change_dumpable(false).map_err(ProcessConfigurationError::CouldNotDisableDumpable)
	}

	#[inline(always)]
	fn set_process_name(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		self.name.set_process_name(ProcessIdentifierChoice::Current, proc_path).map_err(ProcessConfigurationError::CouldNotSetProcessName)
	}

	#[inline(always)]
	fn configure_process_affinity(&self, proc_path: &ProcPath, process_affinity: Option<&HyperThreads>) -> Result<(), ProcessConfigurationError>
	{
		set_value(proc_path, |_proc_path, value| value.set_current_process_affinity(), process_affinity, ProcessConfigurationError::CouldNotChangeProcessAffinity)
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
	fn set_global_configuration(&self, sys_path: &SysPath, proc_path: &ProcPath, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(), ProcessConfigurationError>
	{
		set_value(proc_path, |proc_path, global| global.configure(sys_path, proc_path, defaults), self.global.as_ref(), ProcessConfigurationError::CouldNotChangeGlobalConfiguration)
	}

	#[inline(always)]
	fn set_global_computed_configuration(sys_path: &SysPath, proc_path: &ProcPath, global_computed_scheduling_affinity: Option<&GlobalComputedSchedulingConfiguration>) -> Result<(), ProcessConfigurationError>
	{
		set_value(proc_path, |proc_path, global_computed_scheduling_affinity| global_computed_scheduling_affinity.configure(sys_path, proc_path), global_computed_scheduling_affinity, ProcessConfigurationError::CouldNotChangeGlobalComputedSchedulingConfiguration)
	}

	#[inline(always)]
	fn enable_or_disable_io_flusher(&self) -> Result<(), ProcessConfigurationError>
	{
		if let Some(enable_or_disable_io_flusher) = self.enable_or_disable_io_flusher
		{
			change_io_flusher(enable_or_disable_io_flusher).map_err(ProcessConfigurationError::CouldNotEnableOrDisableIoFlusher)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn change_machine_check_exception_kill_policy(&self) -> io::Result<()>
	{
		if let Some(machine_check_exception_kill_policy) = self.machine_check_exception_kill_policy
		{
			match machine_check_exception_kill_policy
			{
				None => MachineCheckExceptionKillPolicy::clear_for_current_thread(),
				Some(machine_check_exception_kill_policy) => machine_check_exception_kill_policy.set_for_current_thread(),
			}
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn set_timestamp_counter_setting(&self) -> Result<(), ProcessConfigurationError>
	{
		if let Some(timestamp_counter_setting) = self.timestamp_counter_setting
		{
			timestamp_counter_setting.set().map_err(ProcessConfigurationError::CouldNotSetTimestampCounterSetting)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn enable_or_disable_process_performance_counters(&self) -> Result<(), ProcessConfigurationError>
	{
		if let Some(enable_or_disable_process_performance_counters) = self.enable_or_disable_process_performance_counters
		{
			change_process_performance_counters(enable_or_disable_process_performance_counters).map_err(ProcessConfigurationError::CouldNotEnableOrDisableProcessPerformanceCounters)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn block_all_signals_on_current_thread()
	{
		Signals::block_all_signals_on_current_thread();
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
	const fn minimum_linux_kernel_version_default() -> LinuxKernelVersionNumber
	{
		LinuxKernelVersionNumber::MinimumForIoUringSupport
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
	const fn enable_full_rust_stack_back_traces_default() -> bool
	{
		true
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
	fn core_dump_filter_default() -> CoreDumpFilterFlags
	{
		CoreDumpFilterFlags::empty()
	}
}

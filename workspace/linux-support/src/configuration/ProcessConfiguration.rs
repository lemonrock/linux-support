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

	/// Process scheduling.
	#[serde(default)] pub process_scheduling_configuration: ProcessSchedulingConfiguration,

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
	#[cfg(feature = "seccomp")] #[serde(default)] pub seccomp: SeccompConfiguration,

	/// Whitelist of capabilities to retain.
	///
	/// eg:-
	/// * `SystemAdministration`.
	/// * `LockMemory`.
	/// * `BindPortsBelow1024`.
	/// * `SetUid`.
	/// * `SetGid`.
	/// * `Nice`.
	#[serde(default)] pub capabilities_to_retain: HashSet<Capability>,
}

impl ProcessConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath, running_interactively_so_also_log_to_standard_error: bool) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		Self::block_all_signals_on_current_thread();

		Self::validate_not_running_setuid_or_setgid()?;

		Self::validate_current_personality(proc_path)?;

		self.cpu_feature_checks()?;

		if LinuxKernelVersion::parse(proc_path).map_err(|cause| CouldNotParseLinuxKernelVersion(cause))?.major_minor_revision() < self.minimum_linux_kernel_version
		{
			return Err(LinuxKernelVersionIsTooOld)
		}

		self.name.set_process_name(ProcessIdentifierChoice::Current, proc_path).map_err(|cause| CouldNotSetProcessName(cause))?;

		self.locale.set_all().map_err(|_: ()| CouldNotSetLocale(self.locale.clone()))?;

		self.umask.set_umask();

		self.resource_limits.change().map_err(|cause| CouldNotChangeResourceLimit(cause));

		self.process_scheduling_configuration.configure(proc_path)?;

		self.logging_configuration.start_logging(running_interactively_so_also_log_to_standard_error, &self.name);

		self.user_and_group_settings.change_user_and_groups(etc_path)?;

		populate_clean_environment(&self.binary_paths, UserIdentifier::current_real().user_name_home_directory_and_shell(etc_path))?;

		set_current_dir(&self.working_directory).map_err(|cause| CouldNotChangeWorkingDirectory(cause))?;

		// TODO:
			// prctl per thread
		// capabilities per thread
		// drop capabilities at start
		// store thread configuration, start all threads at start, set signal handlers for them, get them to pause before continuing.

		// TODO: Simple seccomp filters: https://outflux.net/teach-seccomp/

		// TODO: Get my head round capabilities.

		// TODO: Revised daemon code: https://chaoticlab.io/c/c++/unix/2018/10/01/daemonize.html




		#[cfg(feature = "seccomp")] self.seccomp.load().map_err(|_: ()| CouldNotLoadSeccompFilters)?;

		disable_dumpable();



		// TODO: Minimum capabilities to launch with.
		xxxx;
//		Capability::drop_all_capabilities_except(&self.capabilities_to_retain);

		#[inline(always)]
		fn set_current_process_affinity(valid_hyper_threads_for_the_current_process: &BTreeSet<HyperThread>) -> Result<(), DetailedProcessConfigurationError>
		{
			let cpu_set = CpuSet::from(valid_hyper_threads_for_the_current_process);
			cpu_set.set_current_process_affinity().map_err(DetailedProcessConfigurationError::CouldNotSetCurrentProcessAffinity)
		}

		// Is this per thread or per process.
		Capability::clear_all_ambient_capabilities();

		lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();

		Ok(())
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

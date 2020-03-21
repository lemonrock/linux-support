/// Common process configuration.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct DetailedProcessConfiguration
{
	/// Paths to add to `PATH`.
	#[serde(default = "DetailedProcessConfiguration::binary_paths_default")] pub binary_paths: BTreeSet<PathBuf>,

	/// Process niceness.
	#[serde(default)] pub process_niceness: ProcessNiceness,

	/// Soft and hard resource limits.
	#[serde(default = "DetailedProcessConfiguration::resource_limits_default")] pub resource_limits: ResourceLimitsSet,

	/// The folder path to use as the 'current working directory' (CWD).
	///
	/// Equivalent functionality to the shell command `chdir`.
	///
	/// Defaults to `/`.
	#[serde(default = "DetailedProcessConfiguration::working_folder_path_default")] pub working_folder_path: PathBuf,

	/// User and group settings.
	#[serde(default)] pub user_and_group_settings: UserAndGroupSettings,

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

	/// Seccomp.
	#[serde(default)] pub seccomp: SeccompConfiguration,

	/// Location of `/proc`.
	#[serde(default)] pub proc_path: ProcPath,

	/// Location of `/sys`.
	#[serde(default)] pub sys_path: SysPath,

	/// Location of `/dev`.
	#[serde(default)] pub dev_path: DevPath,
}

impl Default for DetailedProcessConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			binary_paths: Self::binary_paths_default(),

			process_niceness: ProcessNiceness::default(),

			resource_limits: Self::resource_limits_default(),

			working_folder_path: Self::working_folder_path_default(),

			user_and_group_settings: UserAndGroupSettings::default(),

			capabilities_to_retain: HashSet::default(),

			seccomp: SeccompConfiguration::default(),

			proc_path: ProcPath::default(),

			sys_path: SysPath::default(),

			dev_path: DevPath::default(),
		}
	}
}

impl DetailedProcessConfiguration
{
	#[inline(always)]
	fn configure(&self, valid_hyper_threads_for_the_current_process: &BTreeSet<HyperThread>, daemonize: Option<&Daemonize>) -> Result<(), DetailedProcessConfigurationError>
	{
		let (effective_user_name, effective_user_home_folder_path) = self.user_and_group_settings.switch_user(daemonize);

		Self::set_current_process_affinity(&valid_hyper_threads_for_the_current_process)?;

		self.adjust_process_niceness()?;

		self.set_resource_limits();

		// TODO: Mucks up logging as we wipe out rust stack trace env variable setting.
		populate_clean_environment(&self.binary_paths, effective_user_name, &effective_user_home_folder_path);

		self.change_current_working_directory();

		self.lock_down_security();

		self.load_seccomp_filters()
	}

	#[inline(always)]
	fn set_current_process_affinity(valid_hyper_threads_for_the_current_process: &BTreeSet<HyperThread>) -> Result<(), DetailedProcessConfigurationError>
	{
		let cpu_set = CpuSet::from(valid_hyper_threads_for_the_current_process);
		cpu_set.set_current_process_affinity().map_err(DetailedProcessConfigurationError::CouldNotSetCurrentProcessAffinity)
	}

	#[inline(always)]
	fn adjust_process_niceness(&self) -> Result<(), DetailedProcessConfigurationError>
	{
		self.process_niceness.adjust(self.proc_path())?;
		Ok(())
	}

	#[inline(always)]
	fn set_resource_limits(&self)
	{
		self.resource_limits.change()
	}

	#[inline(always)]
	fn change_current_working_directory(&self)
	{
		set_current_dir(&self.working_folder_path).unwrap();
	}

	#[inline(always)]
	fn lock_down_security(&self)
	{
		Capability::drop_all_capabilities_except(&self.capabilities_to_retain);

		disable_dumpable();

		no_new_privileges();

		Capability::clear_all_ambient_capabilities();

		lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();
	}

	#[inline(always)]
	fn load_seccomp_filters(&self) -> Result<(), DetailedProcessConfigurationError>
	{
		if unlikely!(self.seccomp.load().is_err())
		{
			Err(DetailedProcessConfigurationError::CouldNotLoadSeccompFilters)
		}
		else
		{
			Ok(())
		}
	}

	/// `/proc`
	#[inline(always)]
	pub(crate) fn proc_path(&self) -> &ProcPath
	{
		&self.proc_path
	}

	/// `/sys`
	#[inline(always)]
	pub(crate) fn sys_path(&self) -> &SysPath
	{
		&self.sys_path
	}

	/// `/dev`
	#[inline(always)]
	pub(crate) fn dev_path(&self) -> &DevPath
	{
		&self.dev_path
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
	fn working_folder_path_default() -> PathBuf
	{
		PathBuf::from("/")
	}
}

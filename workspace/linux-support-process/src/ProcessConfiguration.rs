// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process configuration.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct ProcessConfiguration
{
	/// Logging configuration.
	#[serde(default)] pub logging_configuration: LoggingConfiguration,

	/// Locale.
	#[serde(default = "ProcessConfiguration::locale_default")] pub locale: CString,

	/// Disable hyper threading.
	#[serde(default)] pub disable_hyper_threading: bool,

	/// Configure transparent huge pages?
	#[serde(default)] pub transparent_huge_pages_configuration: Option<TransparentHugePagesConfiguration>,

	/// System control settings (`sysctl`).
	///
	/// By default turns off swapping.
	#[serde(default = "ProcessConfiguration::system_control_settings_default")] pub system_control_settings: HashMap<String, u64>,

	/// Suppress any unwanted warnings about ideal CPU features or the Linux Kernel command line parameters.
	#[serde(default)] pub warnings_to_suppress: WarningsToSuppress,

	/// Detailed process configuration.
	#[serde(default)] pub detailed_process_configuration: DetailedProcessConfiguration,

	/// Should we daemonize? (Default, yes).
	#[serde(default)] pub daemonize: Option<Daemonize>,
}

impl Default for ProcessConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			logging_configuration: LoggingConfiguration::default(),

			locale: Self::locale_default(),

			disable_hyper_threading: false,

			transparent_huge_pages_configuration: Default::default(),

			system_control_settings: Self::system_control_settings_default(),

			warnings_to_suppress: WarningsToSuppress::default(),

			detailed_process_configuration: DetailedProcessConfiguration::default(),

			daemonize: Some(Daemonize::default()),
		}
	}
}

impl ProcessConfiguration
{
	/// Executes a program.
	///
	/// It is recommended that Linux run with at least 2 cores assigned to the Kernel; one of these will be used as a master logical core, and the other will be used for control threads as necessary.
	/// Neither usage is particularly high or critical.
	///
	/// If running interactively `SIGINT` and `SIGQUIT` are intercepted and will be re-raised (using libc's `raise()`) after handling so that any parent shell can behave correctly.
	///
	/// Always returns normally; panics are handled and returned as `ProcessConfigurationExecutionError::ExecutionPanicked`.
	///
	/// Notes:-
	///
	/// * The daemon `irqbalance` should not really be run when this program is running. It isn't incompatible per se, but it isn't useful.
	/// * It is recommended to boot the kernel with the command line parameter `irqaffinity` set to the inverse of `isolcpus`.
	/// * If running causes Linux Kernel modules to load, these are **not** unloaded at process exit as we no longer have the permissions to do so.
	/// * Likewise, if we mount `hugeltbfs` it is not unmounted (and, if we created its mount point folder, this is not deleted) at process exit.
	#[cold]
	pub fn execute<P: Process>(&self, process: P) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		self.start_logging();

		Self::verify_not_running_with_set_uid_bit_set();

		Self::verify_personality_is_standard_linux();

		Self::restrict_umask_to_current_user();

		let result: thread::Result<Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>> = catch_unwind(AssertUnwindSafe(|| self.inner_execute(process)));

		self.stop_logging();

		result?
	}

	fn inner_execute<P: Process>(&self, process: P) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		use self::ProcessConfigurationExecutionError::*;

		block_all_signals_on_current_thread();

		self.set_locale();

		self.enable_or_disable_hyper_threading();

		let valid_hyper_threads_for_the_current_process = self.valid_hyper_threads_for_the_current_process();

		process.load_kernel_modules().map_err(CouldNotLoadKernelModules)?;

		self.write_system_control_values::<P>()?;

		self.rescan_all_pci_buses_and_devices::<P>()?;

		let cpu_features = self.validate_minimal_cpu_features::<P>()?;

		let isolated_hyper_threads_including_those_offline = self.validate_kernel_command_line(&cpu_features, &process)?;
		let (online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core) = self.hyper_thread_sets::<P>(isolated_hyper_threads_including_those_offline);

		self.tell_linux_to_use_shared_hyper_threads_for_all_needs::<P>(&online_shared_hyper_threads_for_os)?;

		self.disable_transparent_huge_pages_if_requested::<P>()?;

		self.detailed_process_configuration.configure(&valid_hyper_threads_for_the_current_process, self.daemonize.as_ref())?;

		let reraise_signal = self.daemonize_if_required(process, online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core).map_err(ExecutionFailed)?;

		match reraise_signal
		{
			Some(reraise_signal_number) =>
			{
				self.stop_logging();
				unsafe { raise(reraise_signal_number) };
				Ok(())
			}

			None => Ok(()),
		}
	}

	#[inline(always)]
	fn daemonize_if_required<P: Process>(&self, process: P, online_shared_hyper_threads_for_os: BTreeSet<HyperThread>, online_shared_hyper_threads_for_process: BTreeSet<HyperThread>, online_isolated_hyper_threads_for_process: BTreeSet<HyperThread>, master_logical_core: HyperThread) -> Result<Option<SignalNumber>, P::MainError>
	{
		let main_loop = AssertUnwindSafe(|| process.main(online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core, self.proc_path()));

		let success_or_failure = match self.daemonize.as_ref()
		{
			None => catch_unwind(main_loop),

			Some(daemonize) =>
			{
				let daemonize_clean_up_on_exit = daemonize.daemonize(self.dev_path());

				let success_or_failure = catch_unwind(main_loop);

				daemonize_clean_up_on_exit.clean_up();

				success_or_failure
			}
		};

		match success_or_failure
		{
			Err(failure) => resume_unwind(failure),

			Ok(reraise_signal_or_failure_explanation) => reraise_signal_or_failure_explanation,
		}
	}

	#[inline(always)]
	fn start_logging(&self)
	{
		self.logging_configuration.start_logging(self.running_interactively())
	}

	#[inline(always)]
	fn set_locale(&self)
	{
		let result = unsafe { setlocale(LC_ALL, self.locale.as_ptr() as *const _) };
		assert!(!result.is_null(), "Could not set locale to `{:?}`", self.locale)
	}

	#[inline(always)]
	fn enable_or_disable_hyper_threading(&self)
	{
		HyperThread::hyper_threading_control(self.sys_path()).map(|current_status|
		{
		   if self.disable_hyper_threading
		   {
			   HyperThread::try_to_disable_hyper_threading(self.sys_path(), current_status)
		   }
		   else
		   {
			   HyperThread::try_to_enable_hyper_threading(self.sys_path(), current_status)
		   }
		});
	}

	#[inline(always)]
	fn valid_hyper_threads_for_the_current_process(&self) -> BTreeSet<HyperThread>
	{
		HyperThread::valid_hyper_threads_for_the_current_process(self.proc_path())
	}

	#[inline(always)]
	fn stop_logging(&self)
	{
		self.logging_configuration.stop_logging()
	}

	#[inline(always)]
	fn write_system_control_values<P: Process>(&self) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		for (setting_name, setting_value) in self.system_control_settings.iter()
		{
			let file_path = self.proc_path().sys_file_path(setting_name);
			file_path.write_value(*setting_value).map_err(ProcessConfigurationExecutionError::CouldNotWriteSystemControlValues)?;
		}
		Ok(())
	}

	#[inline(always)]
	fn rescan_all_pci_buses_and_devices<P: Process>(&self) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		PciDevice::rescan_all_pci_buses_and_devices(self.sys_path()).map_err(ProcessConfigurationExecutionError::RescanOfAllPciBusesAndDevices)
	}

	#[inline(always)]
	fn validate_minimal_cpu_features<P: Process>(&self) -> Result<CpuFeatures, ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		CpuFeatures::validate_minimal_cpu_features(&self.warnings_to_suppress, P::UseEnhancedIntelSpeedStepTechnology).map_err(ProcessConfigurationExecutionError::CpuFeaturesValidationFailed)
	}

	#[inline(always)]
	fn validate_kernel_command_line<P: Process>(&self, cpu_features: &CpuFeatures, process: &P) -> Result<BTreeSet<HyperThread>, ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		let linux_kernel_command_line_validator = LinuxKernelCommandLineValidator::new(self.proc_path());
		let result = linux_kernel_command_line_validator.validate_and_find_isolated_hyper_threads::<P::AdditionalLinuxKernelCommandLineValidationsError, _>
		(
			P::IsolatedCpusRequired,
			&self.warnings_to_suppress,
			cpu_features,
			|linux_kernel_command_line_parameters| process.additional_linux_kernel_command_line_validations(linux_kernel_command_line_parameters, self.proc_path())
		);
		result.map_err(ProcessConfigurationExecutionError::LinuxKernelCommandLineValidationFailed)
	}

	fn hyper_thread_sets<P: Process>(&self, isolated_hyper_threads_including_those_offline: BTreeSet<HyperThread>) -> (BTreeSet<HyperThread>, BTreeSet<HyperThread>, BTreeSet<HyperThread>, HyperThread)
	{
		#[inline(always)]
		fn find_master_logical_core(online_shared_hyper_threads: &BTreeSet<HyperThread>) -> HyperThread
		{
			let master_logical_core = HyperThread::last(online_shared_hyper_threads).unwrap();
			*master_logical_core
		}

		let valid_hyper_threads_for_the_current_process = HyperThread::valid_hyper_threads_for_the_current_process(self.proc_path());

		let (online_shared_hyper_threads_for_os, online_isolated_hyper_threads_for_os) = self.online_shared_and_isolated_hyper_threads(isolated_hyper_threads_including_those_offline);

		let online_shared_hyper_threads_for_process: BTreeSet<HyperThread> = online_shared_hyper_threads_for_os.difference(&valid_hyper_threads_for_the_current_process).cloned().collect();

		let online_isolated_hyper_threads_for_process: BTreeSet<HyperThread> = online_isolated_hyper_threads_for_os.difference(&valid_hyper_threads_for_the_current_process).cloned().collect();

		let master_logical_core = find_master_logical_core(&online_shared_hyper_threads_for_process);

		(online_shared_hyper_threads_for_os, online_shared_hyper_threads_for_process, online_isolated_hyper_threads_for_process, master_logical_core)
	}

	#[inline(always)]
	fn tell_linux_to_use_shared_hyper_threads_for_all_needs<P: Process>(&self, online_shared_hyper_threads: &BTreeSet<HyperThread>) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		use self::ProcessConfigurationExecutionError::*;

		HyperThread::set_work_queue_hyper_thread_affinity(online_shared_hyper_threads, self.sys_path()).map_err(CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads)?;

		HyperThread::force_watchdog_to_just_these_hyper_threads(online_shared_hyper_threads, self.proc_path()).map_err(CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads)
	}

	#[inline(always)]
	fn online_shared_and_isolated_hyper_threads(&self, isolated_hyper_threads_including_those_offline: BTreeSet<HyperThread>) -> (BTreeSet<HyperThread>, BTreeSet<HyperThread>)
	{
		assert_ne!(isolated_hyper_threads_including_those_offline.len(), 0, "There must be at least one hyper thread in `isolated_hyper_threads_including_those_offline`");

		let shared_hyper_threads_including_those_offline = HyperThread::complement(&isolated_hyper_threads_including_those_offline, self.sys_path());
		assert_ne!(shared_hyper_threads_including_those_offline.len(), 0, "There must be at least one hyper thread in `shared_hyper_threads_including_those_offline`");

		let online_isolated_hyper_threads = HyperThread::remove_those_offline(&isolated_hyper_threads_including_those_offline, self.sys_path());
		assert_ne!(online_isolated_hyper_threads.len(), 0, "There must be at least one hyper thread in `online_isolated_hyper_threads`");

		let online_shared_hyper_threads = HyperThread::remove_those_offline(&shared_hyper_threads_including_those_offline, self.sys_path());
		assert_ne!(online_shared_hyper_threads.len(), 0, "There must be at least one hyper thread in `online_shared_hyper_threads`");

		self.warnings_to_suppress.miscellany_warn("too_many_shared_hyper_threads", "There are more than 2 shared hyper threads", || online_shared_hyper_threads.len() <= 2);
		self.warnings_to_suppress.miscellany_warn("too_few_shared_hyper_threads", "There is only 1 shared hyper thread (which will be shared with the master logical core and control threads)", || online_shared_hyper_threads.len() != 1);

		{
			let mut numa_nodes = BTreeSet::new();
			if NumaNode::is_a_numa_machine(self.sys_path())
			{
				for online_shared_hyper_thread in online_shared_hyper_threads.iter()
				{
					let insert = (*online_shared_hyper_thread).numa_node(self.sys_path()).unwrap();
					numa_nodes.insert(insert);
				}
				self.warnings_to_suppress.miscellany_warn("too_many_numa_nodes_shared_hyper_threads", &format!("More than one (actually, {:?}) NUMA nodes are present in the shared hyper threads", numa_nodes), || numa_nodes.len() == 1);
			}
		}

		{
			for hyper_thread_group in HyperThread::hyper_thread_groups(&online_shared_hyper_threads, self.sys_path()).iter()
			{
				let mut hits = 0;
				for hyper_thread in hyper_thread_group.iter()
				{
					if online_shared_hyper_threads.contains(hyper_thread)
					{
						hits += 1;
					}
				}
				self.warnings_to_suppress.miscellany_warn("overlapping_shared_hyper_threads", &format!("More than one (actually, {}) hyper threads of the group '{:?}' are present in the shared hyper threads", hits, hyper_thread_group), || hits < 2);
			}
		}

		(online_shared_hyper_threads, online_isolated_hyper_threads)
	}

	/// Are we running interactively?
	#[inline(always)]
	pub(crate) fn running_interactively(&self) -> bool
	{
		self.daemonize.is_none()
	}

	/// Disable transparent huge pages (THP).
	#[inline(always)]
	pub(crate) fn disable_transparent_huge_pages_if_requested<P: Process>(&self) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		if let Some(ref transparent_huge_pages_configuration) = self.transparent_huge_pages_configuration
		{
			transparent_huge_pages_configuration.configure(self.sys_path())?;
		}
		Ok(())
	}

	/// `/proc`
	#[inline(always)]
	pub(crate) fn proc_path(&self) -> &ProcPath
	{
		&self.detailed_process_configuration.proc_path()
	}

	/// `/sys`
	#[inline(always)]
	pub(crate) fn sys_path(&self) -> &SysPath
	{
		&self.detailed_process_configuration.sys_path()
	}

	/// `/dev`
	#[inline(always)]
	pub(crate) fn dev_path(&self) -> &DevPath
	{
		&self.detailed_process_configuration.dev_path()
	}

	#[inline(always)]
	fn verify_not_running_with_set_uid_bit_set()
	{
		assert_eq!(unsafe { geteuid() }, unsafe { getuid() }, "Can not be run with set uid bit set ('setuid')");
	}

	#[inline(always)]
	fn verify_personality_is_standard_linux()
	{
		let current_personality = PersonalityFlags::get_process_domain_execution_model();
		assert!(current_personality.expect("Can not get personality").is_standard_linux(), "Personality is not standard Linux but '{:?}'", current_personality);
	}

	#[inline(always)]
	fn restrict_umask_to_current_user()
	{
		unsafe { umask(0o0077) };
	}

	#[inline(always)]
	fn locale_default() -> CString
	{
		unsafe { CString::from_vec_unchecked(b"en_US.UTF-8".to_vec()) }
	}

	#[inline(always)]
	fn system_control_settings_default() -> HashMap<String, u64>
	{
		hashmap!
		{
			"vm.swappiness".to_string() => 0,
			"vm.zone_reclaim_mode".to_string() => 0,
			"vm.dirty_ratio".to_string() => 10,
			"vm.dirty_background_ratio".to_string() => 5,
		}
	}
}

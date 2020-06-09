// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process configuration.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessConfiguration
{
	/// System control settings (`sysctl`).
	///
	/// By default turns off swapping.
	#[serde(default = "ProcessConfiguration::system_control_settings_default")] pub system_control_settings: HashMap<String, u64>,
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
	#[cold]
	pub fn execute<P: Process>(&self, process: P) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
	}

	#[inline(always)]
	fn system_control_settings_default() -> HashMap<String, u64>
	{
		hashmap!
		{
			"vm.zone_reclaim_mode".to_string() => 0,
			"vm.dirty_ratio".to_string() => 10,
			"vm.dirty_background_ratio".to_string() => 5,
		}
	}



	#[inline(always)]
	fn rescan_all_pci_buses_and_devices<P: Process>(&self) -> Result<(), ProcessConfigurationExecutionError<P::LoadKernelModulesError, P::AdditionalLinuxKernelCommandLineValidationsError, P::MainError>>
	{
		PciDevice::rescan_all_pci_buses_and_devices(self.sys_path()).map_err(ProcessConfigurationExecutionError::RescanOfAllPciBusesAndDevices)
	}

	fn hyper_thread_sets<P: Process>(&self, isolated_hyper_threads_including_those_offline: BTreeSet<HyperThread>) -> (BitSet<HyperThread>, BitSet<HyperThread>, BitSet<HyperThread>, HyperThread)
	{
		#[inline(always)]
		fn find_master_logical_core(online_shared_hyper_threads: &BitSet<HyperThread>) -> HyperThread
		{
			let master_logical_core = online_shared_hyper_threads.last().unwrap();
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
	fn online_shared_and_isolated_hyper_threads(&self, isolated_hyper_threads_including_those_offline: HyperThreads) -> (HyperThreads, HyperThreads)
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
			let mut numa_nodes = BitSet::new();
			if NumaNode::is_a_numa_machine(self.sys_path())
			{
				for online_shared_hyper_thread in online_shared_hyper_threads.iter()
				{
					let insert = (*online_shared_hyper_thread).numa_node(self.sys_path()).unwrap();
					numa_nodes.add(insert);
				}
				self.warnings_to_suppress.miscellany_warn("too_many_numa_nodes_shared_hyper_threads", &format!("More than one (actually, {:?}) NUMA nodes are present in the shared hyper threads", numa_nodes), || numa_nodes.len() == 1);
			}
		}


		/// Hyper threaded logical cores grouped as hyper thread groups (eg HT 0 and 1, 2 and 3, etc).
		#[inline(always)]
		fn hyper_thread_groups(hyper_threads: &BitSet<HyperThread>, sys_path: &SysPath) -> BTreeSet<BitSet<HyperThread>>
		{
			let mut hyper_thread_groups = BTreeSet::new();
			for hyper_thread in hyper_threads.iterate()
			{
				let hyper_thread_group = (*hyper_thread).level1_cache_hyper_thread_siblings_including_self(sys_path);
				hyper_thread_groups.insert(hyper_thread_group);
			}
			hyper_thread_groups
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
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global scheduling configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSchedulingConfiguration
{
	/// Requires root.
	pub enable_or_disable_hyperthreading: Option<bool>,

	/// Adjust autogroup globally (`Some(false)` to disable it globally).
	///
	/// Requires root.
	pub globally_enable_autogroup_if_true_disable_if_false: Option<bool>,

	/// Changes the quantum used for the round-robin schedulers `Normal` and `RealTimeRoundRobin`.
	///
	/// Requires root.
	pub round_robin_quantum: Option<RoundRobinQuantumMilliseconds>,

	/// Changes the reserved amount of time (by default, 5%) for non-real-time scheduler policies.
	///
	/// Requires root.
	pub reserved_cpu_time_for_non_real_time_scheduler_policies: Option<ReservedCpuTimeForNonRealTimeSchedulerPolicies>,

	/// Enables soft watchdog lockup detection.
	///
	/// Not advisable if we're taking over the entire machine.
	///
	/// See also `GlobalKernelPanicConfiguration::panic_on_software_watchdog_lockup` and `GlobalKernelPanicConfiguration::capture_debug_information_on_software_watchdog_lockup`.
	///
	/// Requires root.
	pub enable_software_watchdog_lockup_detection: Option<bool>,

	/// Enables hard watchdog lockup detection.
	///
	/// Not advisable if we're taking over the entire machine.
	///
	/// See also `GlobalKernelPanicConfiguration::panic_on_hardware_watchdog_lockup` and `GlobalKernelPanicConfiguration::capture_debug_information_on_hardware_watchdog_lockup`.
	///
	/// Requires root.
	pub enable_hardware_watchdog_lockup_detection: Option<bool>,

	/// This controls the frequency of checks.
	///
	/// The software watchdog threshold is double this value.
	///
	/// Requires root.
	pub hardware_watchdog_threshold_in_seconds: Option<u32>,

	/// It is recommended that this value be computed.
	///
	/// If the watchdogs are disabled, then this value does not need to change.
	///
	/// Requires root.
	pub software_and_hardware_watchdog_runs_on_which_kernel_cpus: Option<HyperThreads>,

	/// It is recommended that this value be computed.
	///
	/// Requires root.
	pub work_queue_runs_on_which_kernel_cpus: Option<HyperThreads>,
}

impl GlobalSchedulingConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalSchedulingConfigurationError>
	{
		use self::GlobalSchedulingConfigurationError::*;

		let _current_hyper_threading_status = self.adjust_hyperthreading(sys_path);

		set_proc_sys_kernel_value(proc_path, "sched_autogroup_enabled", self.globally_enable_autogroup_if_true_disable_if_false, CouldNotChangeAutogroup)?;
		set_value(proc_path, |proc_path, value| value.write(proc_path), self.round_robin_quantum, CouldNotChangeRoundRobinQuantum)?;
		set_value(proc_path, |proc_path, value| value.write(proc_path), self.reserved_cpu_time_for_non_real_time_scheduler_policies, CouldNotChangeReservedCpuTimeForNonRealTimeSchedulerPolicies)?;
		set_proc_sys_kernel_value(proc_path, "software_watchdog", self.enable_software_watchdog_lockup_detection, CouldNotChangeSoftwareWatchdog)?;
		set_proc_sys_kernel_value(proc_path, "nmi_watchdog", self.enable_hardware_watchdog_lockup_detection, CouldNotChangeHardwareWatchdog)?;
		set_proc_sys_kernel_value(proc_path, "watchdog_thresh", self.hardware_watchdog_threshold_in_seconds.map(UnpaddedDecimalInteger), CouldNotChangeHardwareWatchdogThreshold)?;
		set_value(proc_path, |proc_path, value| value.force_watchdog_to_just_these_hyper_threads(proc_path), self.software_and_hardware_watchdog_runs_on_which_kernel_cpus.as_ref(), CouldNotChangeSoftwareAndHardwareWatchdogCpus)?;
		set_value(proc_path, |_proc_path, value| value.set_work_queue_hyper_thread_affinity(sys_path), self.work_queue_runs_on_which_kernel_cpus.as_ref(), CouldNotChangeWorkQueueCpus)
	}

	#[inline(always)]
	fn adjust_hyperthreading(&self, sys_path: &SysPath) -> HyperThreadingStatus
	{
		use self::HyperThreadingStatus::*;
		if Self::has_hyper_threading()
		{
			if let Some(current_status) = HyperThread::hyper_threading_control(sys_path)
			{
				if let Some(enable_or_disable_hyperthreading) = self.enable_or_disable_hyperthreading
				{
					match enable_or_disable_hyperthreading
					{
						true => HyperThread::try_to_enable_hyper_threading(sys_path, current_status),
						false => HyperThread::try_to_disable_hyper_threading(sys_path, current_status),
					}
				}
				else
				{
					current_status
				}
			}
			else
			{
				NotSupported
			}
		}
		else
		{
			NotImplemented
		}
	}

	#[cfg(target_arch = "x86_64")]
	fn has_hyper_threading() -> bool
	{
		CpuId::new().get_feature_info().unwrap().has_htt()
	}

	#[cfg(not(target_arch = "x86_64"))]
	fn has_hyper_threading() -> bool
	{
		true
	}
}

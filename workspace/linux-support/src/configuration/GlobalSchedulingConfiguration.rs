// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global scheduling configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
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
	
	/// The initial value for the scheduler period.
	///
	/// The scheduler period is a period of time during which all runnable tasks should be allowed to run at least once.
	/// While the Completely Fair Scheduler (CFS) has no concept of time slices, you can think of the period as the initial chunk of time which is then divided evenly into time slices, one for each runnable process.
	///
	/// Note that this tunable only specifies the initial value.
	/// When too many tasks become runnable ('high load'_ the scheduler	will use `minimum_granularity` instead.
	///
	/// Requires root.
	pub latency: Option<Nanoseconds>,
	
	/// Controls whether the scheduler can adjust `latency`.
	///
	/// The adjustment made is based on the number of CPUs, and increases logarithmically or linearly as implied in the available values.
	///
	/// This is due to the fact that with more CPUs there is an apparent reduction in perceived latency.
	pub latency_scaling: Option<LatencyScaling>,
	
	/// Unlike `latency`, this tunable specifies the target period allocated for each task to run rather than the time in which all tasks should be run once.
	///
	/// Only used under 'high load'.
	///
	/// Requires root.
	pub minimum_granularity: Option<Nanoseconds>,
	
	/// The target period allocated for each task that wakes up.
	///
	/// Requires root.
	pub wake_up_granularity: Option<Nanoseconds>,
	
	/// Amount of time after the last execution that a task is considered to be “cache hot” in migration decisions.
	///
	/// A “cache hot” task is less likely to be migrated, so increasing this variable reduces task migrations.
	///
	/// The default value is 50,0000 nanoseconds.
	///
	/// If the CPU idle time is higher than expected when there are runnable processes, try reducing this value.
	/// If tasks bounce between CPUs or nodes too often, try increasing it.
	///
	/// Requires root.
	pub migration_cost: Option<Nanoseconds>,
	
	/// Completely Fair Scheduler (CFS) bandwidth slice.
	///
	/// Only applies to processes scheduled with `SCHED_NORMAL` (`SCHED_OTHER`).
	///
	/// Default is 5 milliseconds, ie `Microseconds(5000)`.
	///
	/// Requires root.
	pub completely_fair_scheduler_bandwidth_slice: Option<Microseconds>,

	/// Increasing the sched_nr_migrate variable gives high performance from `SCHED_NORMAL` (`SCHED_OTHER`) threads that spawn lots of tasks, at the expense of real-time latencies.
	///
	/// For low real-time task latency at the expense of  `SCHED_NORMAL` (`SCHED_OTHER`) task performance, the value must be lowered.
	///
	/// The default value is 8.
	///
	/// Requires root.
	pub number_of_normal_tasks_to_migrate_to_another_hyper_thread_at_once: Option<NonZeroU8>,
	
	/// Disabling statistics gives a very small performance gain.
	///
	/// Requires root.
	pub enable_statistics: Option<bool>,
	
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

	/// RCU grace period.
	///
	/// Only rarely needs to be changed.
	pub rcu_grace_period: Option<RcuGracePeriodConfiguration>,
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
	
		set_proc_sys_kernel_value(proc_path, "sched_latency_ns", self.latency, CouldNotChangeLatency)?;
		set_value(proc_path, |proc_path, value| value.write(proc_path), self.latency_scaling, CouldNotChangeLatencyScaling)?;
		set_proc_sys_kernel_value(proc_path, "sched_min_granularity_ns", self.minimum_granularity, CouldNotChangeMinimumGranularity)?;
		set_proc_sys_kernel_value(proc_path, "sched_wakeup_granularity_ns", self.wake_up_granularity, CouldNotChangeWakeUpGranularity)?;
		set_proc_sys_kernel_value(proc_path, "sched_migration_cost_ns", self.migration_cost, CouldNotChangeMigrationCost)?;
		set_proc_sys_kernel_value(proc_path, "sched_cfs_bandwidth_slice_us", self.completely_fair_scheduler_bandwidth_slice, CouldNotChangeMigrationCompletelyFairSchedulerBandwidthSlice)?;
		set_proc_sys_kernel_value(proc_path, "sched_nr_migrate", self.number_of_normal_tasks_to_migrate_to_another_hyper_thread_at_once, CouldNotChangeNumberOfNormalTasksToMigrate)?;
	
		set_proc_sys_kernel_value(proc_path, "sched_schedstats", self.enable_statistics, CouldNotChangeStatisticsEnablement)?;
	
		set_value(proc_path, |proc_path, value| value.write(proc_path), self.round_robin_quantum, CouldNotChangeRoundRobinQuantum)?;
		set_value(proc_path, |proc_path, value| value.write(proc_path), self.reserved_cpu_time_for_non_real_time_scheduler_policies, CouldNotChangeReservedCpuTimeForNonRealTimeSchedulerPolicies)?;
		set_proc_sys_kernel_value(proc_path, "software_watchdog", self.enable_software_watchdog_lockup_detection, CouldNotChangeSoftwareWatchdog)?;
		set_proc_sys_kernel_value(proc_path, "nmi_watchdog", self.enable_hardware_watchdog_lockup_detection, CouldNotChangeHardwareWatchdog)?;
		set_proc_sys_kernel_value(proc_path, "watchdog_thresh", self.hardware_watchdog_threshold_in_seconds.map(UnpaddedDecimalInteger), CouldNotChangeHardwareWatchdogThreshold)?;
		set_value(proc_path, |_proc_path, value| value.set(sys_path), self.rcu_grace_period, CouldNotChangeRcuGracePeriod)?;
		
		Ok(())
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

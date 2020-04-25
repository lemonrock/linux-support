// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global scheduling configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
}

impl GlobalSchedulingConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalSchedulingConfigurationError>
	{
		use self::GlobalSchedulingConfigurationError::*;

		let current_hyper_threading_status = if let Some(current_status) = HyperThread::hyper_threading_control(sys_path)
		{
			Some
			(
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
			)
		}
		else
		{
			None
		};

		if let Some(globally_enable_autogroup_if_true_disable_if_false) = self.globally_enable_autogroup_if_true_disable_if_false
		{
			assert_effective_user_id_is_root(&format!("change_autogroup '{:?}'", globally_enable_autogroup_if_true_disable_if_false));
			ProcessNiceConfiguration::sched_autogroup_enabled_file_path(proc_path).write_value(globally_enable_autogroup_if_true_disable_if_false).map_err(|cause| CouldNotChangeAutogroup(cause))?
		}

		if let Some(round_robin_quantum) = self.round_robin_quantum
		{
			round_robin_quantum.write(proc_path).map_err(|cause| CouldNotChangeRoundRobinQuantum(cause))?
		}

		if let Some(reserved_cpu_time_for_non_real_time_scheduler_policies) = self.reserved_cpu_time_for_non_real_time_scheduler_policies
		{
			reserved_cpu_time_for_non_real_time_scheduler_policies.write(proc_path).map_err(|cause| CouldNotChangeReservedCpuTimeForNonRealTimeSchedulerPolicies(cause))?
		}

		Ok(())
	}
}

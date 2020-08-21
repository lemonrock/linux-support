// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchedulingDiagnostics
{
	latency_scheduling: DiagnosticUnobtainableResult<LatencyScaling>,
	autogroup_name_and_nice_value: DiagnosticUnobtainableResult<(Box<[u8]>, Nice)>,
	process_group_priority: DiagnosticUnobtainableResult<Nice>,
	process_priority: DiagnosticUnobtainableResult<Nice>,
	real_user_priority: DiagnosticUnobtainableResult<Nice>,
	rcu_grace_period: DiagnosticUnobtainableResult<RcuGracePeriodConfiguration>,
	reserved_cpu_time_for_non_real_time_scheduler_policies: DiagnosticUnobtainableResult<ReservedCpuTimeForNonRealTimeSchedulerPolicies>,
	round_robin_interval: Option<RoundRobinInterval>,
	round_robin_interval_quantum_milliseconds: DiagnosticUnobtainableResult<RoundRobinQuantumMilliseconds>,
}

impl SchedulingDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath, process_group_identifier: ProcessGroupIdentifierChoice, process_identifier: ProcessIdentifierChoice) -> Self
	{
		Self
		{
			latency_scheduling: LatencyScaling::read(proc_path).map_err(DiagnosticUnobtainable::from),
			autogroup_name_and_nice_value: Nice::get_autogroup_name_and_nice_value(process_identifier, proc_path).map_err(DiagnosticUnobtainable::from),
			process_group_priority: Nice::get_process_group_priority(process_group_identifier).map_err(DiagnosticUnobtainable::from),
			process_priority: Nice::get_process_priority(process_identifier).map_err(DiagnosticUnobtainable::from),
			real_user_priority: Nice::get_real_user_priority(UserIdentifier::current_real()).map_err(DiagnosticUnobtainable::from),
			rcu_grace_period: RcuGracePeriodConfiguration::get(sys_path).map_err(DiagnosticUnobtainable::from),
			reserved_cpu_time_for_non_real_time_scheduler_policies: ReservedCpuTimeForNonRealTimeSchedulerPolicies::read(proc_path).map_err(DiagnosticUnobtainable::from),
			round_robin_interval: RoundRobinInterval::for_process(process_identifier),
			round_robin_interval_quantum_milliseconds: RoundRobinQuantumMilliseconds::read(proc_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}

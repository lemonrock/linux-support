// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux scheduler policy.
///
/// If thread does not have the `CAP_SYS_NICE` capability then change are restricted.
/// Thus:-
///
/// * it can not switch to a real-time policy (ie either `RealTimeFirstInFirstOut` or `RealTimeRoundRobin`).
/// * it can switch to the `Deadline` policy.
/// * if it has a policy of `Idle` it can change to only `Batch` or `Normal` but only if its `nice` value does not exceed `RLIMIT_NICE`.
/// * if is has a real-time policy then the value of `RLIMIT_RTPRIO` affects it:-
/// 	* if it has a nonzero `RLIMIT_RTPRIO` soft limit: it can change its scheduling policy and priority, subject to the restriction that the priority cannot be set to a value higher han the maximum of its current priority and its `RLIMIT_RTPRIO` soft limit.
/// 	* if it it has a zero `RLIMIT_RTPRIO` soft limit: the only permitted changes are to lower the priority, or to switch to a non-real-time policy.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SchedulerPolicy
{
	/// For running very low priority background jobs.
	///
	/// Lower priority than `Idle`.
	///
	/// The process' `nice` value is ignored.
	///
	/// This policy is intended for running jobs at extremely low priority (lower even than a `+19` nice value with the `Normal` or `Batch` policies).
	///
	/// A thread with this policy can only switch to either the `Batch` or the `Normal` policies so long as its nice value falls within the range permitted by its `RLIMIT_NICE` resource limit (see `getrlimit()`), *unless* it has the `CAP_SYS_NICE` capability.
	Idle,

	/// For 'batch' style execution of processes.
	///
	/// Slightly lower priority than `Normal`, higher than `Idle`.
	///
	/// Since Linux 2.6.16.
	///
	/// This policy is similar to `Normal` in that it schedules the process according to its dynamic priority (based on the nice value).
	/// The difference is that this policy will cause the scheduler to always assume that the process is CPU-intensive.
	/// Consequently, the scheduler will apply a small scheduling penalty with respect to wakeup behaviour, so that this process is mildly disfavored in scheduling decisions.
	/// This policy is useful for workloads that are noninteractive, but do not want to lower their nice value, and for workloads that want a deterministic scheduling policy without interactivity causing extra preemptions (between the workload's tasks).
	Batch(Nice),

	/// The standard (and default) round-robin time-sharing scheduler.
	///
	/// This is sometimes called `Other`.
	///
	/// The process to run is chosen from the static priority 0 list based on a dynamic priority that is determined only inside this list.
	/// 
	/// static priority 0 list is one below `RealTimePriority(1)`.
	///
	/// The dynamic priority is based on the nice value (set by `setpriority()`) and increased for each time quantum the process is ready to run, but denied to run by the scheduler.
	///
	/// This ensures fair progress among all `Normal` processes.
	Normal(Nice),

	/// A first-in, first-out real time scheduler.
	///
	/// When a `RealTimeFirstInFirstOut` processes becomes runnable, it will always immediately preempt any currently running `Normal`, `Batch` or `Idle` process.
	///
	/// A `RealTimeFirstInFirstOut` process that has been preempted by another process of higher priority will stay at the head of the list for its priority and will resume execution as soon as all processes of higher priority are blocked again.
	///
	/// When a `RealTimeFirstInFirstOut` process becomes runnable, it will be inserted at the end of the list for its priority.
	///
	/// A `RealTimeFirstInFirstOut` process runs until either it is blocked by an I/O request, it is preempted by a higher priority process, or it calls `thread_yield()`.
	RealTimeFirstInFirstOut(RealTimePriority),

	/// A round-robin real time scheduler.
	///
	/// Everything described above for `RealTimeFirstInFirstOut` also applies to `RealTimeRoundRobin`, except that each process is only allowed to run for a maximum time quantum.
	///
	/// If a `RealTimeRoundRobin` process has been running for a time period equal to or longer than the time quantum, it will be put at the end of the list for its priority.
	///
	/// A `RealTimeRoundRobin` process that has been preempted by a higher priority process and subsequently resumes execution as a running process will complete the unexpired portion of its round-robin time quantum.
	///
	/// The length of the time quantum can be retrieved using `sched_rr_get_interval()`.
	RealTimeRoundRobin(RealTimePriority),

	/// A real time scheduler that takes precedence over all other schedulers.
	///
	/// Also known as "Earliest-Deadline-First" (EDF).
	///
	/// Since Linux 3.14.
	///
	/// Using a Deadline scheduler is impossible if a thread has an affinity to less than the total CPUs on the system (or in the current cgroup).
	///
	/// A thread must have the `CAP_SYS_NICE` capability in order to set or modify a `Deadling` policy
	Deadline
	{
		/// Runtime parameter.
		runtime_in_nanoseconds: u64,

		/// Deadline parameter.
		deadline_in_nanoseconds: u64,

		/// Period parameter.
		period_in_nanoseconds: u64,
	},
}

impl Default for SchedulerPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		SchedulerPolicy::Normal(Nice::Zero)
	}
}

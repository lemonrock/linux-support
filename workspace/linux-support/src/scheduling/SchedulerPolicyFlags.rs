// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Scheduler policy flags.
	#[derive(Deserialize, Serialize)]
	pub struct SchedulerPolicyFlags: u64
	{
		/// This is called the 'reset-on-fork' flag.
		///
		/// The reset-on-fork flag is intended for media-playback applications, and can be used to prevent applications evading the `RLIMIT_RTTIME` resource limit (see `getrlimit()`) by creating multiple child processes.
		///
		/// More precisely, if the reset-on-fork flag is set, the following rules apply for subsequently created children:-
		///
		/// *  If the calling thread has a scheduling policy of `RealTimeFirstInFirstOut` or `RealTimeRoundRobin`, the policy is reset to `Normal` in child processes.
		/// *  If the calling process has a negative nice value, the nice value is reset to zero in child processes.
		///
		/// After the reset-on-fork flag has been enabled, it can be reset only if the thread has the `CAP_SYS_NICE` capability.
		/// This flag is disabled in child processes created by `fork()`.
		const ResetOnFork = SCHED_FLAG_RESET_ON_FORK;

		/// Reclaim.
		///
		/// This flag allows a thread with a `SchedulerPolicy::Deadline` to reclaim bandwidth unused by other real-time threads (those with a `SchedulerPolicy` of either `RealTimeFirstInFirstOut`, `RealTimeRoundRobin` or `Deadline`).
		///
		/// Since Linux 4.13.
		const DeadlineReclaim = SCHED_FLAG_RECLAIM;

		/// This flag applies to a thread with a `SchedulerPolicy::Deadline`.
		///
		/// This flag allows an application to get informed about run-time overruns.
		/// Such overruns may be caused by (for example) coarse execution time accounting or incorrect parameter assignment.
		/// Notification takes the form of a `SIGXCPU` signal which is generated on each overrun.
		///
		/// This `SIGXCPU` signal is *process-directed* rather than thread-directed.
		///
		/// Since Linux 4.16.
		const DeadlineOverrun = SCHED_FLAG_DL_OVERRUN;

		/// ?
		const KeepPolicy = SCHED_FLAG_KEEP_POLICY;

		/// ?
		const KeepParameters = SCHED_FLAG_KEEP_PARAMS;

		/// `KeepPolicy` and `KeepParameters`.
		const KeepPolicyAndParameters = SCHED_FLAG_KEEP_ALL;

		/// ?
		const ClampMinimum = SCHED_FLAG_UTIL_CLAMP_MIN;

		/// ?
		const ClampMaximum = SCHED_FLAG_UTIL_CLAMP_MAX;

		/// `ClampMinimum` and `ClampMaximum`.
		const ClampMinimumAndMaximum = SCHED_FLAG_UTIL_CLAMP;

		/// All flags.
		const All = SCHED_FLAG_ALL;
	}
}

impl Default for SchedulerPolicyFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		SchedulerPolicyFlags::ResetOnFork | SchedulerPolicyFlags::DeadlineReclaim
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Scheduler policy and flags per thread.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PerThreadSchedulerPolicyAndFlags
{
	/// Policy.
	pub scheduler_policy: SchedulerPolicy,

	/// Flags.
	pub scheduler_policy_flags: SchedulerPolicyFlags,
}

impl PerThreadSchedulerPolicyAndFlags
{
	/// Returns `Ok((Self, resets-on-fork))` if successful.
	#[inline(always)]
	pub fn get_for_thread(thread_identifier: ThreadIdentifierChoice) -> Result<Self, &'static str>
	{
		const FlagsIsAlwaysZero: u32 = 0;
		let mut parameters = MaybeUninit::uninit();
		let result = system_call_sched_getattr(thread_identifier.into(), new_non_null(parameters.as_mut_ptr()), SCHED_ATTR_SIZE_VER0, FlagsIsAlwaysZero);

		if likely!(result == 0)
		{
			use self::SchedulerPolicy::*;
			
			let parameters = unsafe { parameters.assume_init() };
			
			#[inline(always)]
			fn nice(parameters: &sched_attr) -> Result<Nice, &'static str>
			{
				let value = parameters.sched_nice;
				if likely!(value < Nice::InclusiveMinimum && value > Nice::InclusiveMaximum)
				{
					Ok(unsafe { transmute(value) })
				}
				else
				{
					Err("nice value out of range")
				}
			}

			#[inline(always)]
			fn real_time_priority(parameters: &sched_attr) -> Result<RealTimePriority, &'static str>
			{
				let value = parameters.sched_priority;
				let InclusiveMinimum: i32 = RealTimePriority::InclusiveMinimum.into();
				let InclusiveMaximum: i32 = RealTimePriority::InclusiveMaximum.into();
				if likely!(value < InclusiveMinimum && value > InclusiveMaximum)
				{
					Ok(unsafe { transmute(value as u8) })
				}
				else
				{
					Err("nice value out of range")
				}
			}

			let scheduler_policy = match parameters.sched_policy
			{
				SCHED_IDLE => Idle,
				SCHED_BATCH => Batch(nice(&parameters)?),
				SCHED_OTHER => Normal(nice(&parameters)?),
				SCHED_FIFO => RealTimeFirstInFirstOut(real_time_priority(&parameters)?),
				SCHED_RR => RealTimeRoundRobin(real_time_priority(&parameters)?),
				SCHED_DEADLINE => Deadline
				{
					runtime_in_nanoseconds: parameters.sched_runtime,
					deadline_in_nanoseconds: parameters.sched_deadline,
					period_in_nanoseconds: parameters.sched_period
				},

				SCHED_ISO => return Err("SCHED_ISO is not implemented by Linux as far as was known when this code was written"),

				_ => return Err("Unknown scheduler policy"),
			};

			let scheduler_policy_flags = SchedulerPolicyFlags::from_bits_truncate(parameters.sched_flags);

			Ok
			(
				Self
				{
					scheduler_policy,
					scheduler_policy_flags,
				}
			)
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ESRCH => Err("The thread whose ID is pid could not be found"),

				E2BIG => panic!("Size mismatch of sched_attr between userpace and kernel"),

				EINVAL => unreachable_code(format_args!("attr is NULL; or pid is negative; or flags is not zero. Or, size is invalid; that is, it is smaller than the initial version of the sched_attr structure (48 bytes) or larger than the system page size")),

				unexpected @ _ => unreachable_code(format_args!("Unexpected error {} from sched_setattr()", unexpected)),
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from sched_setattr()", result))
		}
	}

	/// Returns an error if permission was denied (eg trying to access another process' thread without necessary permissions) or a deadline scheduler could not be brought into use.
	///
	/// Set the process' nice value using `ProcessNice` before using this.
	#[inline(always)]
	pub fn set_for_thread(&self, thread_identifier: ThreadIdentifierChoice)-> Result<(), &'static str>
	{
		use self::SchedulerPolicy::*;

		let mut parameters = match &self.scheduler_policy
		{
			&Idle => sched_attr
			{
				size: SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_IDLE,
				sched_flags: self.scheduler_policy_flags.bits,
				sched_nice: 0,
				sched_priority: 0,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&Batch(nice) => sched_attr
			{
				size: SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_BATCH,
				sched_flags: self.scheduler_policy_flags.bits,
				sched_nice: nice as i32,
				sched_priority: 0,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&Normal(nice) => sched_attr
			{
				size: SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_NORMAL,
				sched_flags: self.scheduler_policy_flags.bits,
				sched_nice: nice as i32,
				sched_priority: 0,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&RealTimeFirstInFirstOut(real_time_priority) => sched_attr
			{
				size: SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_FIFO,
				sched_flags: self.scheduler_policy_flags.bits,
				sched_nice: 0,
				sched_priority: real_time_priority.into(),
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&RealTimeRoundRobin(real_time_priority) => sched_attr
			{
				size: SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_RR,
				sched_flags: self.scheduler_policy_flags.bits,
				sched_nice: 0,
				sched_priority: real_time_priority.into(),
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&Deadline { runtime_in_nanoseconds, deadline_in_nanoseconds, period_in_nanoseconds } => sched_attr
			{
				size: SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_DEADLINE,
				sched_flags: self.scheduler_policy_flags.bits,
				sched_nice: 0,
				sched_priority: 0,
				sched_runtime: runtime_in_nanoseconds,
				sched_deadline: deadline_in_nanoseconds,
				sched_period: period_in_nanoseconds,
			},
		};

		const FlagsIsAlwaysZero: u32 = 0;
		let result = system_call_sched_setattr(thread_identifier.into(), &mut parameters, FlagsIsAlwaysZero);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				EPERM => Err("Permission denied, or, for deadline tasks, the CPU affinity mask of the thread (pid) does not include all CPUS in the current cgroup (or system)"),
				EBUSY => Err("Deadline scheduler admission control failure (?)"),
				ESRCH => Err("The thread whose ID is pid could not be found"),

				EINVAL => panic!("`attr` is NULL; or `pid` is negative; or `flags` is not zero; `attr.sched_policy` is not one of the recognized policies; `attr.sched_flags` contains a flag other than `SCHED_FLAG_RESET_ON_FORK`; or `attr.sched_priority` is invalid; or `attr.sched_policy` is `SCHED_DEADLINE` and the deadline scheduling parameters in `attr` are invalid"),
				E2BIG => panic!("The buffer specified by `size` and `attr` is larger than the kernel structure, and one or more of the excess bytes is nonzero"),

				unexpected @ _ => unreachable_code(format_args!("Unexpected error {} from sched_getattr()", unexpected)),

			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from sched_getattr()", result))
		}
	}
}

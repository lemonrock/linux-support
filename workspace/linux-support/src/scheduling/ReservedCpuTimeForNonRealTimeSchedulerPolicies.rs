// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Time reserved for threads not using real-time or deadline SchedulerPolicy (ie using `Idle`, `Batch` and `Normal` (`Other`)).
///
/// Reserving CPU time in this fashion allows some CPU time to be allocated to (say) a root shell that can be used to kill a runaway process.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ReservedCpuTimeForNonRealTimeSchedulerPolicies
{
	period_microseconds: NonZeroU32,

	runtime_microseconds: Option<NonZeroU32>,
}

impl ReservedCpuTimeForNonRealTimeSchedulerPolicies
{
	/// `period_microseconds`: `1 to i32::MAX as u32`. Default is 1,000,000 microseconds (1 second).
	/// `runtime_microseconds`: `None or 1 to i32::MAX as u32` (but shifted for use by Linux to the range `-1 to (i32::MAX - 1) as u32`). Default is `Some(950,000 microseconds)`.
	/// If `None` then no time is reserved.
	///
	/// `period_microseconds` is always greater than or equal to `runtime_microseconds` if it is `Some()`.
	///
	/// The ratio of `runtime_microseconds : period_microseconds` is the amount of time reserved for threads using real time and deadline scheduler policies.
	/// The ratio of `(period_microseconds - runtime_microseconds) : period_microseconds` is the amount of time reserved for threads not using real time and deadline scheduler policies.
	///
	/// The defaults ensure a reservation of 5%.
	#[inline(always)]
	pub fn new((period_microseconds, runtime_microseconds): (NonZeroU32, Option<NonZeroU32>)) -> Self
	{
		debug_assert!(period_microseconds.get() <= i32::MAX as u32);

		if cfg!(debug_assertions)
		{
			if let Some(runtime_microseconds) = runtime_microseconds
			{
				debug_assert!(runtime_microseconds.get() <= i32::MAX as u32);
				debug_assert!(period_microseconds >= runtime_microseconds)
			}
		}

		Self
		{
			period_microseconds,
			runtime_microseconds,
		}
	}

	/// This is not atomic; a partial write can occur.
	#[inline(always)]
	pub fn write(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		Self::period_file_path(proc_path).write_value(self.period_microseconds)?;
		Self::runtime_file_path(proc_path).write_value(match self.runtime_microseconds
		{
			None => -1i32,
			Some(runtime_microseconds) => (runtime_microseconds.get() - 1) as i32
		})
	}

	/// This is not atomic.
	#[inline(always)]
	pub fn read(proc_path: &ProcPath) -> io::Result<Self>
	{
		let period_microseconds = Self::period_file_path(proc_path).read_value()?;
		let runtime_microseconds_i32: i32 = Self::runtime_file_path(proc_path).read_value()?;
		if unlikely!(runtime_microseconds_i32 < -1)
		{
			return Err(io::Error::new(ErrorKind::Other, "Value of runtime_microseconds < -1"));
		}
		if unlikely!(runtime_microseconds_i32 == i32::MAX)
		{
			return Err(io::Error::new(ErrorKind::Other, "Value of runtime_microseconds i32::MAX"));
		}
		let runtime_microseconds = NonZeroU32::new((runtime_microseconds_i32 + 1) as u32);
		Ok
		(
			Self
			{
				period_microseconds,
				runtime_microseconds,
			}
		)
	}

	#[inline(always)]
	fn period_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("sched_rt_period_us")
	}

	#[inline(always)]
	fn runtime_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("sched_rt_runtime_us")
	}
}

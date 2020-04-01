// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Contains data relevant to the `SIGCHLD` signal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChildData
{
	/// Child process identifier.
	pid: pid_t,

	/// Child user identifier.
	uid: uid_t,

	///  The child's exit code or exit code with signal code; use `status & 0x7F` to separate.
	status: i32,

	/// User CPU time consumed in 'clock ticks'.
	///
	/// The number of 'clock ticks' per second can be found by calling `sysconf(_SC_CLK_TCK)`.
	/// This is always `100` for the musl libc.
	user_cpu_time_consumed_in_clock_ticks: u64,

	/// System CPU time consumed in 'clock ticks'.
	///
	/// The number of 'clock ticks' per second can be found by calling `sysconf(_SC_CLK_TCK)`.
	/// This is always `100` for the musl libc.
	system_cpu_time_consumed_in_clock_ticks: u64,
}

impl ChildData
{
	#[inline(always)]
	pub(crate) fn new(ssi: &signalfd_siginfo) -> Self
	{
		Self
		{
			pid: ssi.ssi_pid,
			uid: ssi.ssi_uid,
			status: ssi.ssi_status,
			user_cpu_time_consumed_in_clock_ticks: ssi.ssi_utime,
			system_cpu_time_consumed_in_clock_ticks: ssi.ssi_stime,
		}
	}
}

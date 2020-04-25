// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Round-robin quantum adjustment.
///
/// Default is 100 milliseconds.
///
/// Only applies to `Scheduler::RoundRobin`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct RoundRobinQuantumMilliseconds(NonZeroU64);

impl RoundRobinQuantumMilliseconds
{
	/// Read.
	#[inline(always)]
	pub fn read(proc_path: &ProcPath) -> io::Result<Self>
	{
		let value = Self::file_path(proc_path).read_value()?;
		Ok(Self(value))
	}

	/// Write.
	#[inline(always)]
	pub fn write(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		Self::file_path(proc_path).write_value(self.0)
	}

	/// Reset.
	#[inline(always)]
	pub fn reset_to_default(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		Self::file_path(proc_path).write_value(0u64)
	}

	#[inline(always)]
	fn file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("sched_rr_timeslice_ms")
	}
}

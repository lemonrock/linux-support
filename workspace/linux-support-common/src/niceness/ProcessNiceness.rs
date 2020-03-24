// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Nice settings for the current process.
///
/// Defaults to aggresive promotion of the current process.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ProcessNiceness
{
	/// Downgrade all other processes for the current user to this value.
	pub all_other_processes_for_current_user: Nice,

	/// Downgrade all other processes in the process group to this value.
	pub all_other_processes_in_process_group: Option<Nice>,

	/// If autogroups are enabled, should we take as close to 100% of all CPU cycles in the autogroup?
	pub share_of_cpu_cycles_in_autogroup: Option<Nice>,
}

impl Default for ProcessNiceness
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			all_other_processes_for_current_user: Nice::Positive_19,
			all_other_processes_in_process_group: Some(Nice::Positive_19),
			share_of_cpu_cycles_in_autogroup: Some(Nice::Negative_20),
		}
	}
}

impl ProcessNiceness
{
	/// Is autogroup active? (from `/proc/sys/kernel/sched_autogroup_enabled`).
	#[inline(always)]
	pub fn is_autogroup_active(proc_path: &ProcPath) -> Result<bool, io::Error>
	{
		let value = Self::sched_autogroup_enabled_file_path(proc_path).read_raw_without_line_feed()?;
		match &value[..]
		{
			b"0" => Ok(false),
			b"1" => Ok(true),
			_ => Err(io::Error::from(ErrorKind::InvalidData)),
		}
	}

	/// Enable the autogroup feature (requires root).
	#[inline(always)]
	pub fn adjust_autogroup(proc_path: &ProcPath, enable_if_true_disable_if_false: bool) -> Result<(), io::Error>
	{
		Self::sched_autogroup_enabled_file_path(proc_path).write_value(enable_if_true_disable_if_false)
	}

	/// Adjusts in favour of the current process.
	#[allow(unused_variables)]
	pub fn adjust(&self, proc_path: &ProcPath) -> Result<(), ProcessNicenessAdjustmentError>
	{
		use self::ProcessNicenessAdjustmentError::*;

		if let Err(_) = self.all_other_processes_for_current_user.set_current_real_effective_user_priority()
		{
			return Err(CouldNotSetCurrentRealEffectiveUserPriorityNiceness)
		}

		if let Some(all_other_processes_in_process_group) = self.all_other_processes_in_process_group
		{
			if let Err(_) = all_other_processes_in_process_group.set_current_process_group_priority()
			{
				return Err(CouldNotSetCurrentProcessGroupPriorityNiceness)
			}
		}

		Nice::set_autogroup_for_current_process_if_desired(self.share_of_cpu_cycles_in_autogroup, proc_path)?;

		Ok(())
	}

	#[inline(always)]
	fn sched_autogroup_enabled_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.file_path("sys/kernel/sched_autogroup_enabled")
	}
}

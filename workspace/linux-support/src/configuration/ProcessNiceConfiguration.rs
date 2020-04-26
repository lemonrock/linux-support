// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Nice settings for the current process.
///
/// Defaults to aggresive promotion of the current process.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProcessNiceConfiguration
{
	/// If autogroups are enabled, should we take as close to 100% of all CPU cycles in the autogroup?
	pub share_of_cpu_cycles_in_autogroup: Option<Nice>,

	/// Downgrade all other processes for the current user to this value.
	pub all_other_processes_for_current_user: Option<Nice>,

	/// Downgrade all other processes in the process group to this value.
	pub all_other_processes_in_process_group: Option<Nice>,

	/// Current process non-real-time priority (nice value).
	pub current_process_priority: Option<Nice>,
}

impl Default for ProcessNiceConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			share_of_cpu_cycles_in_autogroup: Some(Nice::Negative_20),
			all_other_processes_for_current_user: Some(Nice::Positive_19),
			all_other_processes_in_process_group: Some(Nice::Positive_19),
			current_process_priority: Some(Nice::Negative_20),
		}
	}
}

impl ProcessNiceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), ProcessNiceConfigurationError>
	{
		use self::ProcessNiceConfigurationError::*;

		if let Some(share_of_cpu_cycles_in_autogroup) = self.share_of_cpu_cycles_in_autogroup
		{
			Self::set_autogroup_for_current_process_if_desired(share_of_cpu_cycles_in_autogroup, proc_path)?;
		}

		if let Some(all_other_processes_for_current_user) = self.all_other_processes_for_current_user
		{
			all_other_processes_for_current_user.set_current_user_priority().map_err(|_: ()| CouldNotSetCurrentUserPriorityNice)
		}

		if let Some(all_other_processes_in_process_group) = self.all_other_processes_in_process_group
		{
			all_other_processes_in_process_group.set_current_process_group_priority().map_err(|_: ()| CouldNotSetCurrentProcessGroupPriorityNice)
		}

		if let Some(current_process_priority) = self.current_process_priority
		{
			current_process_priority.current_process_priority().map_err(|_: ()| CouldNotSetCurrentProcessPriorityNice)
		}

		Ok(())
	}

	/// Set the autogroup for the current process.
	#[inline(always)]
	fn set_autogroup_for_current_process_if_desired(share_of_cpu_cycles_in_autogroup: Nice, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		if Self::is_autogroup_active(proc_path)?
		{
			share_of_cpu_cycles_in_autogroup.set_autogroup_for_current_process(proc_path)
		}
		else
		{
			Ok(())
		}
	}
	#[inline(always)]
	fn is_autogroup_active(proc_path: &ProcPath) -> Result<bool, io::Error>
	{
		Self::sched_autogroup_enabled_file_path(proc_path).read_value()
	}

	#[inline(always)]
	fn sched_autogroup_enabled_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("sched_autogroup_enabled")
	}
}

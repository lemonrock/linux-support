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
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ionice settings for the current process.
///
/// Defaults to aggressive promotion of the current process.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProcessIoPriorityConfiguration
{
	/// Downgrade all other processes for the current user to this value.
	///
	/// Requires capability `CAP_SYS_NICE`.
	pub all_other_processes_for_current_user: Option<IoPriority>,

	/// Downgrade all other processes in the process group to this value.
	///
	/// Requires capability `CAP_SYS_NICE`.
	pub all_other_processes_in_process_group: Option<IoPriority>,

	/// Current process priority (ionice value).
	pub current_process_priority: Option<IoPriority>,
}

impl Default for ProcessIoPriorityConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::IoPriority::*;
		Self
		{
			all_other_processes_for_current_user: Some(Idle),
			all_other_processes_in_process_group: Some(Idle),
			current_process_priority: Some(RealTime(RealTimeOrBestEffortIoPriorityLevel::_0)),
		}
	}
}

impl ProcessIoPriorityConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self) -> Result<(), ProcessIoPriorityConfigurationError>
	{
		#[inline(always)]
		fn set_io_priority_value(nice_value: Option<IoPriority>, function: impl FnOnce(IoPriority) -> Result<(), bool>, error: impl FnOnce(bool) -> ProcessIoPriorityConfigurationError) -> Result<(), ProcessIoPriorityConfigurationError>
		{
			if let Some(nice_value) = nice_value
			{
				function(nice_value).map_err(|cause| error(cause))
			}
			else
			{
				Ok(())
			}
		}

		use self::ProcessIoPriorityConfigurationError::*;

		set_io_priority_value(self.all_other_processes_for_current_user, |io_priority| io_priority.set_for_user(UserIdentifier::default()), CouldNotSetCurrentUserIoPriority)?;
		set_io_priority_value(self.all_other_processes_in_process_group, |io_priority| io_priority.set_for_process_group(ProcessGroupIdentifierChoice::Current), CouldNotSetCurrentProcessGroupIoPriority)?;
		set_io_priority_value(self.current_process_priority, |io_priority| io_priority.set_for_process(ProcessIdentifierChoice::Current), CouldNotSetCurrentProcessIoPriority)?;

		Ok(())
	}
}

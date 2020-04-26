// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process scheduling configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProcessSchedulingConfiguration
{
	/// Process niceness and its relationship to other processes of the same process group and user.
	pub process_nice_configuration: ProcessNiceConfiguration,

	/// Sets the scheduler policy before any threads other than main are created.
	pub process_scheduler: PerThreadSchedulerPolicyAndFlags,
}

impl ProcessSchedulingConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), ProcessSchedulingConfigurationError>
	{
		self.process_nice_configuration.configure(proc_path)?;

		self.process_scheduler.set_for_thread(ThreadIdentifierChoice::Other(ProcessIdentifierChoice::Current.thread_identifier())).map_err(|reason| ProcessSchedulingConfigurationError::CouldNotSetSchedulerPolicyAndFlags(reason))
	}
}

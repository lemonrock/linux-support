// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global memory statistics configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalMemoryStatisticsConfiguration
{
	/// Requires root.
	///
	/// Default is `1` second.
	///
	/// Can be set as high as 1200 or more.
	pub time_interval_in_seconds_between_statistics_updates: Option<NonZeroU16>,
	
	/// Requires root.
	///
	/// Default is true.
	pub enable_numa_statistics: Option<bool>,
}

impl GlobalMemoryStatisticsConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalMemoryStatisticsConfigurationError>
	{
		use self::GlobalMemoryStatisticsConfigurationError::*;
		
		set_proc_sys_vm_value(proc_path, "stat_interval", self.time_interval_in_seconds_between_statistics_updates.map(|value| UnpaddedDecimalInteger(value)), CouldNotChangeTimeIntervalInSecondsBetweenStatisticsUpdates)?;
		set_proc_sys_vm_value(proc_path, "numa_stat", self.enable_numa_statistics, CouldNotChangeNumaStatisticsEnablement)?;
		
		Ok(())
	}
}

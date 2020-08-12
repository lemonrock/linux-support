// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global NUMA balancing configuration.
///
/// Only used if NUMA balancing is enabled.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNumaBalancingOnConfiguration
{
	/// The amount of CPU time a thread must consume before its data is scanned.
	///
	/// This prevents creating overhead because of short-lived processes.
	///
	/// Requires root.
	pub scan_delay: Option<Milliseconds>,
	
	/// Controls how frequently a task's data is scanned.
	///
	/// Depending on the locality of the faults the scan rate will increase or decrease.
	///
	/// This setting controls the minimum scan rate.
	///
	/// Requires root.
	pub scan_period_minimum: Option<Milliseconds>,
	
	/// Controls how frequently a task's data is scanned.
	///
	/// Depending on the locality of the faults the scan rate will increase or decrease.
	///
	/// This setting controls the maximum scan rate.
	///
	/// Requires root.
	pub scan_period_maximum: Option<Milliseconds>,
	
	/// Controls how much address space is scanned when the task scanner is active.
	///
	/// Requires root.
	pub scan_size: Option<Megabytes>,
}

impl GlobalNumaBalancingOnConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalNumaBalancingOnConfigurationError>
	{
		use self::GlobalNumaBalancingOnConfigurationError::*;
		
		set_proc_sys_kernel_value(proc_path, "numa_balancing_scan_delay_ms", self.scan_delay, CouldNotChangeScanDelay)?;
		set_proc_sys_kernel_value(proc_path, "numa_balancing_scan_period_min_ms", self.scan_period_minimum, CouldNotChangeScanPeriodMinimum)?;
		set_proc_sys_kernel_value(proc_path, "numa_balancing_scan_period_max_ms", self.scan_period_maximum, CouldNotChangeScanPeriodMaximum)?;
		set_proc_sys_kernel_value(proc_path, "numa_balancing_scan_size_mb", self.scan_size, CouldNotChangeScanSize)?;
		
		Ok(())
	}
}

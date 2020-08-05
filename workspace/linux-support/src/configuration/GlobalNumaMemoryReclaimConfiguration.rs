// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global out-of-memory configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNumaMemoryReclaimConfiguration
{
	/// Defaults to `Off`, but `On` may be advantageous; also `On` can occur on some high-end boards and may need to be tuned to `Off`.
	///
	/// It's a bit of hack, really.
	///
	/// Requires root.
	pub numa_zone_reclaim_mode: Option<NumaZoneReclaimMode>,
	
	/// The default is 5% (`5`).
	///
	/// Requires root.
	pub minimum_slab_percentage: Option<Percentage>,
	
	/// The default is 1% (`1`).
	///
	/// Requires root.
	pub minimum_unmapped_percentage: Option<Percentage>,
}

impl GlobalNumaMemoryReclaimConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalNumaMemoryReclaimConfigurationError>
	{
		use self::GlobalNumaMemoryReclaimConfigurationError::*;

		set_value(proc_path, |proc_path, value| value.set(proc_path), self.numa_zone_reclaim_mode, CouldNotChangeNumaZoneReclaimMode)?;
		set_value(proc_path, |proc_path, value| value.set(proc_path), self.minimum_slab_percentage, CouldNotChangeMinimumSlabPercentage)?;
		set_value(proc_path, |proc_path, value| value.set(proc_path), self.minimum_unmapped_percentage, CouldNotChangeMinimumUnmappedPercentage)?;
		Ok(())
	}
}

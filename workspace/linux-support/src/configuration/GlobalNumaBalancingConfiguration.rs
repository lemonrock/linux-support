// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global NUMA balancing configuration.
///
/// Only used if NUMA balancing is enabled.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub enum GlobalNumaBalancingConfiguration
{
	/// Off.
	Off,

	/// On.
	On(GlobalNumaBalancingOnConfiguration),
}

impl GlobalNumaBalancingOnConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalNumaBalancingConfigurationError>
	{
		use self::GlobalNumaBalancingConfiguration::*;
		
		match self
		{
			Off => Self::configure_enablement(proc_path, false)?,
			
			On(ref numa_balancing_on_configuration) =>
			{
				Self::configure_enablement(proc_path, true)?;
				numa_balancing_on_configuration.configure(proc_path)?;
			},
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn configure_enablement(proc_path: &ProcPath, value: bool) -> Result<(), GlobalNumaBalancingConfigurationError>
	{
		set_proc_sys_kernel_value(proc_path, "numa_balancing", Some(value), GlobalNumaBalancingConfigurationError::CouldNotChangeEnablement)
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Linux Kernel Same Page Merging (KSM) configuration
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalLinuxKernelSamePageMergingConfiguration
{
	/// Requires root.
	///
	/// Default is to disable KSM.
	pub enable: Option<bool>,
}

impl GlobalLinuxKernelSamePageMergingConfiguration
{
	/// Configures.
	pub fn configure(&self, sys_path: &SysPath) -> Result<(), GlobalLinuxKernelSamePageMergingConfigurationError>
	{
		use self::GlobalLinuxKernelSamePageMergingConfigurationError::*;

		if let Some(enable) = self.enable
		{
			sys_path.ksm_file_path("run").write_value(enable).map_err(CouldNotChangeEnablement)
		}
		else
		{
			Ok(())
		}
	}
}

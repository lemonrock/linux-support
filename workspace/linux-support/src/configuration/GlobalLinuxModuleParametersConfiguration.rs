// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux module parameters configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct GlobalLinuxModuleParametersConfiguration(HashMap<LinuxKernelModuleName, HashMap<LinuxKernelModuleParameterName, ModuleParameterValueChoice>>);

impl GlobalLinuxModuleParametersConfiguration
{
	/// Configures.
	pub fn configure(&self, sys_path: &SysPath) -> Result<(), GlobalLinuxModuleConfigurationError>
	{
		for (linux_kernel_module_name, parameters) in self.0.iter()
		{
			for (parameter_name, parameter_value) in parameters.iter()
			{
				parameter_value.write(sys_path, linux_kernel_module_name, &parameter_name).map_err(|cause| GlobalLinuxModuleConfigurationError::CouldNotConfigureModuleParameter
				{
					linux_kernel_module_name: linux_kernel_module_name.clone(),
					parameter_name: parameter_name.clone(),
					cause,
				})?
			}
		}
		
		Ok(())
	}
}

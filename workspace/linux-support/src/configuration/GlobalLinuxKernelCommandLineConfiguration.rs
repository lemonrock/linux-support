// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Linux command line configuration.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalLinuxKernelCommandLineConfiguration
{
	/// Check for incompatible settings.
	#[serde(default = "GlobalLinuxKernelCommandLineConfiguration::check_for_incompatible_settings_default")] pub check_for_incompatible_settings: bool,

	/// Check for incorrect huge page settings.
	#[serde(default = "GlobalLinuxKernelCommandLineConfiguration::check_for_huge_page_size_settings_default")] pub check_for_huge_page_size_settings: bool,

	/// Optional checks to suppress.
	#[serde(default = "GlobalLinuxKernelCommandLineConfiguration::optional_kernel_command_line_setting_check_to_suppress_default")] pub optional_kernel_command_line_setting_check_to_suppress: HashSet<OptionalKernelCommandLineSettingCheck>,
}

impl Default for GlobalLinuxKernelCommandLineConfiguration
{
	fn default() -> Self
	{
		Self
		{
			check_for_incompatible_settings: Self::check_for_incompatible_settings_default(),
			check_for_huge_page_size_settings: Self::check_for_huge_page_size_settings_default(),
			optional_kernel_command_line_setting_check_to_suppress: Self::optional_kernel_command_line_setting_check_to_suppress_default(),
		}
	}
}

impl GlobalLinuxKernelCommandLineConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalLinuxKernelCommandLineConfigurationError>
	{
		use self::GlobalLinuxKernelCommandLineConfigurationError::*;

		let linux_kernel_command_line_parameters = LinuxKernelCommandLineParameters::parse(proc_path).map_err(|cause| CouldNotParseLinuxKernelCommandLineParameters(cause))?;

		if self.check_for_incompatible_settings
		{
			incompatible_settings(&linux_kernel_command_line_parameters).map_err(|reason| IncompatibleSettings(reason))?;
		}
		if self.check_for_huge_page_size_settings
		{
			validate_huge_page_sizes(&linux_kernel_command_line_parameters, cpu_supports_1gb_pages()).map_err(|reason| InvalidHugePageSizes(reason))?;
		}

		OptionalKernelCommandLineSettingCheck::run_all_checks(&self.optional_kernel_command_line_setting_check_to_suppress, &linux_kernel_command_line_parameters).map_err(|cause| OptionalLinuxKernelCommandLineSettingChecksFailed(cause))?;

		Ok(())
	}

	#[inline(always)]
	fn optional_kernel_command_line_setting_check_to_suppress_default() -> HashSet<OptionalKernelCommandLineSettingCheck>
	{
		use self::OptionalKernelCommandLineSettingCheck::*;
		hashset!
		{
			hashdist,
			noaliencache,
			skew_tick,
			numa_zonelist_order,
			#[cfg(target_arch = "x86_64")] noxsaveopt,
			#[cfg(target_arch = "x86_64")] idle_poll,
		}
	}

	#[inline(always)]
	const fn check_for_incompatible_settings_default() -> bool
	{
		true
	}

	#[inline(always)]
	const fn check_for_huge_page_size_settings_default() -> bool
	{
		true
	}
}

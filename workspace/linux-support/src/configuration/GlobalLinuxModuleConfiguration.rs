// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global linux module configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalLinuxModuleConfiguration
{
	/// Default is `/sbin/modprobe`.
	///
	/// Verifies this path exists before setting it.
	pub modprobe_executable_path: Option<PathBuf>,

	/// Modules to unload.
	///
	/// Does not use `modprobe`, so the order needs to be respectful of dependencies.
	///
	/// Will not be unloaded if module unloading is disabled by the kernel.
	///
	/// Unaffected by the setting of `disable_module_loading_and_unloading_until_reboot` (unless this application has been re-run).
	pub linux_kernel_modules_to_unload: IndexSet<LinuxKernelModuleName>,

	/// Modules to load; uses `modprobe`.
	///
	/// Location of `modprobe` is affected by setting of `modprobe_executable_path`; regardless, `modprobe` is *not* looked for on the `PATH` but using `LinuxKernelModuleName::modprobe_executable_path()` which reads `/proc/sys/kernel/modprobe`.
	///
	/// Will not be loaded if module loading is disabled by the kernel.
	///
	/// Unaffected by the setting of `disable_module_loading_and_unloading_until_reboot` (unless this application has been re-run).
	pub linux_kernel_modules_to_load: IndexSet<LinuxKernelModuleName>,

	/// Disables module loading and unloading until reboot.
	pub disable_module_loading_and_unloading_until_reboot: bool,
}

impl GlobalLinuxModuleConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalLinuxModuleConfigurationError>
	{
		use self::GlobalLinuxModuleConfigurationError::*;

		set_value(proc_path, LinuxKernelModuleName::set_modprobe_executable_path, self.modprobe_executable_path, CouldNotChangeModprobeExecutablePath)?;

		let potentially_have_modules_to_unload = !self.linux_kernel_modules_to_unload.is_empty();
		let potentially_have_modules_to_load = !self.linux_kernel_modules_to_load.is_empty();

		if potentially_have_modules_to_unload || potentially_have_modules_to_load
		{
			if !self.linux_kernel_modules_to_unload.is_disjoint(&self.linux_kernel_modules_to_load)
			{
				return Err(ReloadingLinuxKernelModulesIsUnsupported)
			}

			let loaded_modules = LinuxKernelModulesList::parse(proc_path)?;

			if LinuxKernelModuleName::is_module_loading_and_unloading_disabled(proc_path)
			{
				if loaded_modules.contains_any_of(self.linux_kernel_modules_to_unload.iter())
				{
					return Err(CouldNotUnloadLinuxKernelModuleBecauseModuleUnloadingIsDisabled)
				}
				if loaded_modules.does_not_contain_all_of(self.linux_kernel_modules_to_load.iter())
				{
					return Err(CouldNotLoadLinuxKernelModuleBecauseModuleLoadingIsDisabled)
				}
			}
			else
			{
				for linux_kernel_module_to_unload in self.linux_kernel_modules_to_unload.iter()
				{
					let _unloaded = linux_kernel_module_to_unload.unload_linux_kernel_module().map_err(|cause| CouldNotUnloadLinuxKernelModule(cause))?;
				}

				if let Some(modprobe_path) = LinuxKernelModuleName::modprobe_executable_path(proc_path)
				{
					for linux_kernel_module_to_load in self.linux_kernel_modules_to_load.iter()
					{
						linux_kernel_module_to_load.load_linux_kernel_module_using_modprobe()?
					}
				}
			}
		}

		set_value_once(proc_path, LinuxKernelModuleName::disable_module_loading_and_unloading_until_reboot, self.disable_module_loading_and_unloading_until_reboot, CouldNotDisableModuleLoadingAndUnloadingUntilNextReboot)
	}
}

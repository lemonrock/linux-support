// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global security configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSecurityConfiguration
{
	/// If `true`, then the following in `/proc/sys/kernel` are hardened if present:-
	///
	/// * `randomize_va_space`
	/// * `sysrq`
	/// * `stack_erasing`
	/// * `kptr_restrict`
	/// * `dmesg_restrict`
	/// * `suid_dumpable`
	/// * `protected_symlinks`
	/// * `protected_hardlinks`
	/// * `protected_fifos`
	/// * `protected_regular`
	pub harden: bool,

	/// Disables kexec loading of new kernel images until reboot.
	///
	/// By default it is enabled.
	pub disable_kexec_loading_of_new_kernel_images_until_reboot: bool,
}

impl GlobalSecurityConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalSecurityConfigurationError>
	{
		use self::GlobalSecurityConfigurationError::*;

		set_value_once
		(
			proc_path,
			|proc_path|
			{
				let file_path = proc_path.sys_kernel_file_path("kexec_load_disabled");
				if file_path.exists()
				{
					let enabled: bool = file_path.read_value().unwrap();
					if !enabled
					{
						return file_path.write_value(true)
					}
				}
				Ok(())
			},
			self.disable_module_loading_and_unloading_until_reboot,
			CouldNotDisableKexecLoadingUntilNextReboot
		)
	}
}

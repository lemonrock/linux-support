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
	/// * `protected_symlinks`
	/// * `protected_hardlinks`
	/// * `protected_fifos`
	/// * `protected_regular`
	///
	/// And the following in `/proc/sys/fs` are hardened if present:-
	///
	/// * `suid_dumpable`
	/// * `protected_symlinks`
	/// * `protected_regular`
	/// * `protected_hardlinks`
	/// * `protected_fifos`
	///
	/// And the following in `/proc/sys/vm` are hardened if present:-
	///
	/// * `unprivileged_userfaultfd`.
	///
	/// And the maximum number of process identifiers is set to 2^22 in `/proc/sys/kernel/pid_max` to reduce the impact of races and process identifier wrap-around (Frankly, they should just be an UUID and be done with it).
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

		#[inline(always)]
		fn harden_value<'a>(proc_path: &ProcPath, file_function: impl FnOnce(&ProcPath, &str) -> PathBuf, file_name: &'static str, value: u8) -> Result<(), GlobalSecurityConfigurationError>
		{
			let file_path = file_function(proc_path, file_name);
			if file_path.exists()
			{
				return file_path.write_value(UnpaddedDecimalInteger(value)).map_err(|cause| CouldNotHarden { cause, proc_sys_kernel_file: file_name })
			}
			Ok(())
		}

		if self.harden
		{
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "randomize_va_space", 2)?;
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "sysrq", 0)?;
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "stack_erasing", 1)?;
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "kptr_restrict", 2)?;
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "dmesg_restrict", 1)?;
			harden_value(proc_path, ProcPath::sys_fs_file_path, "suid_dumpable", 0)?;
			harden_value(proc_path, ProcPath::sys_fs_file_path, "protected_symlinks", 1)?;
			harden_value(proc_path, ProcPath::sys_fs_file_path, "protected_regular", 2)?;
			harden_value(proc_path, ProcPath::sys_fs_file_path, "protected_hardlinks", 1)?;
			harden_value(proc_path, ProcPath::sys_fs_file_path, "protected_fifos", 2)?;
			harden_value(proc_path, ProcPath::sys_vm_file_path, "unprivileged_userfaultfd", 0)?;
			ProcessIdentifier::set_maximum_value_to_maximum(proc_path).map_err(CouldNotSetMaximumProcessIdentifiersToMaximum)?;
		}

		set_value_once
		(
			proc_path,
			|proc_path|
			{
				let file_path = proc_path.sys_kernel_file_path("kexec_load_disabled");
				if file_path.exists()
				{
					let enabled: bool = file_path.read_zero_or_one_bool().unwrap();
					if !enabled
					{
						return file_path.write_value(true)
					}
				}
				Ok(())
			},
			self.disable_kexec_loading_of_new_kernel_images_until_reboot,
			CouldNotDisableKexecLoadingUntilNextReboot
		)
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global BPF configuration.
///
/// JIT hardening is configured using `GlobalSecurityConfiguration`.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalBpfConfiguration
{
	/// Specify `true` if using pinned objects and the `BpfMountPoint`.
	///
	/// Requires root.
	pub mount_file_system_for_pinning_objects_if_not_already_mounted: bool,
	
	/// Normally needs to be overridden.
	///
	/// Requires root.
	pub enable_just_in_time_compilation: Option<JustInTimeCompilationChoice>,

	/// Requires root.
	pub global_limit_for_memory_allocation: Option<JustInTimeMemoryAllocationLimitSizeInBytes>,
}

impl GlobalBpfConfiguration
{
	/// Configures.
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalBpfConfigurationError>
	{
		use self::GlobalBpfConfigurationError::*;
		
		if self.mount_file_system_for_pinning_objects_if_not_already_mounted
		{
			let mounts = Mounts::parse(proc_path, ProcessIdentifierChoice::Current).map_err(CouldNotParseMounts)?;
			let _mount_point = mounts.mount_if_not_mounted::<BpfMountPoint>(sys_path).map_err(CouldNotMount)?;
		}
		
		instance_set_value(proc_path, JustInTimeCompilationChoice::set_value, self.enable_just_in_time_compilation, CouldNotChangeJustInTimeCompilation)?;
		
		instance_set_value(proc_path, JustInTimeMemoryAllocationLimitSizeInBytes::set_global_maximum, self.global_limit_for_memory_allocation, CouldNotChangeJustInTimeCompilationMemoryLimit)?;
		
		Ok(())
	}
}

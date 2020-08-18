// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global cgroup configuration.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalCgroupConfiguration
{
	/// Root cgroup configuration.
	#[serde(flatten)] pub root_cgroup_configuration: RootCgroupConfiguration,
}

impl GlobalCgroupConfiguration
{
	/// Configures.
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(), GlobalCgroupConfigurationError>
	{
		use self::GlobalCgroupConfigurationError::*;
		
		let mounts = Mounts::parse(proc_path, ProcessIdentifierChoice::Current).map_err(CouldNotParseMounts)?;
		
		let mount_point = mounts.mount_if_not_mounted::<CgroupMountPoint>(sys_path).map_err(CouldNotMount)?;
		
		self.root_cgroup_configuration.configure(&mount_point, defaults).map_err(CouldNotChange)
	}
}

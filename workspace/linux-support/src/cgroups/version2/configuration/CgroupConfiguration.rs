// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Cgroup configuration.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CgroupConfiguration
{
	/// Root cgroup configuration.
	#[serde(flatten)] pub root_cgroup_configuration: RootCgroupConfiguration,
}

impl CgroupConfiguration
{
	/// Configures.
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> io::Result<()>
	{
		use self::CgroupConfigurationError::*;
		
		let mounts = Mounts::parse(proc_path, ProcessIdentifierChoice::Current)?;
		
		use self::FileSystemType::cgroup2;
		
		let mount_point = match mounts.existing_mount(cgroup2)
		{
			None =>
			{
				let mount_point = CgroupMountPoint { path: sys_path.file_system_fs_folder_path(cgroup2) };
				RootCgroup::create_and_mount(&mount_point)?;
				mount_point
			}
			
			Some(path) => CgroupMountPoint { path: path.to_owned() },
		};
		
		self.root_cgroup_configuration.configure(&mount_point)
	}
}

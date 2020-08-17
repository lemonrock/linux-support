// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `mount -t cgroup2 none /sys/fs/cgroup2`.
///
/// The only useful mount option is `nsdelegate`; this isn't supported in this design, yet.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct CgroupMountPoint(PathBuf);

impl Default for CgroupMountPoint
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::default_sys_fs_path(&SysPath::default())
	}
}

impl VirtualFileSystemMountPoint for CgroupMountPoint
{
	const FileSystemType: FileSystemType = FileSystemType::cgroup2;
	
	#[inline(always)]
	fn to_path(&self) -> &Path
	{
		&self.0
	}
	
	#[inline(always)]
	fn from_path(path: PathBuf) -> Self
	{
		Self(path)
	}
}

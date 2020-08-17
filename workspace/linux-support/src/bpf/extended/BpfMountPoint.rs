// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `mount -t cgroup2 none /sys/fs/bpf`.
///
/// Used for pinned objects.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BpfMountPoint(PathBuf);

impl Default for BpfMountPoint
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::default_sys_fs_path(&SysPath::default())
	}
}

impl VirtualFileSystemMountPoint for BpfMountPoint
{
	const FileSystemType: FileSystemType = FileSystemType::bpf;
	
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

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `mount -t cgroup2 none /mnt/cgroup2`.
///
/// The only supported mount option is `nsdelegate`.
pub struct CgroupMountPoint
{
	path: PathBuf,
}

impl CgroupMountPoint
{
	/// To a Path.
	#[inline(always)]
	pub fn to_path(&self) -> &Path
	{
		&self.path
	}
}

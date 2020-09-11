// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `ProcessIdentifierChoice` or `ThreadIdentifierChoice`.
pub trait ProcessOrThreadIdentifierChoice: Default + Debug + Copy + Ord + Hash
{
	/// Migrate.
	fn migrate<C: Cgroup>(self, mount_point: &CgroupMountPoint, cgroup: &Rc<C>) -> io::Result<()>;
}

impl ProcessOrThreadIdentifierChoice for ProcessIdentifierChoice
{
	#[inline(always)]
	fn migrate<C: Cgroup>(self, mount_point: &CgroupMountPoint, cgroup: &Rc<C>) -> io::Result<()>
	{
		cgroup.migrate_process_to_this_cgroup(mount_point, self)
	}
}

impl ProcessOrThreadIdentifierChoice for ThreadIdentifierChoice
{
	#[inline(always)]
	fn migrate<C: Cgroup>(self, mount_point: &CgroupMountPoint, cgroup: &Rc<C>) -> io::Result<()>
	{
		cgroup.migrate_thread_to_this_cgroup(mount_point, self)
	}
}

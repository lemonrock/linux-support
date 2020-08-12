// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Migration.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Migration<POTIC: ProcessOrThreadIdentifierChoice>(Vec<POTIC>);

impl<POTIC: ProcessOrThreadIdentifierChoice> Migration<POTIC>
{
	fn leaf_migrate<C: Cgroup>(&self, mount_point: &CgroupMountPoint, cgroup: &C) -> io::Result<()>
	{
		cgroup.write_maximum_descendants(mount_point, MaximumNumber::Finite(0))?;
		
		migration.migrate(mount_point, &cgroup);
		
		Ok(())
	}
	
	fn migrate<C: Cgroup>(&self, mount_point: &CgroupMountPoint, cgroup: &C)
	{
		for potic in self.0.iter()
		{
			let _ignored_as_may_have_exited = potic.migrate(mount_point, cgroup);
		}
	}
}

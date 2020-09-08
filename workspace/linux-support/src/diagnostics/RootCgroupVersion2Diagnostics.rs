// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootCgroupVersion2Diagnostics
{
	#[serde(flatten)]
	pub common: CommonCgroupVersion2Diagnostics,
}

impl RootCgroupVersion2Diagnostics
{
	#[inline(always)]
	fn gather(file_systems: &FileSystemsDiagnostics, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> Option<Self>
	{
		file_systems.cgroup_version2_mount_path().map(|mount_path|
		{
			let mount_point = CgroupMountPoint::from_path(mount_path.to_path_buf());
			
			let root_cgroup = Rc::new(RootCgroup);
			
			Self
			{
				common: CommonCgroupVersion2Diagnostics::gather(&mount_point, &root_cgroup, supported_huge_page_sizes),
			}
		})
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `hugetlb` controller configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct PerHugePageSizeHugetlbControllerConfiguration
{
	/// Hard limit for `hugepagesize` usage.
	pub maximum: MaximumNumber<usize>,
	
	/// Hard limit for `reserved` usage.
	pub reserved_maximum: MaximumNumber<usize>,
}

impl PerHugePageSizeHugetlbControllerConfiguration
{
	#[inline(always)]
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>, huge_page_size: HugePageSize) -> io::Result<()>
	{
		cgroup.write_hugetlb_maximum(mount_point, huge_page_size, self.maximum)?;
		cgroup.write_hugetlb_reserved_maximum(mount_point, huge_page_size, self.reserved_maximum)?;
		Ok(())
	}
}

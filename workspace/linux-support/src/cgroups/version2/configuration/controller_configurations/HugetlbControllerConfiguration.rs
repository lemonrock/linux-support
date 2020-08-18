// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `hugetlb` controller configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct HugetlbControllerConfiguration(HashMap<HugePageSize, PerHugePageSizeHugetlbControllerConfiguration>);

impl ControllerConfiguration for HugetlbControllerConfiguration
{
	const Controller: Controller = Controller::hugetlb;
	
	#[inline(always)]
	fn configure<'name>(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup<'name>>, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		for (huge_page_size, per_huge_page_size_hugetlb_controller_configuration) in self.0.iter()
		{
			if defaults.is_supported_huge_page_size(huge_page_size)
			{
				per_huge_page_size_hugetlb_controller_configuration.configure(mount_point, cgroup, *huge_page_size)?;
			}
		}
		Ok(())
	}
}

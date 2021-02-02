// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Configuration of a child immediately below the root cgroup.
///
/// Special, because immediate children of the root cgroup can be either domains or threaded cgroups.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum BelowRootCgroupConfiguration
{
	/// Domain.
	Domain(DomainCgroupConfiguration),
	
	/// Threaded.
	Threaded(ThreadedCgroupConfiguration),
}

impl ChildCgroupConfiguration for BelowRootCgroupConfiguration
{
	#[inline(always)]
	fn configure<C: Cgroup>(&self, mount_point: &CgroupMountPoint, parent: &Rc<C>, name: &CgroupName, defaults: &DefaultHugePageSizes) -> io::Result<()>
	{
		use self::BelowRootCgroupConfiguration::*;
		
		match self
		{
			&Domain(ref domain_cgroup_configuration) => domain_cgroup_configuration.configure(mount_point, parent, name, defaults),
			
			&Threaded(ref threaded_cgroup_configuration) => threaded_cgroup_configuration.configure(mount_point, parent, name, defaults),
		}
	}
	
	#[inline(always)]
	fn desired_controllers_and_our_depth(&self) -> (&Controllers, usize)
	{
		use self::BelowRootCgroupConfiguration::*;
		
		match self
		{
			&Domain(ref domain_cgroup_configuration) => domain_cgroup_configuration.desired_controllers_and_our_depth(),
			
			&Threaded(ref threaded_cgroup_configuration) => threaded_cgroup_configuration.desired_controllers_and_our_depth(),
		}
	}
}

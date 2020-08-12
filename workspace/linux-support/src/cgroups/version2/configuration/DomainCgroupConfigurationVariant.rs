// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Domain cgroup configuration variant.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum DomainCgroupConfigurationVariant
{
	/// A `NonRootCgroupType::Domain` with children and no processes.
	Domain(ChildrenCgroupConfiguration<DomainCgroupConfiguration>),
	
	/// A `NonRootCgroupType::ThreadedDomain` with children that can only have threads.
	ThreadedDomain(ChildrenCgroupConfiguration<ThreadedCgroupConfiguration>),
	
	/// A `NonRootCgroupType::Domain` with no children and just proceses.
	Leaf(Migration<ProcessIdentifierChoice>),
}

impl Default for DomainCgroupConfigurationVariant
{
	#[inline(always)]
	fn default() -> Self
	{
		DomainCgroupConfigurationVariant::Leaf(Migration::default())
	}
}

impl CgroupConfigurationVariant for DomainCgroupConfigurationVariant
{
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: Rc<impl Cgroup>) -> io::Result<()>
	{
		use self::DomainCgroupConfigurationVariant::*;
		
		match self
		{
			&Domain(ref children) => children.configure_children(mount_point, &cgroup)?,
			
			&ThreadedDomain(ref children) => children.configure_children(mount_point, &cgroup)?,
			
			&Leaf(ref migration) => migration.leaf_migrate(mount_point, &cgroup)
		}
	}
	
	fn desired_controllers_and_our_depth(&self, mut all_desired_controllers_in_parent: Controllers) -> (Controllers, usize)
	{
		use self::DomainCgroupConfigurationVariant::*;
		
		match self
		{
			&Domain(ref children) => children.desired_controllers_and_our_depth_children(all_desired_controllers_in_parent),
			
			&ThreadedDomain(ref children) => children.desired_controllers_and_our_depth_children(all_desired_controllers_in_parent),
			
			&Leaf(_) => (all_desired_controllers_in_parent, 0),
		}
	}
	
	#[inline(always)]
	fn make_type_threaded_if_needed(mount_point: &CgroupMountPoint, cgroup: &Rc<impl Cgroup>) -> io::Result<()>
	{
		Ok(())
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Threaded cgroup configuration variant.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ThreadedCgroupConfigurationVariant
{
	/// A `NonRootCgroupType::Threaded` with children and no threads.
	Threaded(ChildrenCgroupConfiguration<ThreadedCgroupConfiguration>),
	
	/// A `NonRootCgroupType::Threaded` with no children and just threads.
	Leaf(Migration<ThreadIdentifierChoice>),
}

impl Default for ThreadedCgroupConfigurationVariant
{
	#[inline(always)]
	fn default() -> Self
	{
		ThreadedCgroupConfigurationVariant::Leaf(Migration::default())
	}
}

impl CgroupConfigurationVariant for ThreadedCgroupConfigurationVariant
{
	fn configure<C: Cgroup>(&self, mount_point: &CgroupMountPoint, cgroup: Rc<C>, defaults: &DefaultHugePageSizes) -> io::Result<()>
	{
		use self::ThreadedCgroupConfigurationVariant::*;
		
		match self
		{
			&Threaded(ref children) => children.configure_children(mount_point, &cgroup, defaults),
			
			&Leaf(ref migration) => migration.leaf_migrate(mount_point, &cgroup),
		}
	}
	
	fn desired_controllers_and_our_depth(&self, all_desired_controllers_in_parent: Controllers) -> (Controllers, usize)
	{
		use self::ThreadedCgroupConfigurationVariant::*;
		
		match self
		{
			&Threaded(ref children) => children.desired_controllers_and_our_depth_children(all_desired_controllers_in_parent),
			
			&Leaf(_) => (all_desired_controllers_in_parent, 0),
		}
	}
	
	#[inline(always)]
	fn make_type_threaded_if_needed(mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>) -> io::Result<()>
	{
		cgroup.make_type_threaded(mount_point)
	}
}

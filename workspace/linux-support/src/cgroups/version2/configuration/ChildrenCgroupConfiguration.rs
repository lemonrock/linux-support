// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Children.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ChildrenCgroupConfiguration<CCC: ChildCgroupConfiguration>(HashMap<CgroupName, CCC>);

impl<CCC: ChildCgroupConfiguration> Default for ChildrenCgroupConfiguration<CCC>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(HashMap::new())
	}
}

impl<CCC: ChildCgroupConfiguration> Deref for ChildrenCgroupConfiguration<CCC>
{
	type Target = HashMap<CgroupName, CCC>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<CCC: ChildCgroupConfiguration> DerefMut for ChildrenCgroupConfiguration<CCC>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<CCC: ChildCgroupConfiguration> ChildrenCgroupConfiguration<CCC>
{
	#[inline(always)]
	fn configure_children<C: Cgroup>(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<C>, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		cgroup.write_maximum_descendants(mount_point, MaximumNumber::Finite(self.len()))?;
		
		for (name, child_cgroup_configuration) in self.iter()
		{
			child_cgroup_configuration.configure(mount_point, cgroup, name, defaults)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn desired_controllers_and_our_depth_children(&self, mut all_desired_controllers_in_parent: Controllers) -> (Controllers, usize)
	{
		let mut our_depth = 0;
		for child_cgroup_configuration in self.values()
		{
			let (childs_desired_controllers_in_parent, childs_depth) = child_cgroup_configuration.desired_controllers_and_our_depth();
			all_desired_controllers_in_parent.merge(&childs_desired_controllers_in_parent);
			our_depth = max(our_depth, childs_depth + 1);
		}
		(all_desired_controllers_in_parent, our_depth)
	}
}

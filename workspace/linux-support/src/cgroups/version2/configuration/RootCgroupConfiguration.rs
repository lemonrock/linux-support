// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Root configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct RootCgroupConfiguration
{
	/// Children.
	pub children: ChildrenCgroupConfiguration<BelowRootCgroupConfiguration>,
	
	/// Processes.
	pub leaf_processes: Migration<ProcessIdentifierChoice>,
	
	/// Threads.
	pub leaf_threads: Migration<ThreadIdentifierChoice>,
}

impl RootCgroupConfiguration
{
	pub(crate) fn configure(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		let cgroup = Rc::new(RootCgroup);
		
		let (desired_controllers, maximum_depth) = self.desired_controllers_and_our_depth();
		
		let _available_controllers = cgroup.change_subtree_controllers_taking_account_of_those_available(mount_point, &desired_controllers)?;
		
		cgroup.write_maximum_depth(mount_point, MaximumNumber::Finite(maximum_depth))?;
		
		self.children.configure_children(mount_point, &cgroup)?;
		
		self.leaf_processes.migrate(mount_point, &cgroup);
		
		self.leaf_threads.migrate(mount_point, &cgroup);
		
		Ok(())
	}
	
	fn desired_controllers_and_our_depth(&self) -> (Controllers, usize)
	{
		self.children.desired_controllers_and_our_depth_children(Controllers::default())
	}
}

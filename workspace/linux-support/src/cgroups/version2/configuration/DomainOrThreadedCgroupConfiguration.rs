// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Domain or Threaded cgroup configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DomainOrThreadedCgroupConfiguration<CC: ControllersConfiguration, CCV: CgroupConfigurationVariant>
{
	/// Controllers desired and their configuration.
	#[serde(default)] pub desired_controllers: CC,
	
	/// Variant.
	#[serde(default)] pub variant: CCV,
	
	#[serde(skip)] cached_desired_controllers_and_our_depth: CachedDesiredControllersAndOurDepth,
}

impl<CC: ControllersConfiguration, CCV: CgroupConfigurationVariant> ChildCgroupConfiguration for DomainOrThreadedCgroupConfiguration<CC, CCV>
{
	fn configure<'name, C: 'name + Cgroup<'name>>(&self, mount_point: &CgroupMountPoint, parent: &Rc<C>, name: &'name CgroupName, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		let parent = Rc::clone(parent);
		let cgroup = parent.child(Cow::Borrowed(name));
		
		cgroup.create(mount_point)?;
		
		let (desired_controllers, maximum_depth) = self.desired_controllers_and_our_depth();
		let available_controllers = cgroup.change_subtree_controllers_taking_account_of_those_available(mount_point, desired_controllers)?;
		self.desired_controllers.configure(mount_point, &cgroup, &available_controllers, defaults)?;
		cgroup.write_maximum_depth(mount_point, MaximumNumber::Finite(maximum_depth))?;
		
		CCV::make_type_threaded_if_needed(mount_point, &cgroup)?;
		
		self.variant.configure(mount_point, cgroup, defaults)
	}
	
	fn desired_controllers_and_our_depth(&self) -> (&Controllers, usize)
	{
		self.cached_desired_controllers_and_our_depth.get(|| self.variant.desired_controllers_and_our_depth(self.desired_controllers.to_desired_controllers()))
	}
}

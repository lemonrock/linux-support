// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// All controllers configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct AllControllersConfiguration
{
	/// Threaded controllers.
	#[serde(flatten)] pub threaded: ThreadedControllersConfiguration,
	
	/// Domain controllers.
	#[serde(flatten)] pub domain: DomainControllersConfiguration,
}

impl ControllersConfiguration for AllControllersConfiguration
{
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>, available_controllers: &Controllers, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		self.threaded.configure(mount_point, cgroup, available_controllers, defaults)?;
		self.domain.configure(mount_point, cgroup, available_controllers, defaults)
	}
	
	fn to_desired_controllers(&self) -> Controllers
	{
		let mut controllers = self.threaded.to_desired_controllers();
		self.domain.add_to_desired_controllers(&mut controllers);
		
		controllers
	}
}

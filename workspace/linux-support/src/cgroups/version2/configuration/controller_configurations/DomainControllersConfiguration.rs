// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Domain controllers configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct DomainControllersConfiguration
{
	/// `io` controller configuration.
	pub io: Option<IoControllerConfiguration>,
	
	/// `memory` controller configuration.
	pub memory: Option<MemoryControllerConfiguration>,
	
	/// `rdma` controller configuration.
	pub rdma: Option<RdmaControllerConfiguration>,
	
	/// `hugetlb` controller configuration.
	pub hugetlb: Option<HugetlbControllerConfiguration>,
}

impl DomainControllersConfiguration
{
	fn configure<'name>(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup<'name>>, available_controllers: &Controllers, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		configure_controller(&self.io, mount_point, cgroup, available_controllers, defaults)?;
		
		configure_controller(&self.memory, mount_point, cgroup, available_controllers, defaults)?;
		
		configure_controller(&self.rdma, mount_point, cgroup, available_controllers, defaults)?;
		
		configure_controller(&self.hugetlb, mount_point, cgroup, available_controllers, defaults)?;
		
		Ok(())
	}
	
	fn add_to_desired_controllers(&self, controllers: &mut Controllers)
	{
		controllers.add_if_some(&self.io);
		controllers.add_if_some(&self.memory);
		controllers.add_if_some(&self.rdma);
		controllers.add_if_some(&self.hugetlb);
	}
}

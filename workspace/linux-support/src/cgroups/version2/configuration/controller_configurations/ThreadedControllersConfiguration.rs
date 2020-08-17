// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Threaded controllers configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ThreadedControllersConfiguration
{
	/// `cpu` controller configuration.
	pub cpu: Option<CpuControllerConfiguration>,
	
	/// `cpuset` controller configuration.
	pub cpuset: Option<CpusetControllerConfiguration>,
	
	/// `pids` controller configuration.
	pub pids: Option<PidsControllerConfiguration>,
	
	/// `perf_event` controller configuration.
	pub perf_event: Option<PerfEventControllerConfiguration>,
	
	/// `debug` controller configuration.
	pub debug: Option<DebugControllerConfiguration>,
}

impl ControllersConfiguration for ThreadedControllersConfiguration
{
	fn configure<'name>(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup<'name>>, available_controllers: &Controllers) -> io::Result<()>
	{
		configure_controller(&self.cpu, mount_point, cgroup, available_controllers)?;
		
		configure_controller(&self.cpuset, mount_point, cgroup, available_controllers)?;
		
		configure_controller(&self.pids, mount_point, cgroup, available_controllers)?;
		
		configure_controller(&self.perf_event, mount_point, cgroup, available_controllers)?;
		
		configure_controller(&self.debug, mount_point, cgroup, available_controllers)?;
		
		Ok(())
	}
	
	fn to_desired_controllers(&self) -> Controllers
	{
		let mut controllers = Controllers::new_if_going_to_be_full();
		self.add_to_desired_controllers(&mut controllers);
		controllers
	}
}

impl ThreadedControllersConfiguration
{
	fn add_to_desired_controllers(&self, controllers: &mut Controllers)
	{
		controllers.add_if_some(&self.cpu);
		controllers.add_if_some(&self.cpuset);
		controllers.add_if_some(&self.pids);
		controllers.add_if_some(&self.perf_event);
		controllers.add_if_some(&self.debug);
	}
}

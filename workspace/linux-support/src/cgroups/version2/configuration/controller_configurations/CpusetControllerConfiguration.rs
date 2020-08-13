// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `cpuset` controller configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpusetControllerConfiguration
{
	/// HyperThreads.
	pub hyper_threads: HyperThreads,
	
	/// Partition?
	pub hyper_threads_partition: Partition,
	
	/// NUMA nodes.
	pub numa_nodes: NumaNodes,
}

impl ControllerConfiguration for CpusetControllerConfiguration
{
	const Controller: Controller = Controller::cpuset;
	
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>) -> io::Result<()>
	{
		cgroup.write_cpuset_hyper_threads(mount_point, &self.hyper_threads)?;
		
		cgroup.write_cpuset_hyper_threads_partition(mount_point, self.hyper_threads_partition)?;
		
		cgroup.write_cpuset_numa_nodes(mount_point, &self.numa_nodes)
	}
}

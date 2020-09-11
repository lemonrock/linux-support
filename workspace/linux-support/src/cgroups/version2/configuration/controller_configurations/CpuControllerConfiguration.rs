// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `cpu` controller configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CpuControllerConfiguration
{
	/// Weight.
	#[serde(default = "CpuControllerConfiguration::weight_default")] pub weight: Either<CpuWeight, Nice>,
	
	/// Maximum bandwidth limit.
	#[serde(default)] pub maximum_bandwidth_limit: CpuMaximumBandwidthLimit,
}

impl Default for CpuControllerConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			weight: Self::weight_default(),
			maximum_bandwidth_limit: CpuMaximumBandwidthLimit::default(),
		}
	}
}

impl ControllerConfiguration for CpuControllerConfiguration
{
	const Controller: Controller = Controller::cpu;
	
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>, _defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		match self.weight
		{
			Left(cpu_weight) => cgroup.write_cpu_weight(mount_point, cpu_weight)?,
			Right(nice) => cgroup.write_cpu_weight_nice(mount_point, nice)?,
		}
		
		cgroup.write_cpu_maximum_bandwidth_limit(mount_point, self.maximum_bandwidth_limit.clone())
	}
}

impl CpuControllerConfiguration
{
	#[inline(always)]
	const fn weight_default() -> Either<CpuWeight, Nice>
	{
		Right(Nice::Default)
	}
}

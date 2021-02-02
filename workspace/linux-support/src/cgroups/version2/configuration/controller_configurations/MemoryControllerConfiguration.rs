// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `memory` controller configuration.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct MemoryControllerConfiguration
{
	/// Minimum in bytes.
	///
	/// Hard memory protection.
	/// If the memory usage of a cgroup is within its effective min boundary, the cgroup’s memory won’t be reclaimed under any conditions.
	/// If there is no unprotected reclaimable memory available, the Out-of-Memory (`OOM`) killer is invoked.
	///
	/// Effective min boundary is limited by  `minimum` values of all ancestor cgroups.
	/// If there is memory.min overcommitment (child cgroup or cgroups are requiring more protected memory than parent will allow), then each child cgroup will get the part of parent’s protection proportional to its actual memory usage below  `minimum`.
	///
	/// Putting more memory than generally available under this protection is discouraged and may lead to constant `OOMs`.
	///
	/// If a memory cgroup is not populated with processes, its `minimum` is ignored.
	///
	/// Default is `0`.
	pub minimum: u64,
	
	/// Low in bytes.
	///
	/// Best-effort memory protection.
	/// If the memory usage of a cgroup is within its effective low boundary, the cgroup’s memory won’t be reclaimed unless memory can be reclaimed from unprotected cgroups.
	///
	/// Effective low boundary is limited by `low` values of all ancestor cgroups.
	/// If there is `low` overcommitment (child cgroup or cgroups are requiring more protected memory than parent will allow), then each child cgroup will get the part of parent’s protection proportional to its actual memory usage below `low`.
	///
	/// Putting more memory than generally available under this protection is discouraged
	///
	/// Default is `0`.
	pub low: u64,
	
	/// High in bytes.
	///
	/// Memory usage throttle limit.
	/// This is the main mechanism to control memory usage of a cgroup.
	/// If a cgroup’s usage goes over the high boundary, the processes of the cgroup are throttled and put under heavy reclaim pressure.
	///
	/// Going over the high limit never invokes the OOM killer and under extreme conditions the limit may be breached.
	///
	/// Default is `max`.
	pub high: MaximumNumber<u64>,
	
	/// Maximum in bytes.
	///
	/// Memory usage hard limit.
	/// This is the final protection mechanism.
	/// If a cgroup’s memory usage reaches this limit and can’t be reduced, the Out-of-Memory (`OOM`) killer is invoked in the cgroup.
	/// Under certain circumstances, the usage may go over the limit temporarily.
	///
	/// This is the ultimate protection mechanism.
	/// As long as the high limit is used and monitored properly, this limit’s utility is limited to providing the final safety net.
	///
	/// Default is `max`.
	pub maximum: MaximumNumber<u64>,
}

impl ControllerConfiguration for MemoryControllerConfiguration
{
	const Controller: Controller = Controller::memory;
	
	#[inline(always)]
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>, _defaults: &DefaultHugePageSizes) -> io::Result<()>
	{
		cgroup.write_memory_minimum(mount_point, self.minimum)?;
		cgroup.write_memory_low(mount_point, self.low)?;
		cgroup.write_memory_high(mount_point, self.high)?;
		cgroup.write_memory_maximum(mount_point, self.maximum)?;
		
		Ok(())
	}
}

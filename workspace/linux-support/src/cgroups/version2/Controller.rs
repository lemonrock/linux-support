// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A cgroups version 2 controller.
///
/// Legacy cgroup v1 controllers not included here are:-
///
/// * `cpuacct`: Only if kernel was built with `CONFIG_CGROUP_CPUACCT`.
/// * `devices`: Only if kernel was built with `CONFIG_CGROUP_DEVICE`.
/// * `freezer`: Only if kernel was built with `CONFIG_CGROUP_FREEZER`.
/// * `net_cls`: Only if kernel was built with `CONFIG_CGROUP_NET_CLASSID`.
/// * `net_prio`: Only if kernel was built with `CONFIG_CGROUP_NET_PRIO`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(EnumCount, EnumIter)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Controller
{
	/// Equivalent to cgroups version 1 controller `blkio`.
	///
	/// Only if kernel was built with `CONFIG_BLK_CGROUP`.
	io,

	/// Equivalent to cgroups version 1 controller `memory`.
	///
	/// Only if kernel was built with `CONFIG_MEMCG`.
	memory,

	/// Equivalent to cgroups version 1 controller `pids`.
	///
	/// Only if kernel was built with `CONFIG_CGROUP_PIDS`.
	pids,

	/// Equivalent to cgroups version 1 controller `perf_event`.
	///
	/// Since Linux version 4.11.
	///
	/// Only if kernel was built with `CONFIG_CGROUP_PERF`.
	perf_event,

	/// Equivalent to cgroups version 1 controller `rdma`.
	///
	/// Since Linux version 4.11.
	///
	/// Only if kernel was built with `CONFIG_CGROUP_RDMA`.
	rdma,

	/// Successor to cgroups version 1 controllers `cpu` and `cpuacct`.
	///
	/// Since Linux version 4.15.
	///
	/// Only if kernel was built with `CONFIG_CGROUP_SCHED`.
	cpu,

	/// Successor to `cpusetfs`.
	///
	/// Only if kernel was built with `CONFIG_CPUSETS`.
	cpuset,
	
	/// Only if kernel was built with `CONFIG_CGROUP_HUGETLB`.
	hugetlb,
	
	/// Only if kernel was built with `CONFIG_CGROUP_DEBUG`.
	debug,
}

impl FromBytes for Controller
{
	type Error = ParseControllerError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::Controller::*;
		let variant = match bytes
		{
			b"io" => io,
			b"memory" => memory,
			b"pids" => pids,
			b"perf_event" => perf_event,
			b"rdma" => rdma,
			b"cpu" => cpu,
			b"cpuset" => cpuset,
			b"hugetlb" => hugetlb,
			b"debug" => debug,

			_ => return Err(ParseControllerError::UnknownVariant(bytes.to_vec()))
		};
		Ok(variant)
	}
}

impl Controller
{
	const MaximumNumberOfControllers: usize = Self::COUNT;
	
	/// Is domain?
	#[inline(always)]
	pub fn is_domain_controller(self) -> bool
	{
		!self.is_threaded_controller()
	}
	
	/// Is threaded?
	#[inline(always)]
	pub fn is_threaded_controller(self) -> bool
	{
		use self::Controller::*;
		
		match self
		{
			io => false,
			memory => false,
			pids => true,
			perf_event => true,
			rdma => false,
			cpu => true,
			cpuset => true,
			hugetlb => false,
			debug => true,
		}
	}
	
	/// Is implicit (ie is enabled on root cgroup if legacy cgroup v1 equivalent controller not being used)?
	#[inline(always)]
	pub fn is_implicit_controller(self) -> bool
	{
		use self::Controller::*;
		
		match self
		{
			io => false,
			memory => false,
			pids => false,
			perf_event => true,
			rdma => false,
			cpu => false,
			cpuset => false,
			hugetlb => false,
			debug => true,
		}
	}
	
	#[inline(always)]
	fn append_to(self, line: &mut Vec<u8>, sign: u8)
	{
		line.push(sign);
		line.extend_from_slice(self.to_bytes())
	}
	
	#[inline(always)]
	fn is_controller(value: &[u8]) -> bool
	{
		Self::from_bytes(value).is_ok()
	}

	#[inline(always)]
	fn to_bytes(self) -> &'static [u8]
	{
		use self::Controller::*;
		match self
		{
			io => b"io" as &[u8],
			memory => b"memory" as &[u8],
			pids => b"pids" as &[u8],
			perf_event => b"perf_event" as &[u8],
			rdma => b"rdma" as &[u8],
			cpu => b"cpu" as &[u8],
			cpuset => b"cpuset" as &[u8],
			hugetlb => b"hugetlb" as &[u8],
			debug => b"debug" as &[u8],
		}
	}
}

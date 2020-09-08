// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonRootCgroupVersion2Diagnostics
{
	#[serde(flatten)]
	pub common: CommonCgroupVersion2Diagnostics,
	
	pub type_: DiagnosticUnobtainableResult<NonRootCgroupType>,
	
	pub is_frozen: DiagnosticUnobtainableResult<bool>,
	
	pub event_statistics: DiagnosticUnobtainableResult<EventStatistics>,
	
	pub cpu_weight: DiagnosticUnobtainableResult<Option<CpuWeight>>,
	
	pub cpu_weight_nice: DiagnosticUnobtainableResult<Option<Nice>>,
	
	pub cpu_maximum_bandwidth_limit: DiagnosticUnobtainableResult<Option<CpuMaximumBandwidthLimit>>,
	
	pub cpuset_hyper_threads: DiagnosticUnobtainableResult<Option<HyperThreads>>,
	
	pub cpuset_numa_nodes: DiagnosticUnobtainableResult<Option<NumaNodes>>,
	
	pub cpuset_hyper_threads_partition: DiagnosticUnobtainableResult<Option<ReadPartition>>,
	
	pub hugetlb: HashMap<HugePageSize, NonRootCgroupVersion2HugetlbDiagnostics>,
	
	pub memory_current: DiagnosticUnobtainableResult<Option<u64>>,
	
	pub memory_minimum: DiagnosticUnobtainableResult<Option<u64>>,
	
	pub memory_low: DiagnosticUnobtainableResult<Option<u64>>,
	
	pub memory_high: DiagnosticUnobtainableResult<Option<MaximumNumber<u64>>>,
	
	pub memory_maximum: DiagnosticUnobtainableResult<Option<MaximumNumber<u64>>>,
	
	pub memory_event_statistics: DiagnosticUnobtainableResult<MemoryEventStatistics>,
	
	pub memory_event_statistics_local: DiagnosticUnobtainableResult<MemoryEventStatistics>,
	
	pub memory_statistics: DiagnosticUnobtainableResult<MemoryStatistics>,
	
	pub memory_swap_current: DiagnosticUnobtainableResult<Option<u64>>,
	
	pub memory_swap_maximum: DiagnosticUnobtainableResult<Option<MaximumNumber<u64>>>,
	
	pub memory_swap_event_statistics: DiagnosticUnobtainableResult<MemorySwapEventStatistics>,
	
	pub process_identifiers_count_current: DiagnosticUnobtainableResult<Option<usize>>,
	
	pub process_identifiers_count_maximum: DiagnosticUnobtainableResult<Option<ProcessIdentifiersMaximum>>,
	
	pub process_identifiers_event_statistics: DiagnosticUnobtainableResult<ProcessIdentifiersEventStatistics>,
	
	pub rdma_current: DiagnosticUnobtainableResult<RdmaFile>,
	
	pub rdma_maximum: DiagnosticUnobtainableResult<RdmaFile>,
}

impl NonRootCgroupVersion2Diagnostics
{
	fn gather_children_of_parent(mount_point: &CgroupMountPoint, parent_cgroup: &Rc<impl Cgroup>, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> DiagnosticUnobtainableResult<HashMap<CgroupName, Self>>
	{
		match child_cgroup_names(parent_cgroup, mount_point)
		{
			Err(error) => Err(DiagnosticUnobtainable::from(error)),
			
			Ok(child_cgroup_names) =>
			{
				let mut child_cgroup_diagnostics = HashMap::new();
				for child_cgroup_name in child_cgroup_names
				{
					let diagnostics =
					{
						let child_cgroup = parent_cgroup.clone().child(Cow::Borrowed(&child_cgroup_name));
						Self::gather(mount_point, &child_cgroup, supported_huge_page_sizes)
					};
					child_cgroup_diagnostics.insert(child_cgroup_name, diagnostics)
				}
				Ok(child_cgroup_diagnostics)
			}
		}
	}
	
	fn gather(mount_point: &CgroupMountPoint, child_cgroup: &Rc<NonRootCgroup>, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> Self
	{
		Self
		{
			common: CommonCgroupVersion2Diagnostics::gather(mount_point, child_cgroup, supported_huge_page_sizes),
		
			type_: child_cgroup.read_type(mount_point).map_err(DiagnosticUnobtainable::from),
			
			is_frozen: child_cgroup.is_frozen(mount_point).map_err(DiagnosticUnobtainable::from),
			
			event_statistics: child_cgroup.read_events(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpu_weight: child_cgroup.read_cpu_weight(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpu_weight_nice: child_cgroup.read_cpu_weight_nice(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpu_maximum_bandwidth_limit: child_cgroup.read_cpu_maximum_bandwidth_limit(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpuset_hyper_threads: child_cgroup.read_cpuset_hyper_threads(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpuset_numa_nodes: child_cgroup.read_cpuset_numa_nodes(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpuset_hyper_threads_partition: child_cgroup.read_cpuset_hyper_threads_partition(mount_point).map_err(DiagnosticUnobtainable::from),
			
			hugetlb: NonRootCgroupVersion2HugetlbDiagnostics::gather(mount_point, child_cgroup, supported_huge_page_sizes),
			
			memory_current: child_cgroup.read_memory_current(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_minimum: child_cgroup.read_memory_minimum(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_low: child_cgroup.read_memory_low(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_high: child_cgroup.read_memory_high(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_maximum: child_cgroup.read_memory_maximum(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_event_statistics: child_cgroup.read_memory_events(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_event_statistics_local: child_cgroup.read_memory_events_local(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_statistics: child_cgroup.read_memory_statistics(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_swap_current: child_cgroup.read_memory_swap_current(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_swap_maximum: child_cgroup.read_memory_swap_maximum(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_swap_event_statistics: child_cgroup.read_memory_swap_events(mount_point).map_err(DiagnosticUnobtainable::from),
			
			process_identifiers_count_current: child_cgroup.read_process_identifiers_count_current(mount_point).map_err(DiagnosticUnobtainable::from),
			
			process_identifiers_count_maximum: child_cgroup.read_process_identifiers_count_maximum(mount_point).map_err(DiagnosticUnobtainable::from),
			
			process_identifiers_event_statistics: child_cgroup.read_process_identifiers_events(mount_point).map_err(DiagnosticUnobtainable::from),
			
			rdma_current: child_cgroup.read_rdma_current(mount_point).map_err(DiagnosticUnobtainable::from),
			
			rdma_maximum: child_cgroup.read_rdma_maximum(mount_point).map_err(DiagnosticUnobtainable::from),
		}
	}
}

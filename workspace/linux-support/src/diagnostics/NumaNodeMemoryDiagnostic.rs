// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumaNodeMemoryDiagnostic
{
	pub associated_hyper_threads: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub huge_page_pool_statistics: HashMap<HugePageSize, DiagnosticUnobtainableResult<HugePagePoolStatistics>>,
	
	pub distances: DiagnosticUnobtainableResult<Option<HashMap<NumaNode, MemoryLatencyRelativeCost>>>,
	
	pub numa_memory_statistics: DiagnosticUnobtainableResult<HashMap<VirtualMemoryStatisticName, u64>>,
	
	pub zoned_virtual_memory_statistics: DiagnosticUnobtainableResult<HashMap<VirtualMemoryStatisticName, u64>>,
	
	pub memory_information: DiagnosticUnobtainableResult<MemoryInformation>,
}

impl NumaNodeMemoryDiagnostic
{
	#[allow(deprecated)]
	fn gather(sys_path: &SysPath, numa_node: NumaNode, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> Self
	{
		#[inline(always)]
		fn wrap_panic<R>(numa_node: NumaNode, callback: impl FnOnce(NumaNode) -> R) -> DiagnosticUnobtainableResult<R>
		{
			catch_unwind(AssertUnwindSafe(|| callback(numa_node))).map_err(|_| DiagnosticUnobtainable(format!("Panicked")))
		}
		
		Self
		{
			associated_hyper_threads: wrap_panic(numa_node, |numa_node| numa_node.associated_hyper_threads(sys_path).unwrap()),
			huge_page_pool_statistics:
			{
				let mut huge_page_pool_statistics = HashMap::with_capacity(supported_huge_page_sizes.len());
				for huge_page_size in supported_huge_page_sizes
				{
					let huge_page_size = *huge_page_size;
					huge_page_pool_statistics.insert(huge_page_size, wrap_panic(numa_node, |numa_node| numa_node.huge_page_pool_statistics(sys_path, huge_page_size).unwrap()));
				}
				huge_page_pool_statistics
			},
			distances: wrap_panic(numa_node, |numa_node| numa_node.distances(sys_path).unwrap()),
			numa_memory_statistics:  wrap_panic(numa_node, |numa_node| numa_node.numa_memory_statistics(sys_path, None).unwrap()),
			zoned_virtual_memory_statistics: wrap_panic(numa_node, |numa_node| numa_node.zoned_virtual_memory_statistics(sys_path, None).unwrap()),
			memory_information: wrap_panic(numa_node, |numa_node| numa_node.memory_information(sys_path, None).unwrap()),
		}
	}
}

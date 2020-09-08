// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryDiagnostics
{
	pub global_memory_information: DiagnosticUnobtainableResult<MemoryInformation>,
	
	pub global_zoned_virtual_memory_statistics: DiagnosticUnobtainableResult<HashMap<VirtualMemoryStatisticName, u64>>,
	
	pub page_size: DiagnosticUnobtainableResult<PageSizeMemoryDiagnostics>,

	pub numa_nodes: NumaNodeMemoryDiagnostics,
}

impl MemoryDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> Self
	{
		Self
		{
			global_memory_information: MemoryInformation::parse_global(proc_path, true).map_err(DiagnosticUnobtainable::from),
		
			global_zoned_virtual_memory_statistics: VirtualMemoryStatisticName::global_zoned_virtual_memory_statistics(proc_path, true).map_err(DiagnosticUnobtainable::from),
			
			page_size: PageSizeMemoryDiagnostics::gather(sys_path, proc_path, supported_huge_page_sizes),
			
			numa_nodes: NumaNodeMemoryDiagnostics::gather(sys_path, proc_path),
		}
	}
}

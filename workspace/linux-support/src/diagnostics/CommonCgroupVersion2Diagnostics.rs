// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommonCgroupVersion2Diagnostics
{
	pub available_controllers: DiagnosticUnobtainableResult<Controllers>,
	
	pub maximum_depth: DiagnosticUnobtainableResult<MaximumNumber<usize>>,
	
	pub maximum_descendants: DiagnosticUnobtainableResult<MaximumNumber<usize>>,
	
	pub process_identifiers: DiagnosticUnobtainableResult<Vec<ProcessIdentifier>>,
	
	pub statistics: DiagnosticUnobtainableResult<Statistics>,
	
	pub cpu_pressure_stall_information: DiagnosticUnobtainableResult<CpuTimeStalled>,
	
	pub cpu_statistics: DiagnosticUnobtainableResult<CpuStatistics>,
	
	pub input_output_pressure_stall_information: DiagnosticUnobtainableResult<MemoryOrInputOutputTimeStalled>,
	
	pub memory_pressure_stall_information: DiagnosticUnobtainableResult<MemoryOrInputOutputTimeStalled>,
	
	pub subtree_controllers: DiagnosticUnobtainableResult<Controllers>,
	
	pub thread_identifiers: DiagnosticUnobtainableResult<Vec<ThreadIdentifier>>,
	
	pub cpuset_hyper_threads_effective: DiagnosticUnobtainableResult<Option<HyperThreads>>,
	
	pub cpuset_numa_nodes_effective: DiagnosticUnobtainableResult<Option<NumaNodes>>,
	
	pub effective_extended_bpf_program_identifiers: DiagnosticUnobtainableResult<HashMap<CgroupProgramAttachmentType, DiagnosticUnobtainableResult<(Vec<ExtendedBpfProgramIdentifier>, CgroupProgramAttachmentFlags)>>>,
	
	pub children: DiagnosticUnobtainableResult<HashMap<CgroupName, NonRootCgroupVersion2Diagnostics>>,
}

impl CommonCgroupVersion2Diagnostics
{
	fn gather(mount_point: &CgroupMountPoint, cgroup: &Rc<impl Cgroup>, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> Self
	{
		Self
		{
			available_controllers: cgroup.read_available_controllers(mount_point).map_err(DiagnosticUnobtainable::from),
			
			maximum_depth: cgroup.read_maximum_depth(mount_point).map_err(DiagnosticUnobtainable::from),
			
			maximum_descendants: cgroup.read_maximum_descendants(mount_point).map_err(DiagnosticUnobtainable::from),
			
			process_identifiers: cgroup.get_process_identifiers(mount_point).map_err(DiagnosticUnobtainable::from),
			
			statistics: cgroup.read_statistics(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpu_pressure_stall_information: cgroup.read_cpu_pressure_stall_information(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpu_statistics: cgroup.read_cpu_statistics(mount_point).map_err(DiagnosticUnobtainable::from),
			
			input_output_pressure_stall_information: cgroup.read_input_output_pressure_stall_information(mount_point).map_err(DiagnosticUnobtainable::from),
			
			memory_pressure_stall_information: cgroup.read_memory_pressure_stall_information(mount_point).map_err(DiagnosticUnobtainable::from),
			
			subtree_controllers: cgroup.read_subtree_controllers(mount_point).map_err(DiagnosticUnobtainable::from),
			
			thread_identifiers: cgroup.get_thread_identifiers(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpuset_hyper_threads_effective: cgroup.cpuset_hyper_threads_effective(mount_point).map_err(DiagnosticUnobtainable::from),
			
			cpuset_numa_nodes_effective: cgroup.cpuset_numa_nodes_effective(mount_point).map_err(DiagnosticUnobtainable::from),
			
			effective_extended_bpf_program_identifiers: match cgroup.open(mount_point)
			{
				Err(error) => Err(DiagnosticUnobtainable::from(error)),
				
				Ok(cgroup_file_descriptor) =>
				{
					let mut effective_program_identifiers = HashMap::with_capacity(CgroupProgramAttachmentType::COUNT);
					for program_attachment_type in CgroupProgramAttachmentType::iter()
					{
						effective_program_identifiers.insert(program_attachment_type, cgroup_file_descriptor.query_attached_extended_bpf_programs(program_attachment_type, CgroupProgramQueryFlags::Effective).map_err(|_: ()| DiagnosticUnobtainable(format!("Permission Denied"))));
					}
					Ok(effective_program_identifiers)
				}
			},
			
			children: NonRootCgroupVersion2Diagnostics::gather_children_of_parent(mount_point, cgroup, supported_huge_page_sizes),
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumaNodeMemoryDiagnostics
{
	pub is_a_numa_machine: bool,
	
	pub valid: DiagnosticUnobtainableResult<Option<NumaNodes>>,
	
	pub numa_nodes: DiagnosticUnobtainableResult<HashMap<NumaNode, NumaNodeMemoryDiagnostic>>
}

impl NumaNodeMemoryDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		let is_a_numa_machine = NumaNode::is_a_numa_machine(sys_path);
		
		if !is_a_numa_machine
		{
			return Self
			{
				is_a_numa_machine: false,
				valid: Ok(None),
				numa_nodes: Ok(HashMap::new()),
			}
		}
		
		let numa_nodes = match catch_unwind(|| NumaNodes::possible(sys_path)).map_err(|_| DiagnosticUnobtainable(format!("Possible NUMA nodes panicked")))
		{
			Err(error) => Err(error),
			Ok(None) => Ok(HashMap::new()),
			Ok(Some(numa_nodes)) =>
				{
					let mut numa_node_diagnostics = HashMap::with_capacity(numa_nodes.len());
					for numa_node in numa_nodes
					{
						numa_node_diagnostics.insert(numa_node, NumaNodeMemoryDiagnostic::gather(sys_path, proc_path, numa_node));
					}
				}
		};
		
		Self
		{
			is_a_numa_machine: true,
			valid: catch_unwind(|| NumaNodes::valid(sys_path, proc_path)).map_err(|_| DiagnosticUnobtainable(format!("Valid NUMA nodes panicked"))),
			numa_nodes,
		}
	}
}

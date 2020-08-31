// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InterruptRequestDiagnostics
{
	/// Usually `ffffffff` (ie `/sys/devices/system/cpu/possible` but as a bitmask not a list).
	pub default_smp_affinity: DiagnosticUnobtainableResult<HyperThreads>,
	
	/// Per-interrupt request diagnostics.
	pub interrupt_request_diagnostics: DiagnosticUnobtainableResult<HashMap<InterrruptRequest, InterruptRequestDiagnostic>>,
}

impl InterruptRequestDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		Self
		{
			default_smp_affinity: InterruptRequest::default_smp_affinity(proc_path).map_err(DiagnosticUnobtainable::from),
			interrupt_request_diagnostics:
			{
				let mut interrupt_request_diagnostics = HashMap::new();
				
				for interrupt_request in InterruptRequest::all(sys_path).map_err(DiagnosticUnobtainable::from)?
				{
					let interrupt_request_diagnostic = InterruptRequestDiagnostic::gather(sys_path, proc_path, interrupt_request);
					interrupt_request_diagnostics.insert(interrupt_request, interrupt_request_diagnostic);
				}
				
				Ok(interrupt_request_diagnostics)
			}
		}
	}
}

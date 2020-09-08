// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct ThreadDiagnostic
{
	pub name: DiagnosticUnobtainableResult<ThreadName>,
	
	pub priority: DiagnosticUnobtainableResult<Nice>,
	
	pub io_priority: DiagnosticUnobtainableResult<IoPriority>,
	
	pub scheduler_policy_and_flags: DiagnosticUnobtainableResult<PerThreadSchedulerPolicyAndFlags>,

	pub permitted_effective_and_inheritable_capabilities: DiagnosticUnobtainableResult<PermittedEffectiveAndInheritableCapabilitySets>,
}

impl ThreadDiagnostic
{
	fn gather(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier) -> Self
	{
		Self
		{
			name: ThreadName::get_thread_name(process_identifier, thread_identifier, proc_path).map_err(DiagnosticUnobtainable::from),
			
			priority: Nice::get_thread_priority(thread_identifier).map_err(|_: ()| DiagnosticUnobtainable(format!("Could not obtain thread priority"))),
			
			io_priority: IoPriority::get_for_thread(thread_identifier).map_err(|cause| DiagnosticUnobtainable(format!("Could not obtain thread I/O priority ({})", IoPriority::explain_error(cause)))),
			
			scheduler_policy_and_flags: PerThreadSchedulerPolicyAndFlags::get_for_thread(ThreadIdentifierChoice::Other(thread_identifier)).map_err(DiagnosticUnobtainable::from),
			
			permitted_effective_and_inheritable_capabilities: PermittedEffectiveAndInheritableCapabilitySets::get(thread_identifier).map_err(DiagnosticUnobtainable::from),
		}
	}
}

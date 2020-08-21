// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct ThreadDiagnostic
{
	pub thread_name: DiagnosticUnobtainableResult<ThreadName>,
	
	pub current_thread_priority: DiagnosticUnobtainableResult<Nice>,
	
	pub current_thread_scheduler_policy_and_flags: DiagnosticUnobtainableResult<PerThreadSchedulerPolicyAndFlags>,
}

impl ThreadDiagnostic
{
	fn gather(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier) -> Self
	{
		Self
		{
			thread_name: ThreadName::get_thread_name(process_identifier, thread_identifier, proc_path).map_err(DiagnosticUnobtainable::from),
			
			current_thread_priority: Nice::get_thread_priority(current_thread_identifier).map_err(DiagnosticUnobtainable::from),
			
			current_thread_scheduler_policy_and_flags: PerThreadSchedulerPolicyAndFlags::get_for_thread(ThreadIdentifierChoice::Current).map_err(DiagnosticUnobtainable::from),
		}
	}
	
	#[inline(always)]
	fn current_thread_has_keep_capabilities() -> DiagnosticUnobtainableResult<bool>
	{
		Self::prctl_boolean(PR_GET_KEEPCAPS)
	}
	
	#[inline(always)]
	fn no_new_privileges() -> DiagnosticUnobtainableResult<bool>
	{
		Self::prctl_boolean(PR_GET_NO_NEW_PRIVS)
	}
	
	#[inline(always)]
	fn transparent_huge_pages_disabled() -> DiagnosticUnobtainableResult<bool>
	{
		Self::prctl_boolean(PR_GET_THP_DISABLE)
	}
	
	#[inline(always)]
	fn prctl_boolean(operation: i32) -> DiagnosticUnobtainableResult<bool>
	{
		process_control_get_boolean(operation).map_err(DiagnosticUnobtainable::from)
	}
}

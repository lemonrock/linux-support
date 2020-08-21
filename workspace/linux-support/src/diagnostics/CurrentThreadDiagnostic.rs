// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct CurrentThreadDiagnostic
{
	pub current_rust_thread_identifier: NonZeroU64,
	
	pub current_rust_thread_name: Option<String>,
	
	pub current_thread_identifier: ThreadIdentifier,
	
	pub current_thread_name: ThreadName,
	
	pub current_thread_has_keep_capabilities: DiagnosticUnobtainableResult<bool>,
	
	pub current_thread_no_new_privileges: DiagnosticUnobtainableResult<bool>,
	
	pub current_thread_transparent_huge_pages_disabled: DiagnosticUnobtainableResult<bool>,
	
	pub current_thread_current_timer_slack: DiagnosticUnobtainableResult<CurrentTimerSlackNanoseconds>,
	
	pub current_thread_store_bypass_speculation_mitigation: DiagnosticUnobtainableResult<SpeculationMitigation>,
	
	pub current_thread_indirect_store_speculation_mitigation: DiagnosticUnobtainableResult<SpeculationMitigation>,
	
	pub current_thread_secure_bits: DiagnosticUnobtainableResult<SecureBits>,
}

impl CurrentThreadDiagnostic
{
	fn gather(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Self
	{
		let current_thread_identifier = ThreadIdentifier::default();
		
		let thread = current();
		
		let current_thread_name = ThreadName::get_current_thread_name();
		
		Self
		{
			current_rust_thread_identifier: thread.id().as_u64(),
			
			current_rust_thread_name: thread.name().map(str::to_owned),
			
			current_thread_identifier,
			
			current_thread_name,
			
			current_thread_has_keep_capabilities: Self::current_thread_has_keep_capabilities(),
			
			current_thread_no_new_privileges: Self::no_new_privileges(),
			
			current_thread_transparent_huge_pages_disabled: Self::transparent_huge_pages_disabled(),
			
			current_thread_current_timer_slack: CurrentTimerSlackNanoseconds::current().map_err(DiagnosticUnobtainable::from),
			
			current_thread_store_bypass_speculation_mitigation: SpeculationMitigation::store_bypass().map_err(DiagnosticUnobtainable::from),
			
			current_thread_indirect_store_speculation_mitigation: SpeculationMitigation::indirect_branch().map_err(DiagnosticUnobtainable::from),
			
			current_thread_secure_bits: SecureBits::current().map_err(DiagnosticUnobtainable::from),
		}
	}
}

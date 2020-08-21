// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct MiscellaneousProcessControlDiagnostics
{
	pub current_process_parent_death_signal: DiagnosticUnobtainableResult<Option<Signal>>,
	pub current_process_child_subreaper_process: DiagnosticUnobtainableResult<Option<ProcessIdentifier>>,
	pub is_dumpable: DiagnosticUnobtainableResult<bool>,
	pub is_in_io_flusher_state: DiagnosticUnobtainableResult<bool>,
	pub current_thread_has_keep_capabilities: DiagnosticUnobtainableResult<bool>,
	pub machine_check_exception_kill_policy: DiagnosticUnobtainableResult<MachineCheckExceptionKillPolicy>,
	pub no_new_privileges: DiagnosticUnobtainableResult<bool>,
	pub transparent_huge_pages_disabled: DiagnosticUnobtainableResult<bool>,
	pub secure_bits: DiagnosticUnobtainableResult<SecureBits>,
	pub timestamp_counter_setting: DiagnosticUnobtainableResult<TimestampCounterSetting>,
	pub current_timer_slack: DiagnosticUnobtainableResult<CurrentTimerSlackNanoseconds>,
	pub store_bypass_speculation_mitigation: DiagnosticUnobtainableResult<SpeculationMitigation>,
	pub indirect_store_speculation_mitigation: DiagnosticUnobtainableResult<SpeculationMitigation>,
}

impl MiscellaneousProcessControlDiagnostics
{
	fn gather() -> Self
	{
		Self
		{
			current_process_parent_death_signal: Signal::get_current_process_parent_death_signal().map_err(DiagnosticUnobtainable::from),
			current_process_child_subreaper_process: ProcessIdentifier::get_current_process_child_subreaper_process().map_err(DiagnosticUnobtainable::from),
			is_dumpable: Self::is_dumpable(),
			is_in_io_flusher_state: Self::is_in_io_flusher_state(),
			current_thread_has_keep_capabilities: Self::current_thread_has_keep_capabilities(),
			machine_check_exception_kill_policy: MachineCheckExceptionKillPolicy::for_current_process().map_err(DiagnosticUnobtainable::from),
			no_new_privileges: Self::no_new_privileges(),
			transparent_huge_pages_disabled: Self::transparent_huge_pages_disabled(),
			secure_bits: SecureBits::current().map_err(DiagnosticUnobtainable::from),
			timestamp_counter_setting: TimestampCounterSetting::current().map_err(DiagnosticUnobtainable::from),
			current_timer_slack: CurrentTimerSlackNanoseconds::current().map_err(DiagnosticUnobtainable::from),
			store_bypass_speculation_mitigation: SpeculationMitigation::store_bypass().map_err(DiagnosticUnobtainable::from),
			indirect_store_speculation_mitigation: SpeculationMitigation::indirect_branch().map_err(DiagnosticUnobtainable::from),
		}
	}
	
	#[inline(always)]
	fn is_dumpable() -> DiagnosticUnobtainableResult<bool>
	{
		Self::prctl_boolean(PR_GET_DUMPABLE)
	}
	
	#[inline(always)]
	fn is_in_io_flusher_state() -> DiagnosticUnobtainableResult<bool>
	{
		Self::prctl_boolean(PR_GET_IO_FLUSHER)
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
		process_control_wrapper1
		(
			operation,
			|non_negative_result| match non_negative_result
			{
				0 => Ok(false),
				1 => Ok(true),
				_ => Err(DiagnosticUnobtainable(format!("Non-boolean result `{}` from `prctl()`", non_negative_result))),
			},
			|error_number| Err(DiagnosticUnobtainable(format!("Error result `{}` from `prctl()`", error_number))),
		)
	}
}

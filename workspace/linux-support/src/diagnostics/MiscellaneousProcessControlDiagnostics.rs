// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct MiscellaneousProcessControlDiagnostics
{
	pub current_process_parent_death_signal: DiagnosticUnobtainableResult<Option<Signal>>,
	pub current_process_child_subreaper_process: DiagnosticUnobtainableResult<Option<ProcessIdentifier>>,
	pub current_process_is_dumpable: DiagnosticUnobtainableResult<bool>,
	pub current_process_is_in_io_flusher_state: DiagnosticUnobtainableResult<bool>,
	pub current_process_machine_check_exception_kill_policy: DiagnosticUnobtainableResult<MachineCheckExceptionKillPolicy>,
	pub current_process_timestamp_counter_setting: DiagnosticUnobtainableResult<TimestampCounterSetting>,
}

impl MiscellaneousProcessControlDiagnostics
{
	fn gather() -> Self
	{
		Self
		{
			current_process_parent_death_signal: Signal::get_current_process_parent_death_signal().map_err(DiagnosticUnobtainable::from),
			current_process_child_subreaper_process: ProcessIdentifier::get_current_process_child_subreaper_process().map_err(DiagnosticUnobtainable::from),
			current_process_is_dumpable: Self::is_dumpable(),
			current_process_is_in_io_flusher_state: Self::is_in_io_flusher_state(),
			current_process_machine_check_exception_kill_policy: MachineCheckExceptionKillPolicy::for_current_process().map_err(DiagnosticUnobtainable::from),
			current_process_timestamp_counter_setting: TimestampCounterSetting::current().map_err(DiagnosticUnobtainable::from),
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
	fn prctl_boolean(operation: i32) -> DiagnosticUnobtainableResult<bool>
	{
		process_control_get_boolean(operation).map_err(DiagnosticUnobtainable::from)
	}
}

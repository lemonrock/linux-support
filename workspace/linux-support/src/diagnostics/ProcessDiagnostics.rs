// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct ProcessDiagnostics
{
	pub process_group_identifier: DiagnosticUnobtainableResult<ProcessGroupIdentifier>,
	
	pub session_identifier: DiagnosticUnobtainableResult<ProcessGroupIdentifier>,
	
	pub audit_session_identifier: DiagnosticUnobtainableResult<Option<ProcessGroupIdentifier>>,
	
	pub process_identifier: ProcessIdentifier,
	
	pub process_name: DiagnosticUnobtainableResult<ProcessName>,
	
	pub program_invocation_short_name: CString,
	
	pub stat: DiagnosticUnobtainableResult<Stat>,
	
	pub stat_m: DiagnosticUnobtainableResult<StatM>,
	
	pub status: DiagnosticUnobtainableResult<Status>,
	
	pub personality: DiagnosticUnobtainableResult<PersonalityFlags>,

	pub original_command_line: DiagnosticUnobtainableResult<CommandLine>,

	pub original_environment: DiagnosticUnobtainableResult<Environment>,

	pub memory_maps: DiagnosticUnobtainableResult<MemoryMaps>,
}

impl ProcessDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Self
	{
		Self
		{
			process_group_identifier: process_identifier.process_group_identifier().map_err(|_: ()| DiagnosticUnobtainable(format!("Could not obtain process group identifier"))),
			
			session_identifier: process_identifier.session_identifier().map_err(|_: ()| DiagnosticUnobtainable(format!("Could not obtain session identifier"))),
			
			audit_session_identifier: ProcessGroupIdentifier::audit_session_identifier(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			process_identifier: process_identifier.into(),
			
			process_name: process_identifier.process_name(proc_path).map_err(DiagnosticUnobtainable::from),
			
			program_invocation_short_name: get_program_invocation_short_name().to_owned(),
			
			stat: Stat::process_stat(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			stat_m: StatM::process_statm(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			status: Status::process_status(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			personality: PersonalityFlags::for_process(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			original_command_line: CommandLine::for_process(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			original_environment: Environment::original_for_process(proc_path, process_identifier).map_err(DiagnosticUnobtainable::from),
			
			memory_maps: MemoryMaps::smaps_for_process(proc_path, process_identifier, NumaNodes::have_movable_memory(sys_path).as_ref()).map_err(DiagnosticUnobtainable::from),
		}
	}
}

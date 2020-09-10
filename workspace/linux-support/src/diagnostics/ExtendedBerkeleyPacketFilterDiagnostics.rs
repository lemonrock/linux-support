// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedBerkeleyPacketFilterDiagnostics
{
	pub just_in_time_compilation_choice: DiagnosticUnobtainableResult<JustInTimeCompilationChoice>,
	
	pub just_in_time_compilation_hardening: DiagnosticUnobtainableResult<JustInTimeCompilationHardening>,
	
	pub just_in_time_memory_allocation_limit_size_in_bytes: DiagnosticUnobtainableResult<JustInTimeMemoryAllocationLimitSizeInBytes>,
	
	pub program_diagnostics: Vec<DiagnosticUnobtainableResult<ProgramExtendedBerkeleyPacketFilterDiagnostic>>,
	
	pub map_diagnostics: Vec<DiagnosticUnobtainableResult<MapExtendedBerkeleyPacketFilterDiagnostic>>,
	
	pub type_format_diagnostics: Vec<DiagnosticUnobtainableResult<TypeFormatExtendedBerkeleyPacketFilterDiagnostic>>,
	
	pub pinned_program_diagnostics: HashMap<PathBuf, ProgramExtendedBerkeleyPacketFilterDiagnostic>,
	
	pub pinned_map_diagnostics: HashMap<PathBuf, MapExtendedBerkeleyPacketFilterDiagnostic>,
	
	pub pinned_type_format_diagnostics: HashMap<PathBuf, TypeFormatExtendedBerkeleyPacketFilterDiagnostic>,
}

impl ExtendedBerkeleyPacketFilterDiagnostics
{
	#[inline(always)]
	fn gather(proc_path: &ProcPath, file_systems: &FileSystemsDiagnostics) -> Self
	{
		let (pinned_program_diagnostics, pinned_map_diagnostics, pinned_type_format_diagnostics) = Self::pinned(file_systems);
		
		Self
		{
			just_in_time_compilation_choice: JustInTimeCompilationChoice::value(proc_path).map_err(DiagnosticUnobtainable::from),
			
			just_in_time_compilation_hardening: JustInTimeCompilationHardening::value(proc_path).map_err(DiagnosticUnobtainable::from),
			
			just_in_time_memory_allocation_limit_size_in_bytes: JustInTimeMemoryAllocationLimitSizeInBytes::global_maximum(proc_path).map_err(DiagnosticUnobtainable::from),
			
			program_diagnostics: ProgramExtendedBerkeleyPacketFilterDiagnostic::all(),
			
			map_diagnostics: MapExtendedBerkeleyPacketFilterDiagnostic::all(),
		
			type_format_diagnostics: TypeFormatExtendedBerkeleyPacketFilterDiagnostic::all(),
			
			pinned_program_diagnostics,
			
			pinned_map_diagnostics,
			
			pinned_type_format_diagnostics,
		}
	}
	
	fn pinned(file_systems: &FileSystemsDiagnostics) -> (HashMap<PathBuf, ProgramExtendedBerkeleyPacketFilterDiagnostic>, HashMap<PathBuf, MapExtendedBerkeleyPacketFilterDiagnostic>, HashMap<PathBuf, TypeFormatExtendedBerkeleyPacketFilterDiagnostic>)
	{
		let mut pinned_program_diagnostics = HashMap::new();
		let mut pinned_map_diagnostics = HashMap::new();
		let mut pinned_type_format_diagnostics = HashMap::new();
		if let Some(mount_path) = file_systems.bpf_mount_path()
		{
			let mount_point = BpfMountPoint::from_path(mount_path.to_path_buf());
			
			mount_point.all_pinned_object_file_paths(&mut |pinned_object_fully_qualified_file_path|
			{
				if let Ok(Some(program_diagnostic)) = ProgramExtendedBerkeleyPacketFilterDiagnostic::gather_from_pinned_object(&pinned_object_fully_qualified_file_path)
				{
					pinned_program_diagnostics.insert(pinned_object_fully_qualified_file_path, program_diagnostic);
				}
				else if let Ok(Some(map_diagnostic)) = MapExtendedBerkeleyPacketFilterDiagnostic::gather_from_pinned_object(&pinned_object_fully_qualified_file_path)
				{
					pinned_map_diagnostics.insert(pinned_object_fully_qualified_file_path, map_diagnostic);
				}
				else if let Ok(Some(bpf_type_format_diagnostic)) = TypeFormatExtendedBerkeleyPacketFilterDiagnostic::gather_from_pinned_object(&pinned_object_fully_qualified_file_path)
				{
					pinned_type_format_diagnostics.insert(pinned_object_fully_qualified_file_path, bpf_type_format_diagnostic);
				}
			});
		}
		(pinned_program_diagnostics, pinned_map_diagnostics, pinned_type_format_diagnostics)
	}
}

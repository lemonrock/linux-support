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
	
	pub program_diagnostics: Vec<DiagnosticUnobtainableResult<ExtendedBerkeleyPacketFilterProgramDiagnostic>>,
	
	pub map_diagnostics: Vec<DiagnosticUnobtainableResult<ExtendedBerkeleyPacketFilterMapDiagnostic>>,
	
	pub bpf_type_format_diagnostics: Vec<DiagnosticUnobtainableResult<ExtendedBerkeleyPacketFilterTypeFormatDiagnostic>>,
	
	pub pinned_program_diagnostics: HashMap<PathBuf, ExtendedBerkeleyPacketFilterProgramDiagnostic>,
	
	pub pinned_map_diagnostics: HashMap<PathBuf, ExtendedBerkeleyPacketFilterMapDiagnostic>,
	
	pub pinned_bpf_type_format_diagnostics: HashMap<PathBuf, ExtendedBerkeleyPacketFilterTypeFormatDiagnostic>,
}

impl ExtendedBerkeleyPacketFilterDiagnostics
{
	#[inline(always)]
	fn gather(proc_path: &ProcPath, file_systems: &FileSystemsDiagnostics) -> Self
	{
		let (pinned_program_diagnostics, pinned_map_diagnostics, pinned_bpf_type_format_diagnostics) = Self::pinned(file_systems);
		
		Self
		{
			just_in_time_compilation_choice: JustInTimeCompilationChoice::value(proc_path).map_err(DiagnosticUnobtainable::from),
			
			just_in_time_compilation_hardening: JustInTimeCompilationHardening::value(proc_path).map_err(DiagnosticUnobtainable::from),
			
			just_in_time_memory_allocation_limit_size_in_bytes: JustInTimeMemoryAllocationLimitSizeInBytes::value(proc_path).map_err(DiagnosticUnobtainable::from),
			
			program_diagnostics: Self::all::<ExtendedBpfProgramFileDescriptor, ExtendedBerkeleyPacketFilterProgramDiagnostic>(ExtendedBerkeleyPacketFilterProgramDiagnostic::gather),
			
			map_diagnostics: Self::all::<MapFileDescriptor, ExtendedBerkeleyPacketFilterMapDiagnostic>(ExtendedBerkeleyPacketFilterMapDiagnostic::gather),
		
			bpf_type_format_diagnostics: Self::all::<BpfTypeFormatFileDescriptor, ExtendedBerkeleyPacketFilterTypeFormatDiagnostic>(ExtendedBerkeleyPacketFilterTypeFormatDiagnostic::gather),
			
			pinned_program_diagnostics,
			
			pinned_map_diagnostics,
		
			pinned_bpf_type_format_diagnostics,
		}
	}
	
	fn all<BFD: BpfFileDescriptor, I>(gather_information: impl FnOnce(&BFD) -> Result<I, Errno>) -> Vec<DiagnosticUnobtainableResult<I>>
	{
		let mut diagnostics = Vec::new();
		
		let mut next = ExtendedBpfProgramIdentifier::first();
		while let Some(some_next) = next
		{
			if let Ok(Some(file_descriptor)) = some_next.to_file_descriptor(())
			{
				diagnostics.push(gather_information(&file_descriptor).map_err(DiagnosticUnobtainable::from));
			}
			next = some_next.next();
		}
		
		diagnostics
	}
	
	fn pinned(file_systems: &FileSystemsDiagnostics) -> (HashMap<PathBuf, ExtendedBerkeleyPacketFilterProgramDiagnostic>, HashMap<PathBuf, ExtendedBerkeleyPacketFilterMapDiagnostic>, HashMap<PathBuf, ExtendedBerkeleyPacketFilterTypeFormatDiagnostic>)
	{
		let mut pinned_program_diagnostics = HashMap::new();
		let mut pinned_map_diagnostics = HashMap::new();
		let mut pinned_bpf_type_format_diagnostics = HashMap::new();
		if let Some(mount_path) = file_systems.bpf_mount_path()
		{
			let mount_point = BpfMountPoint::from_path(mount_path.to_path_buf());
			
			mount_point.all_pinned_object_file_paths(&mut |pinned_object_fully_qualified_file_path|
			{
				if let Ok(Some(program_diagnostic)) = Self::gather_from_pinned_object::<ExtendedBpfProgramFileDescriptor, ExtendedBerkeleyPacketFilterProgramDiagnostic>(&pinned_object_fully_qualified_file_path, ExtendedBerkeleyPacketFilterProgramDiagnostic::gather)
				{
					pinned_program_diagnostics.insert(pinned_object_fully_qualified_file_path, program_diagnostic);
				}
				else if let Ok(Some(map_diagnostic)) = Self::gather_from_pinned_object::<MapFileDescriptor, ExtendedBerkeleyPacketFilterMapDiagnostic>(&pinned_object_fully_qualified_file_path, ExtendedBerkeleyPacketFilterMapDiagnostic::gather)
				{
					pinned_map_diagnostics.insert(pinned_object_fully_qualified_file_path, map_diagnostic);
				}
				else if let Ok(Some(bpf_type_format_diagnostic)) = Self::gather_from_pinned_object::<BpfTypeFormatFileDescriptor, ExtendedBerkeleyPacketFilterTypeFormatDiagnostic>(&pinned_object_fully_qualified_file_path, ExtendedBerkeleyPacketFilterTypeFormatDiagnostic::gather)
				{
					pinned_bpf_type_format_diagnostics.insert(pinned_object_fully_qualified_file_path, bpf_type_format_diagnostic);
				}
			});
		}
		(pinned_program_diagnostics, pinned_map_diagnostics, pinned_bpf_type_format_diagnostics)
	}
	
	/// There is no way to know in advance whether an `absolute_path` is represented by either of `ExtendedBpfProgramFileDescriptor`, `BpfTypeFormatFileDescriptor`, or `MapFileDescriptor`.
	///
	/// A returned value of `Err()` implies that the file path itself is not one of `ExtendedBpfProgramFileDescriptor`, `BpfTypeFormatFileDescriptor`, or `MapFileDescriptor`, or does not exist or is not a file.
	fn gather_from_pinned_object<BFD: BpfFileDescriptor, I>(absolute_path: &impl AsRef<Path>, gather_information: impl FnOnce(&BFD) -> Result<I, Errno>) -> DiagnosticUnobtainableResult<Option<I>>
	{
		let file_descriptor = BFD::get_pinned_absolute_path(absolute_path, KernelOnlyAccessPermissions::KernelReadAndWriteUserspaceReadWrite).map_err(|errno| DiagnosticUnobtainable::from(format!("Could not open pinned Extended BPF file descriptor: {}", errno)))?;
		Ok(gather_information(&file_descriptor).ok())
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An eBPF program loaded using the `bpf()` syscall.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedBpfProgramTemplate<'name>
{
	/// Program type.
	pub program_type: ProgramType,
	
	/// Program name.
	#[serde(default = "ExtendedBpfProgramTemplate::program_name_default")] pub program_name: ProgramName,
	
	/// Program licence (usually GPL).
	#[serde(default)] pub license: BpfProgramLicense,
	
	/// If omitted, BTF type definitions are not produced.
	#[serde(skip)] pub bpf_type_format_program_details: Option<BpfTypeFormatProgramDetails>,
	
	/// Lines of the program.
	///
	/// There must be at least one line.
	#[serde(borrow)] pub program_lines: ProgramLines<'name>,
}

impl<'name> ExtendedBpfProgramTemplate<'name>
{
	/// Wraps parse and load with defaults for common arguments.
	///
	/// Creates a VerifierLog of 16Kb.
	///
	/// Assumes the program (`self`) has no `Offset::Named` (`'name`) memory offsets or immediates to resolve.
	#[inline(always)]
	pub fn convenient_load(&self, map_file_descriptors: &FileDescriptorsMap<MapFileDescriptor>, extended_bpf_program_file_descriptors: &mut FileDescriptorsMap<ExtendedBpfProgramFileDescriptor>) -> Result<Rc<ExtendedBpfProgramFileDescriptor>, ProgramLoadError>
	{
		let arguments = ExtendedBpfProgramArguments::new(&map_file_descriptors, extended_bpf_program_file_descriptors);
		self.parse_and_load(arguments, Some(VerifierLog::default())).map(|(extended_bpf_program_file_descriptor, _verifier_log)| extended_bpf_program_file_descriptor)
	}
	
	/// Parse and load.
	#[inline(always)]
	pub fn parse_and_load(&self, arguments: ExtendedBpfProgramArguments, verifier_log: Option<VerifierLog>) -> Result<(Rc<ExtendedBpfProgramFileDescriptor>, Option<VerifierLog>), ProgramLoadError>
	{
		let (instructions, parsed_bpf_type_format_data, extended_bpf_program_file_descriptors, verifier_log) = ProgramLinesParser::parse(self.bpf_type_format_program_details.as_ref(), &self.program_lines, arguments, verifier_log)?;
		
		let (extended_bpf_program_file_descriptor, verifier_log) = self.load(&instructions[..], parsed_bpf_type_format_data.as_ref(), extended_bpf_program_file_descriptors, verifier_log)?;
		
		let extended_bpf_program_file_descriptor = extended_bpf_program_file_descriptors.add(self.program_name.clone(), extended_bpf_program_file_descriptor)?;
		Ok((extended_bpf_program_file_descriptor, verifier_log))
	}
	
	#[inline(always)]
	fn load(&self, instructions: &[bpf_insn], parsed_bpf_type_format_data: Option<&ParsedBpfTypeFormatData>, extended_bpf_program_file_descriptors: &FileDescriptorsMap<ExtendedBpfProgramFileDescriptor>, mut verifier_log: Option<VerifierLog>) -> Result<(ExtendedBpfProgramFileDescriptor, Option<VerifierLog>), ProgramLoadError>
	{
		let (log_level, log_buf, log_size) = VerifierLog::to_values_for_syscall(verifier_log.as_mut());
		
		let (prog_type, expected_attach_type, attach_btf_id, attach_prog_fd, kern_version, prog_ifindex) = self.program_type.to_values(&extended_bpf_program_file_descriptors)?;
		
		let
		(
			prog_btf_fd,
			(func_info_rec_size, func_info, func_info_cnt),
			(line_info_rec_size, line_info, line_info_cnt),
		) = ParsedBpfTypeFormatData::optionally_to_bpf_load_data(parsed_bpf_type_format_data)?;
		
		let mut attributes = bpf_attr
		{
			program_load: BpfCommandProgramLoad
			{
				prog_type,
				insn_cnt: Self::instruction_count(instructions)?,
				insns: AlignedU64::from(instructions),
				license: self.license.as_aligned_u64(),
				log_level,
				log_size,
				log_buf,
				kern_version,
				prog_flags: BPF_PROG_LOAD_flags::BPF_F_STRICT_ALIGNMENT,
				prog_name: self.program_name.clone().into(),
				prog_ifindex,
				expected_attach_type,
				
				prog_btf_fd,
				func_info_rec_size,
				func_info,
				func_info_cnt,
				line_info_rec_size,
				line_info,
				line_info_cnt,
				
				attach_btf_id,
				attach_prog_fd,
			}
		};
		
		let result = attributes.syscall(bpf_cmd::BPF_PROG_LOAD);
		
		if likely!(result >= 0)
		{
			Ok((unsafe { ExtendedBpfProgramFileDescriptor::from_raw_fd(result) }, verifier_log))
		}
		else if likely!(result == -1)
		{
			use self::ProgramLoadError::*;
			
			match SystemCallErrorNumber::from_errno()
			{
				ENOSPC => if log_level > 0
				{
					Err(NotEnoughSpaceForVerifierLogMessages)
				}
				else
				{
					unreachable_code(format_args!(""))
				},
				
				EBADF => unreachable_code(format_args!("")),
				
				EACCES | EINVAL | E2BIG => Err(InvalidProgram(verifier_log.map(|verifier_log| verifier_log.into()))),
				
				ENOMEM => Err(OutOfMemoryOrResources),
				
				EPERM => Err(PermissionDenied),
					
				EFAULT => panic!("Memory fault"),
				
				unexpected @ _ => panic!("Unexpected error `{}`", unexpected),
			}
		}
		else
		{
			unreachable_code(format_args!("result `{}` from bpf(BPF_PROG_LOAD) was unexpected", result))
		}
	}
	
	#[inline(always)]
	fn instruction_count(instructions: &[bpf_insn]) -> Result<u32, ProgramLoadError>
	{
		use self::ParseError::*;
		use self::ProgramLoadError::*;
		
		const UpperLimit: usize = u32::MAX as usize;
		match instructions.len()
		{
			0 => Err(Parse(ThereAreNoInstructions)),
			
			length @ 1 ..= UpperLimit => Ok(length as u32),
			
			_ => Err(Parse(ThereAreMoreThanU32MaxInstructions)),
		}
	}
	
	#[inline(always)]
	fn program_name_default() -> ProgramName
	{
		ProgramName::from_bytes(b"bpf_program").unwrap()
	}
}

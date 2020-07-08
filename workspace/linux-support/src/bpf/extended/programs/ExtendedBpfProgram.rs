// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An eBPF program loaded using the `bpf()` syscall.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct ExtendedBpfProgramTemplate<'name>
{
	/// Program type.
	pub program_type: ProgramType<'name>,
	
	/// Program name, if any.
	#[serde(default)] pub program_name: Option<ProgramName>,
	
	/// Program licence (usually GPL).
	#[serde(default)] pub license: BpfProgramLicense,
	
	/// If omitted, BTF type definitions are not produced.
	#[serde(skip)] pub btf_program_details: Option<BtfProgramDetails>,
	
	/// Lines of the program.
	///
	/// There must be at least one line.
	pub program_lines: Vec<ProgramLine<'name>>,
}

impl<'name> ExtendedBpfProgramTemplate<'name>
{
	#[inline(always)]
	pub fn parse_and_load(&self, arguments: ExtendedBpfProgramArguments, verifier_log: Option<&mut VerifierLog>) -> Result<RawFd, ProgramLoadError>
	{
		let (instructions, parsed_btf_data, extended_bpf_program_file_descriptor_labels_map) = ProgramLinesParser::parse(self.btf_program_details.as_ref(), &self.program_lines, arguments)?;
		
		self.load(&instructions[..], parsed_btf_data.as_ref(), extended_bpf_program_file_descriptor_labels_map, verifier_log)
	}
	
	fn load(&self, instructions: &[bpf_insn], parsed_btf_data: Option<&ParsedBtfData>, extended_bpf_program_file_descriptor_labels_map: FileDescriptorLabelsMap<ExtendedBpfProgramFileDescriptor>, verifier_log: Option<&mut VerifierLog>) -> Result<RawFd, ProgramLoadError>
	{
		let (log_level, log_buf, log_size) = VerifierLog::to_values_for_syscall(verifier_log)?;
		
		let (prog_type, expected_attach_type, attach_btf_id, attach_prog_fd, kern_version, prog_ifindex) = self.program_type.to_values(&extended_bpf_program_file_descriptor_labels_map)?;
		extended_bpf_program_file_descriptor_labels_map.guard_all_values_have_been_resolved_at_least_once()?;
		
		let
		(
			prog_btf_fd,
			(func_info_rec_size, func_info, func_info_cnt),
			(line_info_rec_size, line_info, line_info_cnt),
		) = ParsedBtfData::optionally_to_bpf_load_data(parsed_btf_data)?;
		
		let mut attr = bpf_attr
		{
			program_load: BpfCommandProgramLoad
			{
				prog_type,
				insn_cnt: Self::instruction_count(instructions)?,
				insns: AlignedU64::from(instructions),
				license: AlignedU64::from(self.license.0.as_ptr()),
				log_level,
				log_size,
				log_buf,
				kern_version,
				prog_flags: BPF_PROG_LOAD_flags::BPF_F_STRICT_ALIGNMENT,
				prog_name: match self.program_name
				{
					None => unsafe { zeroed() },
					Some(program_name) => program_name.prog_name()
				},
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
		
		let result = bpf(bpf_cmd::BPF_PROG_LOAD, &mut attributes, size_of::<bpf_attr>() as u32);
		
		if likely!(result >= 0)
		{
			return Ok(result)
		}
		else if likely!(result == -1)
		{
			// TODO: See tools/lib/bpf/bpf.c: bpf_load_program_xattr.
			match errno().0
			{
				// TODO: If the size of the buffer is not large enough to store all verifier messages, -1 is returned and errno is set to ENOSPC.
				ENOSPC @ if log_level > 0 =>
				{
					// not enough log space
				}
				
				// 'btf_data_size > BTF_MAX_SIZE' in btf_parse()
				E2BIG,
				
				ENOMEM
					
					EFAULT
			}
		}
		else
		{
			unreachable!("result `{}` was unexpected from bpf()", result)
		}
	}
	
	#[inline(always)]
	fn instruction_count(instructions: &[bpf_insn]) -> Result<u32, ProgramLoadError>
	{
		use self::ProgramError::*;
		use self::ProgramLoadError::*;
		
		const UpperLimit: usize = u32::MAX as usize;
		let insn_cnt = match instructions.len()
		{
			0 => Err(Program(ThereAreNoInstructions)),
			
			length @ 1 ..= UpperLimit => Ok(length as u32),
			
			_ => Err(Program(ThereAreMoreThanU32MaxInstructions)),
		};
	}
	
	#[inline(always)]
	fn verifier_log(verifier_log: VerifierLog) -> Result<(u32, AlignedU64, u32), ProgramLoadError>
	{
		match verifier_log
		{
			None => Ok((0, AlignedU64::Null, 0)),
			
			Some((verifier_log_level, log_buffer)) =>
			{
				use self::ProgramLoadError::*;
				
				// Checks deduced from Linux kernel function `btf_parse()`.
				let buffer_size = log_buffer.len();
				if unlikely!(buffer_size < 128)
				{
					return Err(VerifierLogBufferMustBeAtLeast128Bytes)
				}
				
				const ExclusiveMaximumBufferSize: usize = (1 << 24) as usize;
				if unlikely!(buffer_size >= ExclusiveMaximumBufferSize)
				{
					return Err(VerifierLogBufferMustBeLessThan2ToThe24Bytes)
				}
				
				Ok
				(
					(
						verifier_log_level as u32,
						AlignedU64::from(log_buffer),
						buffer_size as u32,
					)
				)
			}
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct BtfTypeInformationParser
{
	btf_type_identifiers: BtfTypeIdentifiers,
	function_information: Vec<bpf_func_info>,
	line_information: Vec<bpf_line_info>,
	file_name_off: u32,
}

impl BtfTypeInformationParser
{
	#[inline(always)]
	pub(crate) fn new(number_of_functions_size_hint: usize, file_name: &str) -> Self
	{
		let mut btf_type_identifiers = BtfTypeIdentifiers::default();
		file_name_off = unsafe { transmute(btf_type_identifiers.push_any(file_name)?) };
		
		Self
		{
			btf_type_identifiers,
			function_information: Vec::with_capacity(number_of_functions_size_hint),
			line_information: Vec::with_capacity(number_of_functions_size_hint),
			file_name_off,
		}
	}
	
	#[inline(always)]
	pub(crate) fn finish(self) -> Result<Option<ParsedBtfData>, ProgramError>
	{
		// There must always be at least one function information and one line information.
		if self.function_information.is_empty() || self.line_information.is_empty()
		{
			return Ok(None)
		}
		
		let header_and_type_identifier_section_and_string_section = self.btf_type_identifiers.finish()?;
		
		// TODO: btf_check_sec_info();

// TODO: eg attach_kprobe for kprobe and uprobe but not kretprobe and uretprobe
// eg attach_tp for TRACEPOINT (tracepoint_category and tracepoint_name) use a pfd - perfevent fd using perf_event_open_probe
// eg attach_raw_tp for RAW_TRACEPOINT (tracepoint_name)
// eg attach_trace for TRACING and EXT and LSM
		
		
		Ok
		(
			Some
			(
				ParsedBtfData
				{
					btf_file_descriptor: XXXXX,
					function_information: self.function_information.into_boxed_slice(),
					line_information: self.line_information.into_boxed_slice()
				}
			)
		)
	}
	
	#[inline(always)]
	pub(crate) fn push_relative_function_definition(&mut self, name: &Name, function_prototype: Option<&FunctionPrototype>, current_program_counter: ProgramCounter, line_number: u32) -> Result<(), ProgramError>
	{
		if let Some(function_prototype) = function_prototype
		{
			let function_type_identifier = self.btf_type_identifiers.create_function(name, function_prototype)?;
			
			let insn_off = current_program_counter.0 as u32;
			self.function_information.push
			(
				bpf_func_info
				{
					insn_off,
					type_identifier: function_type_identifier
				}
			);
			
			debug_assert!(line_number <= bpf_line_info::InclusiveMaximumLineNumber, "Should have been guarded for before processing");
			
			let column_number = 0;
			if unlikely!(column_number > bpf_line_info::InclusiveMaximumColumnNumber)
			{
				return Err(ProgramError::LineColumnNumberExceedsMaximum)
			}
			
			let line_off = unsafe { transmute(self.btf_type_identifiers.push_any(&format!("{}", function_prototype))?) };
			self.line_information.push
			(
				bpf_line_info
				{
					insn_off,
					file_name_off: self.file_name_off,
					line_off,
					line_col: (line_number << 10 | column_number)
				}
			);
		}
		
		Ok(())
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct BpfTypeFormatInformationParser
{
	bpf_type_format_type_identifiers: BpfTypeFormatTypeIdentifiers,
	function_information: Vec<bpf_func_info>,
	line_information: Vec<bpf_line_info>,
	file_name_off: u32,
}

impl BpfTypeFormatInformationParser
{
	#[inline(always)]
	pub(crate) fn new(number_of_functions_size_hint: usize, file_name: &str) -> Result<Self, BpfTypeFormatError>
	{
		let mut bpf_type_format_type_identifiers = BpfTypeFormatTypeIdentifiers::default();
		let file_name_off = unsafe { transmute(bpf_type_format_type_identifiers.push_any(file_name)?) };
		
		Ok
		(
			Self
			{
				bpf_type_format_type_identifiers,
				function_information: Vec::with_capacity(number_of_functions_size_hint),
				line_information: Vec::with_capacity(number_of_functions_size_hint),
				file_name_off,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn finish(self, verifier_log: Option<VerifierLog>) -> Result<(Option<ParsedBpfTypeFormatData>, Option<VerifierLog>), ParseError>
	{
		// There must always be at least one function information and one line information.
		if self.function_information.is_empty() || self.line_information.is_empty()
		{
			return Ok((None, verifier_log))
		}
		
		let header_and_type_identifier_section_and_string_section = self.bpf_type_format_type_identifiers.finish()?;
		
		let (bpf_type_format_file_descriptor, verifier_log) = BpfTypeFormatFileDescriptor::load_data(&header_and_type_identifier_section_and_string_section[..], verifier_log)?;
		
		Ok
		(
			(
				Some
				(
					ParsedBpfTypeFormatData
					{
						bpf_type_format_file_descriptor,
						function_information: self.function_information.into_boxed_slice(),
						line_information: self.line_information.into_boxed_slice()
					}
				),
				verifier_log,
			)
		)
	}
	
	#[inline(always)]
	pub(crate) fn push_relative_function_definition(&mut self, name: &Name, function_prototype: Option<&FunctionPrototype>, current_program_counter: ProgramCounter, line_number: u32) -> Result<(), ParseError>
	{
		if let Some(function_prototype) = function_prototype
		{
			let function_type_identifier = self.bpf_type_format_type_identifiers.create_function(name, function_prototype)?;
			
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
				return Err(ParseError::LineColumnNumberExceedsMaximum)
			}
			
			let line_off = unsafe { transmute(self.bpf_type_format_type_identifiers.push_any(&format!("{}", function_prototype))?) };
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

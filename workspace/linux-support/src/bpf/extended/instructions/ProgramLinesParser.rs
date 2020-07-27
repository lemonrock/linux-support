// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `ProgramLine` parser.
#[derive(Debug, Clone)]
pub struct ProgramLinesParser<'name>
{
	instructions: Vec<bpf_insn>,
	
	labels_to_program_counters: HashMap<Name<'name>, ProgramCounter>,
	jump_instructions_labelled_offset_to_resolve: HashMap<Name<'name>, Vec<ProgramCounter>>,
	
	relative_function_names_to_program_counters: HashMap<Name<'name>, ProgramCounter>,
	jump_instructions_relative_function_named_offset_to_resolve: HashMap<Name<'name>, Vec<ProgramCounter>>,

	line_number: u32,
	bpf_type_format_type_information_parser: Option<BpfTypeFormatInformationParser>,
}

impl<'name> ProgramLinesParser<'name>
{
	/// Process instructions.
	///
	/// If `bpf_type_format_program_details` is `None`, no function or line information is produced.
	pub fn parse<'map_file_descriptor_label_map, 'extended_bpf_program_file_descriptor_label_map>(bpf_type_format_program_details: Option<&BpfTypeFormatProgramDetails>, program_lines: &Vec<ProgramLine<'name>>, arguments: ExtendedBpfProgramArguments<'map_file_descriptor_label_map, 'extended_bpf_program_file_descriptor_label_map>, verifier_log: Option<&mut VerifierLog>) -> Result<(Box<[bpf_insn]>, Option<ParsedBpfTypeFormatData>, &'extended_bpf_program_file_descriptor_label_map mut FileDescriptorsMap<ExtendedBpfProgramFileDescriptor>), ProgramError>
	{
		let number_of_program_lines = program_lines.len();
		if unlikely!(number_of_program_lines > bpf_line_info::MaximumNumberOfProgramLines)
		{
			return Err(ProgramError::TooManyProgramLines)
		}
		
		let instructions_size_hint = number_of_program_lines * 2;
		let number_of_jumps_size_hint = number_of_program_lines / 16;
		let number_of_functions_size_hint = number_of_program_lines / 32;
		
		let this = Self
		{
			instructions: Vec::with_capacity(instructions_size_hint),
			labels_to_program_counters: HashMap::default(),
			jump_instructions_labelled_offset_to_resolve: HashMap::with_capacity(number_of_jumps_size_hint),
			relative_function_names_to_program_counters: HashMap::with_capacity(number_of_functions_size_hint),
			jump_instructions_relative_function_named_offset_to_resolve: HashMap::with_capacity(number_of_jumps_size_hint),
			
			line_number: 0,
			bpf_type_format_type_information_parser: if let Some(bpf_type_format_program_details) = bpf_type_format_program_details
			{
				Some(BpfTypeFormatInformationParser::new(number_of_functions_size_hint, bpf_type_format_program_details.file_name)?)
			}
			else
			{
				None
			},
		};
		
		this.parse_internal(program_lines, bpf_type_format_program_details, arguments, verifier_log)
	}
	
	#[inline(always)]
	fn parse_internal<'map_file_descriptor_label_map, 'extended_bpf_program_file_descriptor_label_map>(mut self, program_lines: &Vec<ProgramLine<'name>>, bpf_type_format_program_details: Option<&BpfTypeFormatProgramDetails>, arguments: ExtendedBpfProgramArguments<'map_file_descriptor_label_map, 'extended_bpf_program_file_descriptor_label_map>, verifier_log: Option<&mut VerifierLog>) -> Result<(Box<[bpf_insn]>, Option<ParsedBpfTypeFormatData>, &'extended_bpf_program_file_descriptor_label_map mut FileDescriptorsMap<ExtendedBpfProgramFileDescriptor>), ProgramError>
	{
		use self::ProgramError::*;
		
		let ExtendedBpfProgramArguments { i32_immediates_map, u64_immediates_map, memory_offsets_map, map_file_descriptors: map_file_descriptors, extended_bpf_program_file_descriptors } = arguments;
		
		if let Some(&BpfTypeFormatProgramDetails { ref main_function, .. }) = bpf_type_format_program_details
		{
			self.push_relative_function_definition(&"main".into(), Some(main_function))?;
		}
		
		for program_line in program_lines
		{
			program_line.parse(&mut self, &i32_immediates_map, &u64_immediates_map, &memory_offsets_map, map_file_descriptors)?;
			self.line_number += 1;
		}
		
		if unlikely!(self.number_of_instructions() == 0)
		{
			return Err(ThereAreNoInstructions)
		}
		
		if !self.jump_instructions_labelled_offset_to_resolve.is_empty()
		{
			return Err(SomeJumpLabelsAreUnresolved)
		}
		
		if !self.jump_instructions_relative_function_named_offset_to_resolve.is_empty()
		{
			return Err(SomeRelativeFunctionNamesAreUnresolved)
		}
		
		i32_immediates_map.guard_all_values_have_been_resolved_at_least_once()?;
		u64_immediates_map.guard_all_values_have_been_resolved_at_least_once()?;
		memory_offsets_map.guard_all_values_have_been_resolved_at_least_once()?;
		
		let instructions = self.instructions.into_boxed_slice();
		let parsed_bpf_type_format_data = match self.bpf_type_format_type_information_parser
		{
			None => None,
			Some(bpf_type_format_type_information_parser) => bpf_type_format_type_information_parser.finish(verifier_log)?,
		};
		
		Ok
		(
			(
				instructions,
				parsed_bpf_type_format_data,
				extended_bpf_program_file_descriptors,
			)
		)
	}
	
	/// Registering a label more than once is permitted; the latest registration 'wins'.
	#[inline(always)]
	pub(crate) fn register_label(&mut self, label: &Name<'name>) -> Result<(), ProgramError>
	{
		Self::register(label, self.current_program_counter(), &mut self.labels_to_program_counters, &mut self.jump_instructions_labelled_offset_to_resolve, &mut self.instructions)
	}
	
	/// Registering a relative function name more than once is permitted; the latest registration 'wins'.
	#[inline(always)]
	pub(crate) fn register_relative_function_name(&mut self, relative_function_name: &Name<'name>, function_prototype: Option<&FunctionPrototype>) -> Result<(), ProgramError>
	{
		self.push_relative_function_definition(relative_function_name, function_prototype)?;
		
		Self::register(relative_function_name, self.current_program_counter(), &mut self.relative_function_names_to_program_counters, &mut self.jump_instructions_relative_function_named_offset_to_resolve, &mut self.instructions)
	}
	
	#[inline(always)]
	fn push_relative_function_definition(&mut self, relative_function_name: &Name<'name>, function_prototype: Option<&FunctionPrototype>) -> Result<(), ProgramError>
	{
		let current_program_counter = self.current_program_counter();
		if let Some(ref mut bpf_type_format_type_information_parser) = self.bpf_type_format_type_information_parser
		{
			bpf_type_format_type_information_parser.push_relative_function_definition(relative_function_name, function_prototype, current_program_counter, self.line_number)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn resolve_label(&mut self, program_counter_offset: &ProgramCounterOffset<'name, i16>) -> Result<i16, ProgramError>
	{
		Self::resolve::<i16>(program_counter_offset, self.current_program_counter(), &mut self.labels_to_program_counters, &mut self.jump_instructions_labelled_offset_to_resolve)
	}
	
	#[inline(always)]
	pub(crate) fn resolve_relative_function_name(&mut self, program_counter_offset: &ProgramCounterOffset<'name, i32>) -> Result<i32, ProgramError>
	{
		Self::resolve::<i32>(program_counter_offset, self.current_program_counter(), &mut self.labels_to_program_counters, &mut self.jump_instructions_labelled_offset_to_resolve)
	}
	
	#[inline(always)]
	fn register(name: &Name<'name>, name_program_counter: ProgramCounter, name_to_program_counters: &mut HashMap<Name<'name>, ProgramCounter>, jump_instructions_to_resolve: &mut HashMap<Name<'name>, Vec<ProgramCounter>>, instructions: &mut Vec<bpf_insn>) -> Result<(), ProgramError>
	{
		let first_registration_of_name = name_to_program_counters.insert(name.clone(), name_program_counter).is_none();
		
		if first_registration_of_name
		{
			if let Some(jump_instructions_to_resolve_program_counter) = jump_instructions_to_resolve.remove(name)
			{
				for from_program_counter in jump_instructions_to_resolve_program_counter
				{
					let jump_instruction_to_resolve = unsafe { instructions.get_unchecked_mut(from_program_counter.0) };
					if unlikely!(jump_instruction_to_resolve.is_relative_call())
					{
						let offset = name_program_counter.i32_offset(from_program_counter)?;
						jump_instruction_to_resolve.set_jump_immediate(offset)
					}
					else
					{
						let offset = name_program_counter.i16_offset(from_program_counter)?;
						jump_instruction_to_resolve.set_jump_offset(offset)
					}
				}
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn resolve<PCOV: ProgramCounterOffsetValue>(program_counter_offset: &ProgramCounterOffset<'name, PCOV>, current_program_counter: ProgramCounter, names_to_program_counters: &mut HashMap<Name<'name>, ProgramCounter>, jump_instructions_to_resolve: &mut HashMap<Name<'name>, Vec<ProgramCounter>>) -> Result<PCOV, ProgramError>
	{
		use self::Offset::*;
		match program_counter_offset.as_ref()
		{
			&Known(program_counter_offset_value) => Ok(program_counter_offset_value),
			
			Named(ref name) =>
			{
				if let Some(program_counter) = names_to_program_counters.get(name)
				{
					let label = *program_counter;
					PCOV::program_counter_offset_value(label, current_program_counter)
				}
				else
				{
					let jump_instructions_labelled_offset_to_resolve = jump_instructions_to_resolve.entry(name.clone()).or_insert(Vec::new());
					jump_instructions_labelled_offset_to_resolve.push(current_program_counter);
					Ok(PCOV::Invalid)
				}
			}
		}
	}
	
	#[inline(always)]
	fn current_program_counter(&self) -> ProgramCounter
	{
		ProgramCounter(self.number_of_instructions())
	}
	
	#[inline(always)]
	fn one_instruction(&mut self, one_instruction: bpf_insn) -> Result<(), ProgramError>
	{
		const One: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
		self.maximum_instructions_guard(One)?;
		
		self.instructions.push(one_instruction);
		Ok(())
	}
	
	#[inline(always)]
	fn two_instructions(&mut self, two_instructions: [bpf_insn; 2]) -> Result<(), ProgramError>
	{
		const Two: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(2) };
		self.maximum_instructions_guard(Two)?;
		
		self.instructions.extend_from_slice(&two_instructions[..]);
		Ok(())
	}
	
	#[inline(always)]
	fn maximum_instructions_guard(&self, number_of_instructions_to_add: NonZeroUsize) -> Result<(), ProgramError>
	{
		const MaximumInstructionsOnRecentLinux: usize = 1_000_000;
		if self.number_of_instructions() + (number_of_instructions_to_add.get() - 1) >= MaximumInstructionsOnRecentLinux
		{
			Err(ProgramError::MaximumNumberOfInstructionsUsed)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn number_of_instructions(&self) -> usize
	{
		self.instructions.len()
	}
}

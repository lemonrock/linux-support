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
	btf_type_information_parser: Option<BtfTypeInformationParser>,
}

impl<'name> ProgramLinesParser<'name>
{
	/// Process instructions.
	///
	/// If `btf_program_details` is `None`, no function or line information is produced.
	pub fn parse<'file_descriptor>(btf_program_details: Option<&BtfProgramDetails>, program_lines: &Vec<ProgramLine<'name>>, arguments: ExtendedBpfProgramArguments<'file_descriptor>) -> Result<(Box<[bpf_insn]>, Option<ParsedBtfData>, FileDescriptorLabelsMap<'file_descriptor, ExtendedBpfProgramFileDescriptor>), ProgramError>
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
			btf_type_information_parser: if let Some(btf_program_details) = btf_program_details
			{
				Some(BtfTypeInformationParser::new(number_of_functions_size_hint, btf_program_details.file_name))
			}
			else
			{
				None
			},
		};
		
		this.parse_internal(program_lines, btf_program_details, arguments)
	}
	
	#[inline(always)]
	fn parse_internal<'file_descriptor>(mut self, program_lines: &Vec<ProgramLine<'name>>, btf_program_details: Option<&BtfProgramDetails>, arguments: ExtendedBpfProgramArguments<'file_descriptor>) -> Result<(Box<[bpf_insn]>, Option<ParsedBtfData>, FileDescriptorLabelsMap<'file_descriptor, ExtendedBpfProgramFileDescriptor>), ProgramError>
	{
		use self::ProgramError::*;
		
		let ExtendedBpfProgramArguments { i32_immediates_map, u64_immediates_map, memory_offsets_map, map_file_descriptor_labels_map, extended_bpf_program_file_descriptor_labels_map } = arguments;
		
		if let Some(&BtfProgramDetails { ref main_function, .. }) = btf_program_details
		{
			self.push_relative_function_definition("main".into(), Some(main_function))?;
		}
		
		for program_line in program_lines
		{
			program_line.parse(&mut self, &i32_immediates_map, &u64_immediates_map, &memory_offsets_map, &map_file_descriptor_labels_map)?;
			self.line_number += 1;
		}
		
		if unlikely!(self.number_of_instructions() == 0)
		{
			Err(ThereAreNoInstructions)
		}
		else
		{
			Ok(())
		}
		
		if !self.jump_instructions_labelled_offset_to_resolve.is_empty()
		{
			Err(SomeJumpLabelsAreUnresolved)
		}
		
		if !self.jump_instructions_relative_function_named_offset_to_resolve.is_empty()
		{
			Err(SomeRelativeFunctionNamesAreUnresolved)
		}
		
		i32_immediates_map.guard_all_values_have_been_resolved_at_least_once()?;
		u64_immediates_map.guard_all_values_have_been_resolved_at_least_once()?;
		memory_offsets_map.guard_all_values_have_been_resolved_at_least_once()?;
		map_file_descriptor_labels_map.guard_all_values_have_been_resolved_at_least_once()?;
		
		let instructions = self.instructions.into_boxed_slice();
		let parsed_btf_data = match self.btf_type_information_parser
		{
			None => None,
			Some(btf_type_information_parser) => btf_type_information_parser.finish(),
		};
		
		Ok
		(
			(
				instructions,
				parsed_btf_data,
				extended_bpf_program_file_descriptor_labels_map,
			)
		)
	}
	
	/// Registering a label more than once is permitted; the latest registration 'wins'.
	#[inline(always)]
	pub(crate) fn register_label(&mut self, label: &Name<'name>) -> Result<(), ProgramError>
	{
		Self::register(label, self.current_program_counter(), &mut self.labels_to_program_counters, &mut self.jump_instructions_labelled_offset_to_resolve)
	}
	
	/// Registering a relative function name more than once is permitted; the latest registration 'wins'.
	#[inline(always)]
	pub(crate) fn register_relative_function_name(&mut self, relative_function_name: &Name<'name>, function_prototype: Option<&FunctionPrototype>) -> Result<(), ProgramError>
	{
		if let Some(ref btf_type_information_parser) = self.btf_type_information_parser
		{
			btf_type_information_parser.push_relative_function_definition(relative_function_name, function_prototype, self.current_program_counter(), self.line_number)?;
		}
		
		Self::register(relative_function_name, self.current_program_counter(), &mut self.relative_function_names_to_program_counters, &mut self.jump_instructions_relative_function_named_offset_to_resolve)
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
	fn register(name: &Name<'name>, name_program_counter: ProgramCounter, name_to_program_counters: &mut HashMap<Name<'name>, ProgramCounter>, jump_instructions_to_resolve: &mut HashMap<Name<'name>, Vec<ProgramCounter>>) -> Result<(), ProgramError>
	{
		let first_registration_of_name = name_to_program_counters.insert(name.clone(), name_program_counter).is_none();
		
		if first_registration_of_name
		{
			if let Some(jump_instructions_to_resolve) = jump_instructions_to_resolve.remove(name)
			{
				for jump_instruction_to_resolve in jump_instructions_to_resolve
				{
					if unlikely!(jump_instruction_to_resolve.is_relative_call())
					{
						let offset = name_program_counter.i32_offset_to_label(jump_instruction_to_resolve)?;
						jump_instruction_to_resolve.set_jump_immediate(offset)
					}
					else
					{
						let offset = name_program_counter.i16_offset_to_label(jump_instruction_to_resolve)?;
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
		self.maximum_instructions_guard(unsafe { NonZeroUsize::new(1) });
		
		self.instructions.push(one_instruction);
		Ok(())
	}
	
	#[inline(always)]
	fn two_instructions(&mut self, two_instructions: [bpf_insn; 2]) -> Result<(), ProgramError>
	{
		self.maximum_instructions_guard(unsafe { NonZeroUsize::new(2) });
		
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

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instructions<'name>
{
	instructions: Vec<bpf_insn>,
	labels_to_program_counters: HashMap<Name<'name>, ProgramCounter>,
	jump_instructions_labelled_offset_to_resolve: HashMap<Name<'name>, Vec<ProgramCounter>>,
}

impl<'name> Instructions<'name>
{
	/// Process instructions.
	pub fn process(instruction_stream: &mut impl Iterator<Item=&'name Instruction<'name>>, i32_immediates_map: &OffsetsMap<i32>, u64_immediates_map: &OffsetsMap<u64>, memory_offsets_map: &OffsetsMap<i16>, map_file_descriptor_labels_map: &MapFileDescriptorLabelsMap) -> Result<Box<[bpf_insn]>, InstructionError>
	{
		let mut instructions = Self
		{
			instructions: Vec::default(),
			labels_to_program_counters: HashMap::default(),
			jump_instructions_labelled_offset_to_resolve: HashMap::default(),
		};
		
		for instruction in instruction_stream
		{
			instruction.add_to_instructions(&mut instructions, i32_immediates_map, u64_immediates_map, memory_offsets_map, map_file_descriptor_labels_map)?;
		}
		
		instructions.validate_all_labels_resolved()
	}
	
	#[inline(always)]
	fn validate_all_labels_resolved(self) -> Result<Box<[bpf_insn]>, InstructionError>
	{
		if self.jump_instructions_labelled_offset_to_resolve.is_empty()
		{
			Ok(self.instructions.into_boxed_slice())
		}
		else
		{
			Err(InstructionError::SomeJumpLabelsAreUnresolved)
		}
	}
	
	/// Registering a label more than once is permitted; the latest registration 'wins'.
	#[inline(always)]
	fn register_label(&mut self, label: &Name<'name>) -> Result<(), InstructionError>
	{
		let label_program_counter = self.current_program_counter();
		let first_registration_of_label = self.labels_to_program_counters.insert(label.clone(), label_program_counter).is_none();
		
		if first_registration_of_label
		{
			if let Some(jump_instructions_labelled_offset_to_resolve) = self.jump_instructions_labelled_offset_to_resolve.remove(label)
			{
				for jump_instruction_labelled_offset_to_resolve in jump_instructions_labelled_offset_to_resolve
				{
					let jump_instruction = self.instructions.get_mut(jump_instruction_labelled_offset_to_resolve.0).unwrap();
				
					if unlikely!(jump_instruction.is_relative_call())
					{
						let offset = label_program_counter.i32_offset_to_label(jump_instruction_labelled_offset_to_resolve)?;
						jump_instruction.set_jump_immediate(offset)
					}
					else
					{
						let offset = label_program_counter.i16_offset_to_label(jump_instruction_labelled_offset_to_resolve)?;
						jump_instruction.set_jump_offset(offset)
					}
				}
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn resolve_label<PCOV: ProgramCounterOffsetValue>(&mut self, program_counter_offset: &ProgramCounterOffset<'name, PCOV>) -> Result<PCOV, InstructionError>
	{
		use self::Offset::*;
		match program_counter_offset.as_ref()
		{
			&Known(program_counter_offset_value) => Ok(program_counter_offset_value),
			
			Named(ref name) =>
			{
				let current_program_counter = self.current_program_counter();
				if let Some(program_counter) = self.labels_to_program_counters.get(name)
				{
					let label = *program_counter;
					PCOV::program_counter_offset_value(label, current_program_counter)
				}
				else
				{
					let jump_instructions_labelled_offset_to_resolve = self.jump_instructions_labelled_offset_to_resolve.entry(name.clone()).or_insert(Vec::new());
					jump_instructions_labelled_offset_to_resolve.push(current_program_counter);
					Ok(PCOV::Invalid)
				}
			}
		}
	}
	
	#[inline(always)]
	fn current_program_counter(&self) -> ProgramCounter
	{
		ProgramCounter(self.instructions.len())
	}
	
	#[inline(always)]
	fn one_instruction(&mut self, one_instruction: bpf_insn) -> Result<(), InstructionError>
	{
		self.instructions.push(one_instruction);
		Ok(())
	}
	
	#[inline(always)]
	fn two_instructions(&mut self, two_instructions: [bpf_insn; 2]) -> Result<(), InstructionError>
	{
		self.instructions.extend_from_slice(&two_instructions[..]);
		Ok(())
	}
}

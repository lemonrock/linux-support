// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An instruction that can be deserialized or serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Instruction<'de>
{
	/// Represents a label used for conditional and non-conditional jumps.
	Label(Name<'de>),
	
	/// Load a true 64-bit value.
	///
	/// `destination_register = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `immediate`.
	Load64Immediate(Register, Immediate<'de, u64>),
	
	/// Load a map file descriptor
	///
	/// `destination_register = map_file_descriptor`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `map_file_descriptor`.
	LoadMapFileDescriptor(Register, MapFileDescriptorLabel<'de>),
	
	/// Load a map value.
	///
	/// `destination_register = map_file_descriptor`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `map_file_descriptor`.
	/// * `2`: `offset_into_value`.
	LoadMapValue(Register, MapFileDescriptorLabel<'de>, Immediate<'de, i32>),
	
	///
	///
	/// # Fields
	///
	/// * `0`: `operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source_register` or `immediate`.
	Alu32(AluOperation, Register, RegisterOrImmediate<'de>),
	
	///
	///
	/// # Fields
	///
	/// * `0`: `operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source_register` or `immediate`.
	Alu64(AluOperation, Register, RegisterOrImmediate<'de>),
	
	/// ?Uncertain of Encoding?
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `length` (eg `16` or `32`).
	ToLittleEndian(Register, Immediate<'de, i32>),
	
	/// ?Uncertain of Encoding?
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `length` (eg `16` or `32`).
	ToBigEndian(Register, Immediate<'de, i32>),
	
	/// Move of lower 32 bits.
	///
	/// `destination_register = source_register`.
	/// `destination_register = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register` or `immediate`.
	Move32(Register, RegisterOrImmediate<'de>),
	
	/// Move of all 64 bits.
	///
	/// `destination_register = source_register`.
	/// `destination_register = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register` or `immediate`.
	Move64(Register, RegisterOrImmediate<'de>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u8).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct8(Immediate<'de, i32>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u16).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct16(Immediate<'de, i32>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u32).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct32(Immediate<'de, i32>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u64).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct64(Immediate<'de, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u8).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect8(Register, Immediate<'de, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u16).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect16(Register, Immediate<'de, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u32).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect32(Register, Immediate<'de, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u64).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect64(Register, Immediate<'de, i32>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u8).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory8(Register, Register, MemoryOffset<'de>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u16).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory16(Register, Register, MemoryOffset<'de>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u32).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory32(Register, Register, MemoryOffset<'de>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u64).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory64(Register, Register, MemoryOffset<'de>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u8).add(memory_offset)) = source_register`.
	/// `*((destination_register as *mut u8).add(memory_offset)) = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register` or `immediate`.
	/// * `2`: `memory_offset`.
	StoreToMemory8(Register, RegisterOrImmediate<'de>, MemoryOffset<'de>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u16).add(memory_offset)) = source_register`.
	/// `*((destination_register as *mut u16).add(memory_offset)) = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register` or `immediate`.
	/// * `2`: `memory_offset`.
	StoreToMemory16(Register, RegisterOrImmediate<'de>, MemoryOffset<'de>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u32).add(memory_offset)) = source_register`.
	/// `*((destination_register as *mut u32).add(memory_offset)) = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register` or `immediate`.
	/// * `2`: `memory_offset`.
	StoreToMemory32(Register, RegisterOrImmediate<'de>, MemoryOffset<'de>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u64).add(memory_offset)) = source_register`.
	/// `*((destination_register as *mut u64).add(memory_offset)) = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register` or `immediate`.
	/// * `2`: `memory_offset`.
	StoreToMemory64(Register, RegisterOrImmediate<'de>, MemoryOffset<'de>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u8).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd8(Register, Register, MemoryOffset<'de>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u16).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd16(Register, Register, MemoryOffset<'de>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u32).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd32(Register, Register, MemoryOffset<'de>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u64).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd64(Register, Register, MemoryOffset<'de>),
	
	/// Conditional jump after comparison of lower 32 bits.
	///
	/// ```bash
	/// if destination_register jump_operation source_register
	/// {
	/// 	goto program_counter + program_counter_offset
	/// }
	/// ```
	/// ```bash
	/// if destination_register jump_operation immediate
	/// {
	/// 	goto program_counter + program_counter_offset
	/// }
	/// ```
	///
	/// `program_counter` is also known as `pc`.
	///
	/// # Fields
	///
	/// * `0`: `jump_operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source_register` or `immediate`.
	/// * `3`: `program_counter_offset`.
	ConditionalJump32(JumpOperation, Register, RegisterOrImmediate<'de>, ProgramCounterOffset<'de>),
	
	/// Conditional jump after comparison of all 64 bits.
	///
	/// ```bash
	/// if destination_register jump_operation source_register
	/// {
	/// 	goto program_counter + program_counter_offset
	/// }
	/// ```
	/// ```bash
	/// if destination_register jump_operation immediate
	/// {
	/// 	goto program_counter + program_counter_offset
	/// }
	/// ```
	///
	/// `program_counter` is also known as `pc`.
	///
	///
	/// # Fields
	///
	/// * `0`: `jump_operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source_register` or `immediate`.
	/// * `3`: `program_counter_offset`.
	ConditionalJump64(JumpOperation, Register, RegisterOrImmediate<'de>, ProgramCounterOffset<'de>),
	
	/// Unconditional jump.
	///
	/// `goto program_counter + program_counter_offset`.
	///
	/// `program_counter` is also known as `pc`.
	///
	///
	/// # Fields
	///
	/// * `0`: `program_counter_offset`.
	UnconditionalJump(ProgramCounterOffset<'de>),
	
	/// Function call.
	///
	/// Registers `r1` through to `r5` inclusive are used to pass function arguments and are clobbered.
	/// The function result will be returned in `r0`.
	/// The function `bpf_tail_call()` never returns if successfully invoked.
	///
	/// `call function_identifier`.
	///
	///
	/// # Fields
	///
	/// * `0`: `function_identifier`.
	FunctionCall(bpf_func_id),
	
	/// Relative function call.
	///
	/// Calls a BPF function within the same block of instructions.
	///
	/// Registers `r1` through to `r5` inclusive are used to pass function arguments and are clobbered.
	/// The function result will be returned in `r0`.
	RelativeCall(LargerProgramCounterOffset<'de>),
	
	/// Program exit.
	///
	/// Returns the value in the register `r0`.
	ProgramExit,
}

impl<'de> Instruction<'de>
{
	/// Add to instruction(s).
	#[inline(always)]
	pub fn add_to_instructions(&self, instructions: &mut Instructions, i32_immediates_map: &OffsetsMap<i32>, u64_immediates_map: &OffsetsMap<u64>, memory_offsets_map: &MemoryOffsetsMap, map_file_descriptor_labels_map: &MapFileDescriptorLabelsMap) -> Result<(), InstructionError>
	{
		use self::Instruction::*;
		use self::EndiannessOperation::*;
		use self::RegisterOrImmediate::*;
		
		let instruction = match self
		{
			&Label(ref label) => return instructions.register_label(label),
			
			&Load64Immediate(destination_register, ref immediate) => return instructions.two_instructions(bpf_insn::load64_immediate(destination_register, u64_immediates_map.resolve(immediate)?)),
			
			&LoadMapFileDescriptor(destination_register, ref map_file_descriptor_label) => return instructions.two_instructions(bpf_insn::load_map_file_descriptor(destination_register, map_file_descriptor_labels_map.resolve(map_file_descriptor_label)?)),
			
			&LoadMapValue(destination_register, ref map_file_descriptor_label, ref offset_into_value) => return instructions.two_instructions(bpf_insn::load_map_value(destination_register, map_file_descriptor_labels_map.resolve(map_file_descriptor_label)?, i32_immediates_map.resolve(offset_into_value)?)),
			
			&Alu32(operation, destination_register, Register(source_register)) => bpf_insn::alu32(operation, destination_register, source_register),
			
			&Alu32(operation, destination_register, Immediate(ref immediate)) => bpf_insn::alu32_immediate(operation, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&Alu64(operation, destination_register, Register(source_register)) => bpf_insn::alu64(operation, destination_register, source_register),
			
			&Alu64(operation, destination_register, Immediate(ref immediate)) => bpf_insn::alu64_immediate(operation, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&ToLittleEndian(destination_register, ref immediate) => bpf_insn::endian(ToLittleEndian, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&ToBigEndian(destination_register, ref immediate) => bpf_insn::endian(ToBigEndian, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&Move32(destination_register, Register(source_register)) => bpf_insn::move32(destination_register, source_register),
			
			&Move32(destination_register, Immediate(ref immediate)) => bpf_insn::move32_immediate(destination_register, i32_immediates_map.resolve(immediate)?),
			
			&Move64(destination_register, Register(source_register)) => bpf_insn::move64(destination_register, source_register),
			
			&Move64(destination_register, Immediate(ref immediate)) => bpf_insn::move64_immediate(destination_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct8(ref immediate) => bpf_insn::load8_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct16(ref immediate) => bpf_insn::load16_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct32(ref immediate) => bpf_insn::load32_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct64(ref immediate) => bpf_insn::load64_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect8(source_register, ref immediate) => bpf_insn::load8_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect16(source_register, ref immediate) => bpf_insn::load16_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect32(source_register, ref immediate) => bpf_insn::load32_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect64(source_register, ref immediate) => bpf_insn::load64_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadFromMemory8(destination_register, source_register, ref memory_offset) => bpf_insn::load8_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&LoadFromMemory16(destination_register, source_register, ref memory_offset) => bpf_insn::load16_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&LoadFromMemory32(destination_register, source_register, ref memory_offset) => bpf_insn::load32_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&LoadFromMemory64(destination_register, source_register, ref memory_offset) => bpf_insn::load64_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory8(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store8_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory8(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store8_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemory16(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store16_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory16(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store16_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemory32(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store32_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory32(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store32_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemory64(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store64_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory64(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store64_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemoryAtomicAdd8(destination_register, source_register, ref memory_offset) => bpf_insn::store8_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemoryAtomicAdd16(destination_register, source_register, ref memory_offset) => bpf_insn::store16_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemoryAtomicAdd32(destination_register, source_register, ref memory_offset) => bpf_insn::store32_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemoryAtomicAdd64(destination_register, source_register, ref memory_offset) => bpf_insn::store64_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&ConditionalJump32(jump_operation, destination_register, Register(source_register), ref program_counter_offset) => bpf_insn::conditional_jump32(jump_operation, destination_register, source_register, instructions.resolve_label(program_counter_offset)?),
			
			&ConditionalJump32(jump_operation, destination_register, Immediate(ref immediate), ref program_counter_offset) => bpf_insn::conditional_jump32_immediate(jump_operation, destination_register, i32_immediates_map.resolve(immediate)?, instructions.resolve_label(program_counter_offset)?),
			
			&ConditionalJump64(jump_operation, destination_register, Register(source_register), ref program_counter_offset) => bpf_insn::conditional_jump64(jump_operation, destination_register, source_register, instructions.resolve_label(program_counter_offset)?),
			
			&ConditionalJump64(jump_operation, destination_register, Immediate(ref immediate), ref program_counter_offset) => bpf_insn::conditional_jump64_immediate(jump_operation, destination_register, i32_immediates_map.resolve(immediate)?, instructions.resolve_label(program_counter_offset)?),
			
			&UnconditionalJump(ref program_counter_offset) => bpf_insn::unconditional_jump(instructions.resolve_label(program_counter_offset)?),
			
			&FunctionCall(function_identifier) => bpf_insn::function_call(function_identifier),
			
			&RelativeCall(ref larger_program_counter_offset) => bpf_insn::relative_call(program_counter_offsets_map.resolve_larger(larger_program_counter_offset)?),
			
			&ProgramExit => bpf_insn::program_exit(),
		};
		
		instructions.one_instruction(instruction)
	}
}

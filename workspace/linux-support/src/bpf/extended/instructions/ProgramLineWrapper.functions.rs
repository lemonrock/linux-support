// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Block or subroutine.
#[inline(always)]
pub fn block<'name>(program_lines: Vec<ProgramLineWrapper<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLineWrapper::ProgramLines(program_lines)
}

/// Represents a label used for conditional and non-conditional jumps.
///
/// ***CAUTION***: emits ***NO*** instructions - be aware when manually counting jump offsets!
#[inline(always)]
pub fn label<'name>(name: impl Into<Name<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::Label(name.into()).into()
}

/// Represents a label used for relative function calls
///
/// ***CAUTION***: emits ***NO*** instructions - be aware when manually counting jump offsets!
#[inline(always)]
pub fn function<'name>(name: impl Into<Name<'name>>, function_prototype: Option<FunctionPrototype>) -> ProgramLineWrapper<'name>
{
	ProgramLine::Function(name.into(), function_prototype).into()
}

/// Load a true 64-bit value.
///
/// `destination_register = immediate`.
///
/// ***CAUTION***: emits ***2*** instructions - be aware when manually counting jump offsets!
#[inline(always)]
pub fn load_immediate_64<'name>(destination_register: Register, immediate: impl Into<Immediate<'name, u64>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadImmediate64(destination_register, immediate.into()).into()
}

/// Load a map file descriptor.
///
/// `destination_register = map_file_descriptor`.
///
/// ***CAUTION***: emits ***2*** instructions - be aware when manually counting jump offsets!
#[inline(always)]
pub fn load_map_file_descriptor<MN: TryInto<MapName>>(destination_register: Register, map_name: MN) -> ProgramLineWrapper<'static>
where MN::Error: Debug
{
	ProgramLine::LoadMapFileDescriptor(destination_register, map_name.try_into().unwrap()).into()
}

/// Load a map value.
///
/// `destination_register = map_file_descriptor`.
///
/// ***CAUTION***: emits ***2*** instructions - be aware when manually counting jump offsets!
#[inline(always)]
pub fn load_map_value<'name, MN: TryInto<MapName>>(destination_register: Register, map_name: MN, offset_into_value: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
where MN::Error: Debug
{
	ProgramLine::LoadMapValue(destination_register, map_name.try_into().unwrap(), offset_into_value.into()).into()
}

/// Operation on 32-bits of values.
///
/// `destination_register = destination_register operation source`.
#[inline(always)]
pub fn alu_32<'name>(operation: AluOperation, destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::Alu32(operation, destination_register, source.into()).into()
}

/// Operation on all 64-bits of values.
///
/// `destination_register = destination_register operation source`.
#[inline(always)]
pub fn alu_64<'name>(operation: AluOperation, destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::Alu64(operation, destination_register, source.into()).into()
}

/// ?Uncertain of Encoding?
#[inline(always)]
pub fn to_little_endian<'name>(destination_register: Register, length: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::ToLittleEndian(destination_register, length.into()).into()
}

/// ?Uncertain of Encoding?
#[inline(always)]
pub fn to_big_endian<'name>(destination_register: Register, length: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::ToBigEndian(destination_register, length.into()).into()
}

/// Move of lower 32 bits.
///
/// `destination_register = source`.
#[inline(always)]
pub fn move_32<'name>(destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::Move32(destination_register, source.into()).into()
}

/// Move of all 64 bits.
///
/// `destination_register = source`.
#[inline(always)]
pub fn move_64<'name>(destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::Move64(destination_register, source.into()).into()
}

/// Move of stack pointer.
///
/// `destination_register = stack_pointer`.
#[inline(always)]
pub fn move_stack_pointer(destination_register: Register) -> ProgramLineWrapper<'static>
{
	move_64(destination_register, Register::fp).into()
}

/// Direct packet access.
///
/// Uses a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u8).add(immediate))`.
#[inline(always)]
pub fn load_r0_direct_8<'name>(immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Direct8(immediate.into()).into()
}

/// Direct packet access.
///
/// Uses a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u16).add(immediate))`.
#[inline(always)]
pub fn load_r0_direct_16<'name>(immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Direct16(immediate.into()).into()
}

/// Direct packet access.
///
/// Uses a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u32).add(immediate))`.
#[inline(always)]
pub fn load_r0_direct_32<'name>(immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Direct32(immediate.into()).into()
}

/// Direct packet access.
///
/// Uses a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u64).add(immediate))`.
#[inline(always)]
pub fn load_r0_direct_64<'name>(immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Direct64(immediate.into()).into()
}

/// Indirect packet access.
///
/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u8).add(source_register + immediate))`.
#[inline(always)]
pub fn load_r0_indirect_8<'name>(source_register: Register, immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Indirect8(source_register, immediate.into()).into()
}

/// Indirect packet access.
///
/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u16).add(source_register + immediate))`.
#[inline(always)]
pub fn load_r0_indirect_16<'name>(source_register: Register, immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Indirect16(source_register, immediate.into()).into()
}

/// Indirect packet access.
///
/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u32).add(source_register + immediate))`.
#[inline(always)]
pub fn load_r0_indirect_32<'name>(source_register: Register, immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Indirect32(source_register, immediate.into()).into()
}

/// Indirect packet access.
///
/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
///
/// `r0 = *((skb.data as *const u64).add(source_register + immediate))`.
#[inline(always)]
pub fn load_r0_indirect_64<'name>(source_register: Register, immediate: impl Into<Immediate<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadR0Indirect64(source_register, immediate.into()).into()
}

/// Memory load.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((source_register as *const u8).add(memory_offset))`.
#[inline(always)]
pub fn load_from_memory_8<'name>(destination_register: Register, source_register: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadFromMemory8(destination_register, source_register, memory_offset.into()).into()
}

/// Memory load.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((source_register as *const u16).add(memory_offset))`.
#[inline(always)]
pub fn load_from_memory_16<'name>(destination_register: Register, source_register: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadFromMemory16(destination_register, source_register, memory_offset.into()).into()
}

/// Memory load.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((source_register as *const u32).add(memory_offset))`.
#[inline(always)]
pub fn load_from_memory_32<'name>(destination_register: Register, source_register: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadFromMemory32(destination_register, source_register, memory_offset.into()).into()
}

/// Memory load.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((source_register as *const u64).add(memory_offset))`.
#[inline(always)]
pub fn load_from_memory_64<'name>(destination_register: Register, source_register: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::LoadFromMemory64(destination_register, source_register, memory_offset.into()).into()
}

/// Memory load from stack.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((stack_pointer as *const u8).sub(variable_slot))`.
#[inline(always)]
pub fn load_from_stack_variable_8<VS: TryInto<VariableSlotU64>>(destination_register: Register, variable_slot: VS) -> ProgramLineWrapper<'static>
where VS::Error: Debug
{
	load_from_memory_8(destination_register, Register::fp, VariableSlotU64::to_memory_slot_from_try_into(variable_slot)).into()
}

/// Memory load from stack.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((stack_pointer as *const u16).sub(variable_slot * 2))`.
#[inline(always)]
pub fn load_from_stack_variable_16<VS: TryInto<VariableSlotU64>>(destination_register: Register, variable_slot: VS) -> ProgramLineWrapper<'static>
where VS::Error: Debug
{
	load_from_memory_16(destination_register, Register::fp, VariableSlotU64::to_memory_slot_from_try_into(variable_slot)).into()
}

/// Memory load from stack.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((stack_pointer as *const u64).sub(variable_slot * 4))`.
#[inline(always)]
pub fn load_from_stack_variable_32<VS: TryInto<VariableSlotU64>>(destination_register: Register, variable_slot: VS) -> ProgramLineWrapper<'static>
where VS::Error: Debug
{
	load_from_memory_32(destination_register, Register::fp, VariableSlotU64::to_memory_slot_from_try_into(variable_slot)).into()
}

/// Memory load from stack.
///
/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
///
/// `destination_register = *((stack_pointer as *const u64).sub(variable_slot * 8))`.
#[inline(always)]
pub fn load_from_stack_variable_64<VS: TryInto<VariableSlotU64>>(destination_register: Register, variable_slot: VS) -> ProgramLineWrapper<'static>
where VS::Error: Debug
{
	load_from_memory_64(destination_register, Register::fp, VariableSlotU64::to_memory_slot_from_try_into(variable_slot)).into()
}

/// Memory store.
///
/// `*((destination_register as *mut u8).add(memory_offset)) = source`.
#[inline(always)]
pub fn store_to_memory_8<'name>(destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemory8(destination_register, source.into(), memory_offset.into()).into()
}

/// Memory store.
///
/// `*((destination_register as *mut u16).add(memory_offset)) = source`.
#[inline(always)]
pub fn store_to_memory_16<'name>(destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemory16(destination_register, source.into(), memory_offset.into()).into()
}

/// Memory store.
///
/// `*((destination_register as *mut u32).add(memory_offset)) = source`.
#[inline(always)]
pub fn store_to_memory_32<'name>(destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemory32(destination_register, source.into(), memory_offset.into()).into()
}

/// Memory store.
///
/// `*((destination_register as *mut u64).add(memory_offset)) = source`.
#[inline(always)]
pub fn store_to_memory_64<'name>(destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemory64(destination_register, source.into(), memory_offset.into()).into()
}

/// Memory store to stack.
///
/// `*((stack_pointer as *mut u8).sub(variable_slot)) = source`.
#[inline(always)]
pub fn store_to_stack_variable_8<'name, VS: TryInto<VariableSlotU64>>(source: impl Into<RegisterOrImmediate<'name>>, variable_slot: VS) -> ProgramLineWrapper<'name>
where VS::Error: Debug
{
	store_to_memory_8(Register::fp, source.into(), VariableSlotU64::to_memory_slot_from_try_into(variable_slot))
}

/// Memory store to stack.
///
/// `*((stack_pointer as *mut u16).sub(variable_slot * 2)) = source`.
#[inline(always)]
pub fn store_to_stack_variable_16<'name, VS: TryInto<VariableSlotU64>>(source: impl Into<RegisterOrImmediate<'name>>, variable_slot: VS) -> ProgramLineWrapper<'name>
where VS::Error: Debug
{
	store_to_memory_16(Register::fp, source.into(), VariableSlotU64::to_memory_slot_from_try_into(variable_slot))
}

/// Memory store to stack.
///
/// `*((stack_pointer as *mut u32).sub(variable_slot * 4)) = source`.
#[inline(always)]
pub fn store_to_stack_variable_32<'name, VS: TryInto<VariableSlotU64>>(source: impl Into<RegisterOrImmediate<'name>>, variable_slot: VS) -> ProgramLineWrapper<'name>
where VS::Error: Debug
{
	store_to_memory_32(Register::fp, source.into(), VariableSlotU64::to_memory_slot_from_try_into(variable_slot))
}

/// Memory store to stack.
///
/// `*((stack_pointer as *mut u64).sub(variable_slot * 8)) = source`.
#[inline(always)]
pub fn store_to_stack_variable_64<'name, VS: TryInto<VariableSlotU64>>(source: impl Into<RegisterOrImmediate<'name>>, variable_slot: VS) -> ProgramLineWrapper<'name>
where VS::Error: Debug
{
	store_to_memory_64(Register::fp, source.into(), VariableSlotU64::to_memory_slot_from_try_into(variable_slot))
}

/// Memory store using an atomic add.
///
/// `*((destination_register as *mut u8).add(memory_offset)) += source_register`.
#[inline(always)]
pub fn store_to_memory_atomic_add_8<'name>(destination_register: Register, source: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemoryAtomicAdd8(destination_register, source, memory_offset.into()).into()
}

/// Memory store using an atomic add.
///
/// `*((destination_register as *mut u16).add(memory_offset)) += source_register`.
#[inline(always)]
pub fn store_to_memory_atomic_add_16<'name>(destination_register: Register, source: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemoryAtomicAdd16(destination_register, source, memory_offset.into()).into()
}

/// Memory store using an atomic add.
///
/// `*((destination_register as *mut u32).add(memory_offset)) += source_register`.
#[inline(always)]
pub fn store_to_memory_atomic_add_32<'name>(destination_register: Register, source: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemoryAtomicAdd32(destination_register, source, memory_offset.into()).into()
}

/// Memory store using an atomic add.
///
/// `*((destination_register as *mut u64).add(memory_offset)) += source_register`.
#[inline(always)]
pub fn store_to_memory_atomic_add_64<'name>(destination_register: Register, source: Register, memory_offset: impl Into<MemoryOffset<'name>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::StoreToMemoryAtomicAdd64(destination_register, source, memory_offset.into()).into()
}

/// Conditional jump after comparison of lower 32 bits.
///
/// ```bash
/// if destination_register jump_operation source
/// {
/// 	goto program_counter + program_counter_offset
/// }
/// ```
///
/// `program_counter` is also known as `pc`.
#[inline(always)]
pub fn conditional_jump_32<'name>(jump_operation: JumpOperation, destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>, program_counter_offset: impl Into<ProgramCounterOffset<'name, i16>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::ConditionalJump32(jump_operation, destination_register, source.into(), program_counter_offset.into()).into()
}

/// Conditional jump after comparison of all 64 bits.
///
/// ```bash
/// if destination_register jump_operation source
/// {
/// 	goto program_counter + program_counter_offset
/// }
/// ```
///
/// `program_counter` is also known as `pc`.
#[inline(always)]
pub fn conditional_jump_64<'name>(jump_operation: JumpOperation, destination_register: Register, source: impl Into<RegisterOrImmediate<'name>>, program_counter_offset: impl Into<ProgramCounterOffset<'name, i16>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::ConditionalJump64(jump_operation, destination_register, source.into(), program_counter_offset.into()).into()
}

/// Unconditional jump.
///
/// `goto program_counter + program_counter_offset`.
///
/// `program_counter` is also known as `pc`.
#[inline(always)]
pub fn unconditional_jump<'name>(program_counter_offset: impl Into<ProgramCounterOffset<'name, i16>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::UnconditionalJump(program_counter_offset.into()).into()
}

/// Function call.
///
/// Registers `r1` through to `r5` inclusive are used to pass function arguments and are clobbered.
/// The function result will be returned in `r0`.
/// The function `bpf_tail_call()` never returns if successfully invoked.
///
/// `call function_identifier`.
#[inline(always)]
pub fn function_call(function_identifier: bpf_func_id) -> ProgramLineWrapper<'static>
{
	ProgramLine::FunctionCall(function_identifier).into()
}

/// Relative function call.
///
/// Calls a BPF function within the same block of instructions.
///
/// Registers `r1` through to `r5` inclusive are used to pass function arguments and are clobbered.
/// The function result will be returned in `r0`.
#[inline(always)]
pub fn relative_function_call<'name>(program_counter_offset: impl Into<ProgramCounterOffset<'name, i32>>) -> ProgramLineWrapper<'name>
{
	ProgramLine::RelativeFunctionCall(program_counter_offset.into()).into()
}

/// Program exit.
#[inline(always)]
pub fn program_exit() ->  ProgramLineWrapper<'static>
{
	ProgramLine::ProgramExit.into()
}

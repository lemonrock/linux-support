// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// BPF instruction.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_insn
{
	code: u8,
	destination_and_source_registers: DestinationAndSourceRegisters,
	off: i16,
	imm: i32,
}

impl bpf_insn
{
	/// `BPF_ALU64_REG(OP, DST, SRC)`.
	#[inline(always)]
	pub const fn alu64(operation: AluOperation, destination_register: Register, source_register: Register) -> Self
	{
		Self::new
		(
			BPF_ALU64 | operation as u8 | (BPF_X as u8),
			destination_register,
			source_register,
			0,
			0,
		)
	}
	
	/// `BPF_ALU32_REG(OP, DST, SRC)`.
	#[inline(always)]
	pub const fn alu32(operation: AluOperation, destination_register: Register, source_register: Register) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | operation as u8 | (BPF_X as u8),
			destination_register,
			source_register,
			0,
			0,
		)
	}
	
	/// `BPF_ALU64_IMM(OP, DST, IMM)`.
	#[inline(always)]
	pub const fn alu64_immediate(operation: AluOperation, destination_register: Register, immediate: i32) -> Self
	{
		Self::new
		(
			BPF_ALU64 | operation as u8 | (BPF_K as u8),
			destination_register,
			Register::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_ALU32_IMM(OP, DST, IMM)`.
	#[inline(always)]
	pub const fn alu32_immediate(operation: AluOperation, destination_register: Register, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | operation as u8 | (BPF_K as u8),
			destination_register,
			Register::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_ENDIAN(TYPE, DST, LEN)`.
	///
	/// Endianness conversion.
	///
	/// Values of length seen in the wild are `16` and `32`; it seems zero-extension occurs when using these values.
	pub const fn endian(endianness: EndiannessOperation, destination_register: Register, length: i32) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | BPF_END | endianness as u8,
			destination_register,
			Register::r0,
			0,
			length,
		)
	}
	
	/// `BPF_MOV64_REG(DST, SRC)`.
	#[inline(always)]
	pub const fn move64(destination_register: Register, source_register: Register) -> Self
	{
		Self::new
		(
			BPF_ALU64 | BPF_MOV | (BPF_X as u8),
			destination_register,
			source_register,
			0,
			0,
		)
	}
	
	/// `BPF_MOV32_REG(DST, SRC)`.
	#[inline(always)]
	pub const fn move32(destination_register: Register, source_register: Register) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | BPF_MOV | (BPF_X as u8),
			destination_register,
			source_register,
			0,
			0,
		)
	}
	
	/// `BPF_MOV64_IMM(DST, IMM)`.
	#[inline(always)]
	pub const fn move64_immediate(destination_register: Register, immediate: i32) -> Self
	{
		Self::new
		(
			BPF_ALU64 | BPF_MOV | (BPF_K as u8),
			destination_register,
			Register::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_MOV32_IMM(DST, IMM)`.
	#[inline(always)]
	pub const fn move32_immediate(destination_register: Register, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | BPF_MOV | (BPF_K as u8),
			destination_register,
			Register::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_LD_IMM64(DST, IMM)`.
	///
	/// Load a 64-bit immediate.
	#[inline(always)]
	pub const fn load64_immediate(destination_register: Register, immediate: u64) -> [Self; 2]
	{
		Self::load64_immediate_raw(destination_register, Register::r0, immediate)
	}
	
	/// `BPF_LD_MAP_FD(DST, MAP_FD)`.
	#[inline(always)]
	pub const fn load_map_file_descriptor(destination_register: Register, map_file_descriptor: RawFd) -> [Self; 2]
	{
		Self::load64_immediate_raw_full(destination_register, Register::BPF_PSEUDO_MAP_FD, 0, 0, map_file_descriptor, 0)
	}
	
	/// `BPF_LD_MAP_VALUE(DST, MAP_FD, VALUE_OFF)`.
	#[inline(always)]
	pub const fn load_map_value(destination_register: Register, map_file_descriptor: RawFd, offset_into_value: i32) -> [Self; 2]
	{
		Self::load64_immediate_raw_full(destination_register, Register::BPF_PSEUDO_MAP_VALUE, 0, 0, map_file_descriptor, offset_into_value)
	}
	
	/// `BPF_LD_ABS(BPF_B, IMM)`.
	///
	/// "Direct packet access, `r0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load8_r0_direct(immediate: i32) -> Self
	{
		Self::load_r0_direct(_8, immediate)
	}
	
	/// `BPF_LD_ABS(BPF_H, IMM)`.
	///
	/// "Direct packet access, `r0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load16_r0_direct(immediate: i32) -> Self
	{
		Self::load_r0_direct(_16, immediate)
	}
	
	/// `BPF_LD_ABS(BPF_W, IMM)`.
	///
	/// "Direct packet access, `r0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load32_r0_direct(immediate: i32) -> Self
	{
		Self::load_r0_direct(_32, immediate)
	}
	
	/// `BPF_LD_ABS(BPF_DW, IMM)`.
	///
	/// "Direct packet access, `r0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load64_r0_direct(immediate: i32) -> Self
	{
		Self::load_r0_direct(_64, immediate)
	}
	
	/// `BPF_LD_IND(BPF_B, SRC, IMM)`.
	///
	/// Indirect packet access, `r0 = *(uint *) (skb->data + source_register + immediate)`.
	#[inline(always)]
	pub const fn load8_r0_indirect(source_register: Register, immediate: i32) -> Self
	{
		Self::load_r0_indirect(_8, source_register, immediate)
	}
	
	/// `BPF_LD_IND(BPF_H, SRC, IMM)`.
	///
	/// Indirect packet access, `r0 = *(uint *) (skb->data + source_register + immediate)`.
	#[inline(always)]
	pub const fn load16_r0_indirect(source_register: Register, immediate: i32) -> Self
	{
		Self::load_r0_indirect(_16, source_register, immediate)
	}
	
	/// `BPF_LD_IND(BPF_W, SRC, IMM)`.
	///
	/// Indirect packet access, `r0 = *(uint *) (skb->data + source_register + immediate)`.
	#[inline(always)]
	pub const fn load32_r0_indirect(source_register: Register, immediate: i32) -> Self
	{
		Self::load_r0_indirect(_32, source_register, immediate)
	}
	
	/// `BPF_LD_IND(BPF_DW, SRC, IMM)`.
	///
	/// Indirect packet access, `r0 = *(uint *) (skb->data + source_register + immediate)`.
	#[inline(always)]
	pub const fn load64_r0_indirect(source_register: Register, immediate: i32) -> Self
	{
		Self::load_r0_indirect(_64, source_register, immediate)
	}
	
	/// `BPF_LDX_MEM(BPF_B, DST, SRC, OFF)`.
	///
	/// "Memory load, `destination_register = *(uint *) (source_register + memory_offset)`".
	#[inline(always)]
	pub const fn load8_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::load_memory(_8, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_LDX_MEM(BPF_H, DST, SRC, OFF)`.
	///
	/// "Memory load, `destination_register = *(uint *) (source_register + memory_offset)`".
	#[inline(always)]
	pub const fn load16_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::load_memory(_16, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_LDX_MEM(BPF_W, DST, SRC, OFF)`.
	///
	/// "Memory load, `destination_register = *(uint *) (source_register + memory_offset)`".
	#[inline(always)]
	pub const fn load32_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::load_memory(_32, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_LDX_MEM(BPF_DW, DST, SRC, OFF)`.
	///
	/// "Memory load, `destination_register = *(uint *) (source_register + memory_offset)`".
	#[inline(always)]
	pub const fn load64_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::load_memory(_64, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_MEM(BPF_B, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (destination_register + off16) = source_register`".
	#[inline(always)]
	pub const fn store8_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory(_8, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_MEM(BPF_H, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (destination_register + off16) = source_register`".
	#[inline(always)]
	pub const fn store16_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory(_16, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_MEM(BPF_W, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (destination_register + off16) = source_register`".
	#[inline(always)]
	pub const fn store32_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory(_32, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_MEM(BPF_DW, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (destination_register + off16) = source_register`".
	#[inline(always)]
	pub const fn store64_memory(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory(_64, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_XADD(BPF_B, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(destination_register + off16) += source_register`".
	#[inline(always)]
	pub const fn store8_memory_atomic_add(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_8, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_XADD(BPF_H, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(destination_register + off16) += source_register`".
	#[inline(always)]
	pub const fn store16_memory_atomic_add(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_16, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_XADD(BPF_W, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(destination_register + off16) += source_register`".
	#[inline(always)]
	pub const fn store32_memory_atomic_add(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_32, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_STX_XADD(BPF_DW, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(destination_register + off16) += source_register`".
	#[inline(always)]
	pub const fn store64_memory_atomic_add(destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_64, destination_register, source_register, memory_offset)
	}
	
	/// `BPF_ST_MEM(BPF_B, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store8_memory_immediate(destination_register: Register, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_8, destination_register, offset, immediate)
	}
	
	/// `BPF_ST_MEM(BPF_H, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store16_memory_immediate(destination_register: Register, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_16, destination_register, offset, immediate)
	}
	
	/// `BPF_ST_MEM(BPF_W, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store32_memory_immediate(destination_register: Register, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_32, destination_register, offset, immediate)
	}
	
	/// `BPF_ST_MEM(BPF_DW, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store64_memory_immediate(destination_register: Register, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_64, destination_register, offset, immediate)
	}
	
	/// `BPF_JMP_REG(OP, DST, SRC, OFF)`.
	///
	/// "Conditional jumps against registers, `if (destination_register 'op' source_register) goto pc + offset`".
	///
	/// Uses a 64-bit comparision.
	#[inline(always)]
	pub const fn conditional_jump64(jump_operation: JumpOperation, destination_register: Register, source_register: Register, program_counter_offset: i16) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | jump_operation as u8 | (BPF_X as u8),
			destination_register,
			source_register,
			program_counter_offset,
			0,
		)
	}
	
	/// `BPF_JMP32_REG(OP, DST, SRC, OFF)`.
	///
	/// "Conditional jumps against registers, `if (destination_register 'op' source_register) goto pc + offset`".
	///
	/// Uses a 32-bit comparision of the lower 64-bits of registers.
	#[inline(always)]
	pub const fn conditional_jump32(jump_operation: JumpOperation, destination_register: Register, source_register: Register, program_counter_offset: i16) -> Self
	{
		Self::new
		(
			BPF_JMP32 | jump_operation as u8 | (BPF_X as u8),
			destination_register,
			source_register,
			program_counter_offset,
			0,
		)
	}
	
	/// `BPF_JMP_IMM(OP, DST, IMM, OFF)`.
	///
	/// "Conditional jumps against immediates, `if (destination_register 'op' immediate) goto pc + offset`".
	///
	/// Uses a 64-bit comparision.
	#[inline(always)]
	pub const fn conditional_jump64_immediate(jump_operation: JumpOperation, destination_register: Register, immediate: i32, program_counter_offset: i16) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | jump_operation as u8 | (BPF_K as u8),
			destination_register,
			Register::r0,
			program_counter_offset,
			immediate,
		)
	}
	
	/// `BPF_JMP32_IMM(OP, DST, SRC, OFF)`.
	///
	/// "Conditional jumps against immediates, `if (destination_register 'op' immediate) goto pc + offset`".
	///
	/// Uses a 32-bit comparision of the lower 64-bits of registers.
	#[inline(always)]
	pub const fn conditional_jump32_immediate(jump_operation: JumpOperation, destination_register: Register, immediate: i32, program_counter_offset: i16) -> Self
	{
		Self::new
		(
			BPF_JMP32 | jump_operation as u8 | (BPF_K as u8),
			destination_register,
			Register::r0,
			program_counter_offset,
			immediate,
		)
	}
	
	/// `BPF_JMP_A(OFF)`.
	///
	/// "Unconditional jumps, `goto pc + offset`".
	#[inline(always)]
	pub const fn unconditional_jump(program_counter_offset: i16) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | (BPF_JA as u8),
			Register::r0,
			Register::r0,
			program_counter_offset,
			0,
		)
	}
	
	/// `BPF_EMIT_CALL(FUNC)`.
	///
	/// Function call.
	#[inline(always)]
	pub const fn function_call(function_identifier: bpf_func_id) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | BPF_CALL,
			Register::r0,
			Register::r0,
			0,
			(function_identifier as i32) - (bpf_func_id::BPF_FUNC_unspec as i32),
		)
	}
	
	/// `BPF_CALL_REL(TGT)`.
	///
	/// Relative call.
	#[inline(always)]
	pub const fn relative_call(program_counter_offset_to_another_bpf_function: i32) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | BPF_CALL,
			Register::r0,
			Register::BPF_PSEUDO_CALL,
			0,
			program_counter_offset_to_another_bpf_function,
		)
	}
	
	#[inline(always)]
	pub(crate) fn is_relative_call(&self) -> bool
	{
		(self.code == (BPF_JMP as u8) | BPF_CALL) && self.destination_and_source_registers == DestinationAndSourceRegisters::new(Register::r0, Register::BPF_PSEUDO_CALL)
	}
	
	/// `BPF_EXIT_INSN()`.
	///
	/// Use the value in register `r0`.
	#[inline(always)]
	pub const fn program_exit() -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | BPF_EXIT,
			Register::r0,
			Register::r0,
			0,
			0,
		)
	}
	
	#[inline(always)]
	pub(crate) fn set_jump_offset(&mut self, offset: i16)
	{
		self.off = offset
	}
	
	#[inline(always)]
	pub(crate) fn set_jump_immediate(&mut self, immediate: i32)
	{
		self.imm = immediate
	}
	
	/// `BPF_LD_IMM64_RAW(DST, SRC, IMM)`.
	#[inline(always)]
	const fn load64_immediate_raw(destination_register: Register, source_register: Register, immediate: u64) -> [Self; 2]
	{
		[
			Self::new
			(
				(BPF_LD as u8) | BPF_DW | (BPF_IMM as u8),
				destination_register,
				source_register,
				0,
				(immediate & 0xFFFF_FFFF) as i32,
			),
			Self::new
			(
				0,
				Register::r0,
				Register::r0,
				0,
				(immediate >> 32) as i32,
			),
		]
	}
	
	/// `BPF_LD_IMM64_RAW_FULL(DST, SRC, OFF1, OFF2, IMM1, IMM2)`.
	#[inline(always)]
	const fn load64_immediate_raw_full(destination_register: Register, source_register: Register, offset1: i16, offset2: i16, immediate1: i32, immediate2: i32) -> [Self; 2]
	{
		[
			Self::new
			(
				(BPF_LD as u8) | BPF_DW | (BPF_IMM as u8),
				destination_register,
				source_register,
				offset1,
				immediate1,
			),
			Self::new
			(
				0,
				Register::r0,
				Register::r0,
				offset2,
				immediate2,
			),
		]
	}
	
	/// `BPF_LD_ABS(SIZE, IMM)`.
	///
	/// "Direct packet access, `r0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	const fn load_r0_direct(size: LoadSize, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_LD as u8) | (size as u8) | (BPF_ABS as u8),
			Register::r0,
			Register::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_LD_IND(SIZE, SRC, IMM)`.
	///
	/// Indirect packet access, `r0 = *(uint *) (skb->data + source_register + immediate)`.
	#[inline(always)]
	const fn load_r0_indirect(size: LoadSize, source_register: Register, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_LD as u8) | size as u8 | (BPF_IND as u8),
			Register::r0,
			source_register,
			0,
			immediate
		)
	}
	
	/// `BPF_LDX_MEM(SIZE, DST, SRC, OFF)`.
	///
	/// "Memory load, `destination_register = *(uint *) (source_register + off16)`".
	#[inline(always)]
	const fn load_memory(size: LoadSize, destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::new
		(
			(BPF_LDX as u8) | (size as u8) | (BPF_MEM as u8),
			destination_register,
			source_register,
			memory_offset,
			0,
		)
	}
	
	/// `BPF_STX_MEM(SIZE, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (destination_register + off16) = source_register`".
	#[inline(always)]
	const fn store_memory(size: LoadSize, destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::new
		(
			(BPF_STX as u8) | (size as u8) | (BPF_MEM as u8),
			destination_register,
			source_register,
			memory_offset,
			0,
		)
	}
	
	/// `BPF_STX_XADD(SIZE, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(destination_register + off16) += source_register`".
	#[inline(always)]
	const fn store_memory_atomic_add(size: LoadSize, destination_register: Register, source_register: Register, memory_offset: i16) -> Self
	{
		Self::new
		(
			(BPF_STX as u8) | (size as u8) | (BPF_XADD as u8),
			destination_register,
			source_register,
			memory_offset,
			0,
		)
	}
	
	/// `BPF_ST_MEM(SIZE, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	const fn store_memory_immediate(size: LoadSize, destination_register: Register, memory_offset: i16, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_ST as u8) | (size as u8) | (BPF_MEM as u8),
			destination_register,
			Register::r0,
			memory_offset,
			immediate,
		)
	}
	
	/// `BPF_RAW_INSN(CODE, DST, SRC, OFF, IMM)`.
	#[inline(always)]
	const fn new
	(
		extended_opcode: u8,
		destination_register: Register,
		source_register: Register,
		offset: i16,
		immediate: i32,
	) -> Self
	{
		Self
		{
			code: extended_opcode,
			destination_and_source_registers: DestinationAndSourceRegisters::new(destination_register, source_register),
			off: offset,
			imm: immediate,
		}
	}
}

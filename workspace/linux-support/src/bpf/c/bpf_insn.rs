// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct bpf_insn
{
	code: u8,
	destination_and_source_registers: DestinationAndSourceExtendedRegisters,
	off: i16,
	imm: i32,
}

impl bpf_insn
{
	/// `BPF_ALU64_REG(OP, DST, SRC)`.
	#[inline(always)]
	pub const fn alu64(operation: AluOperation, destination_register: ExtendedRegister, source_register: ExtendedRegister) -> Self
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
	pub const fn alu32(operation: AluOperation, destination_register: ExtendedRegister, source_register: ExtendedRegister) -> Self
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
	pub const fn alu64_immediate(operation: AluOperation, destination_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::new
		(
			BPF_ALU64 | operation as u8 | (BPF_K as u8),
			destination_register,
			source_register,
			0,
			0,
		)
	}
	
	/// `BPF_ALU32_IMM(OP, DST, IMM)`.
	#[inline(always)]
	pub const fn alu32_immediate(operation: AluOperation, destination_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | operation as u8 | (BPF_K as u8),
			destination_register,
			source_register,
			0,
			0,
		)
	}
	
	/// `BPF_ENDIAN(TYPE, DST, LEN)`.
	///
	/// Endianness conversion.
	pub const fn endian32(endianness: EndiannessOperation, destination_register: ExtendedRegister, length: i32) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | BPF_END | endianness as u8,
			destination_register,
			ExtendedRegister::r0,
			0,
			immmediate,
		)
	}
	
	/// `BPF_MOV64_REG(DST, SRC)`.
	#[inline(always)]
	pub const fn move64(destination_register: ExtendedRegister, source_register: ExtendedRegister) -> Self
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
	pub const fn move32(destination_register: ExtendedRegister, source_register: ExtendedRegister) -> Self
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
	pub const fn move64_immediate(destination_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::new
		(
			BPF_ALU64 | BPF_MOV | (BPF_K as u8),
			destination_register,
			ExtendedRegister::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_MOV32_IMM(DST, IMM)`.
	#[inline(always)]
	pub const fn move32_immediate(destination_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_ALU as u8) | BPF_MOV | (BPF_K as u8),
			destination_register,
			ExtendedRegister::r0,
			0,
			immediate,
		)
	}
	
	// /// `BPF_MOV64_RAW(TYPE, DST, SRC, IMM)`.
	// ///
	// /// Short form of mov based on type, `BPF_X: dst_reg = src_reg  , BPF_K: dst_reg = immediate`.
	// #[inline(always)]
	// pub const fn move64_raw(bpf_k_or_bpf_x, destination_register: ExtendedRegister, source_register: ExtendedRegister, immediate: i32) -> Self
	// {
	// 	Self::new
	// 	(
	// 		BPF_ALU64 | BPF_MOV | BPF_SRC(bpf_k_or_bpf_x),
	// 		destination_register,
	// 		source_register,
	// 		0,
	// 		immediate,
	// 	)
	// }
	//
	// /// `BPF_MOV32_RAW(TYPE, DST, SRC, IMM)`.
	// ///
	// /// Short form of mov based on type, `BPF_X: dst_reg = src_reg  , BPF_K: dst_reg = immediate`.
	// #[inline(always)]
	// pub const fn move32_raw(bpf_k_or_bpf_x, destination_register: ExtendedRegister, source_register: ExtendedRegister, immediate: i32) -> Self
	// {
	// 	Self::new
	// 	(
	// 		(BPF_ALU as u8) | BPF_MOV | BPF_SRC(bpf_k_or_bpf_x),
	// 		destination_register,
	// 		source_register,
	// 		0,
	// 		immediate,
	// 	)
	// }
	
	/// `BPF_LD_IMM64(DST, IMM)`.
	///
	/// Load a 64-bit immediate.
	#[inline(always)]
	pub const fn load64_immediate(destination_register: ExtendedRegister, immediate: u64) -> [Self; 2]
	{
		Self::load64_immediate_raw(destination_register, ExtendedRegister::r0, immediate)
	}
	
	/// `BPF_LD_MAP_FD(DST, MAP_FD)`.
	#[inline(always)]
	pub const fn load_map_file_descriptor_into_proces_local_map_file_descriptor(destination_register: ExtendedRegister, map_file_descriptor: RawFd) -> [Self; 2]
	{
		const BPF_PSEUDO_MAP_FD: ExtendedRegister = ExtendedRegister::r1;
		Self::load64_immediate_raw(destination_register, BPF_PSEUDO_MAP_FD, map_file_descriptor as u64)
	}
	
	/// `BPF_LD_ABS(BPF_B, IMM)`.
	///
	/// "Direct packet access, `R0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load8_absolute(immediate: i32) -> Self
	{
		Self::load_absolute(_8, immediate)
	}
	
	/// `BPF_LD_ABS(BPF_H, IMM)`.
	///
	/// "Direct packet access, `R0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load16_absolute(immediate: i32) -> Self
	{
		Self::load_absolute(_16, immediate)
	}
	
	/// `BPF_LD_ABS(BPF_W, IMM)`.
	///
	/// "Direct packet access, `R0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load32_absolute(immediate: i32) -> Self
	{
		Self::load_absolute(_32, immediate)
	}
	
	/// `BPF_LD_ABS(BPF_DW, IMM)`.
	///
	/// "Direct packet access, `R0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	pub const fn load64_absolute(immediate: i32) -> Self
	{
		Self::load_absolute(_64, immediate)
	}
	
	/// `BPF_LD_IND(BPF_B, SRC, IMM)`.
	///
	/// Indirect packet access, `R0 = *(uint *) (skb->data + src_reg + immediate)`.
	#[inline(always)]
	pub const fn load_indirect8(source_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::load_indirect(_8, source_register, immediate)
	}
	
	/// `BPF_LD_IND(BPF_H, SRC, IMM)`.
	///
	/// Indirect packet access, `R0 = *(uint *) (skb->data + src_reg + immediate)`.
	#[inline(always)]
	pub const fn load_indirect16(source_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::load_indirect(_16, source_register, immediate)
	}
	
	/// `BPF_LD_IND(BPF_W, SRC, IMM)`.
	///
	/// Indirect packet access, `R0 = *(uint *) (skb->data + src_reg + immediate)`.
	#[inline(always)]
	pub const fn load_indirect32(source_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::load_indirect(_32, source_register, immediate)
	}
	
	/// `BPF_LD_IND(BPF_DW, SRC, IMM)`.
	///
	/// Indirect packet access, `R0 = *(uint *) (skb->data + src_reg + immediate)`.
	#[inline(always)]
	pub const fn load_indirect64(source_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::load_indirect(_64, source_register, immediate)
	}
	
	/// `BPF_LDX_MEM(BPF_B, DST, SRC, OFF)`.
	///
	/// "Memory load, `dst_reg = *(uint *) (src_reg + off16)`".
	#[inline(always)]
	pub const fn load8_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::load_memory(_8, destination_register, source_register, offset)
	}
	
	/// `BPF_LDX_MEM(BPF_H, DST, SRC, OFF)`.
	///
	/// "Memory load, `dst_reg = *(uint *) (src_reg + off16)`".
	#[inline(always)]
	pub const fn load16_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::load_memory(_16, destination_register, source_register, offset)
	}
	
	/// `BPF_LDX_MEM(BPF_W, DST, SRC, OFF)`.
	///
	/// "Memory load, `dst_reg = *(uint *) (src_reg + off16)`".
	#[inline(always)]
	pub const fn load32_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::load_memory(_32, destination_register, source_register, offset)
	}
	
	/// `BPF_LDX_MEM(BPF_DW, DST, SRC, OFF)`.
	///
	/// "Memory load, `dst_reg = *(uint *) (src_reg + off16)`".
	#[inline(always)]
	pub const fn load64_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::load_memory(_64, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_MEM(BPF_B, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (dst_reg + off16) = src_reg`".
	#[inline(always)]
	pub const fn store8_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory(_8, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_MEM(BPF_H, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (dst_reg + off16) = src_reg`".
	#[inline(always)]
	pub const fn store16_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory(_16, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_MEM(BPF_W, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (dst_reg + off16) = src_reg`".
	#[inline(always)]
	pub const fn store32_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory(_32, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_MEM(BPF_DW, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (dst_reg + off16) = src_reg`".
	#[inline(always)]
	pub const fn store64_memory(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory(_64, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_XADD(BPF_B, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(dst_reg + off16) += src_reg`".
	#[inline(always)]
	pub const fn store8_memory_atomic_add(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_8, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_XADD(BPF_H, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(dst_reg + off16) += src_reg`".
	#[inline(always)]
	pub const fn store16_memory_atomic_add(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_16, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_XADD(BPF_W, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(dst_reg + off16) += src_reg`".
	#[inline(always)]
	pub const fn store32_memory_atomic_add(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_32, destination_register, source_register, offset)
	}
	
	/// `BPF_STX_XADD(BPF_DW, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(dst_reg + off16) += src_reg`".
	#[inline(always)]
	pub const fn store64_memory_atomic_add(destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::store_memory_atomic_add(_64, destination_register, source_register, offset)
	}
	
	/// `BPF_ST_MEM(BPF_B, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store8_memory_immediate(destination_register: ExtendedRegister, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_8, destination_register, offset, immediate)
	}
	
	/// `BPF_ST_MEM(BPF_H, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store16_memory_immediate(destination_register: ExtendedRegister, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_16, destination_register, offset, immediate)
	}
	
	/// `BPF_ST_MEM(BPF_W, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store32_memory_immediate(destination_register: ExtendedRegister, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_32, destination_register, offset, immediate)
	}
	
	/// `BPF_ST_MEM(BPF_DW, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	pub const fn store64_memory_immediate(destination_register: ExtendedRegister, offset: i16, immediate: i32) -> Self
	{
		Self::store_memory_immediate(_64, destination_register, offset, immediate)
	}
	
	/// `BPF_JMP_REG(OP, DST, SRC, OFF)`.
	///
	/// "Conditional jumps against registers, `if (dst_reg 'op' src_reg) goto pc + offset`".
	///
	/// Uses a 64-bit comparision.
	#[inline(always)]
	pub const fn conditional_jump64(jump_operation: JumpOperation, destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | jump_operation as u8 | (BPF_X as u8),
			destination_register,
			source_register,
			offset,
			0,
		)
	}
	
	/// `BPF_JMP32_REG(OP, DST, SRC, OFF)`.
	///
	/// "Conditional jumps against registers, `if (dst_reg 'op' src_reg) goto pc + offset`".
	///
	/// Uses a 32-bit comparision of the lower 64-bits of registers.
	#[inline(always)]
	pub const fn conditional_jump32(jump_operation: JumpOperation, destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::new
		(
			BPF_JMP32 | jump_operation as u8 | (BPF_X as u8),
			destination_register,
			source_register,
			offset,
			0,
		)
	}
	
	/// `BPF_JMP_IMM(OP, DST, IMM, OFF)`.
	///
	/// "Conditional jumps against immediates, `if (dst_reg 'op' immediate) goto pc + offset`".
	///
	/// Uses a 64-bit comparision.
	#[inline(always)]
	pub const fn conditional_jump64_immediate(jump_operation: JumpOperation, destination_register: ExtendedRegister, immediate: i32, offset: i16) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | jump_operation as u8 | (BPF_K as u8),
			destination_register,
			ExtendedRegister::r0,
			offset,
			immediate,
		)
	}
	
	/// `BPF_JMP32_IMM(OP, DST, SRC, OFF)`.
	///
	/// "Conditional jumps against immediates, `if (dst_reg 'op' immediate) goto pc + offset`".
	///
	/// Uses a 32-bit comparision of the lower 64-bits of registers.
	#[inline(always)]
	pub const fn conditional_jump32_immediate(jump_operation: JumpOperation, destination_register: ExtendedRegister, immediate: i32, offset: i16) -> Self
	{
		Self::new
		(
			BPF_JMP32 | jump_operation as u8 | (BPF_K as u8),
			destination_register,
			ExtendedRegister::r0,
			offset,
			immediate,
		)
	}
	
	/// `BPF_JMP_A(OFF)`.
	///
	/// "Unconditional jumps, `goto pc + offset`".
	#[inline(always)]
	pub const fn unconditional_jump(offset: i16) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | (BMP_JA as u8),
			ExtendedRegister::r0,
			ExtendedRegister::r0,
			offset,
			0,
		)
	}
	
	/// `BPF_EMIT_CALL(FUNC)`.
	///
	/// Function call.
	#[inline(always)]
	pub const fn function_call(function: bpf_func_id) -> Self
	{
		Self::new
		(
			(BPF_JMP as u8) | BPF_CALL,
			ExtendedRegister::r0,
			ExtendedRegister::r0,
			0,
			(bpf_func_id as i32) - (func_id::BPF_FUNC_unspec as i32),
		)
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
			ExtendedRegister::r0,
			ExtendedRegister::r0,
			0,
			0,
		)
	}
	
	/// `BPF_LD_IMM64_RAW(DST, SRC, IMM)`.
	#[inline(always)]
	const fn load64_immediate_raw(destination_register: ExtendedRegister, source_register: ExtendedRegister, immediate: u64) -> [Self; 2]
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
				ExtendedRegister::r0,
				ExtendedRegister::r0,
				0,
				(immediate >> 32) as i32,
			),
		]
	}
	
	/// `BPF_LD_ABS(SIZE, IMM)`.
	///
	/// "Direct packet access, `R0 = *(uint *) (skb->data + immediate)`".
	#[inline(always)]
	const fn load_absolute(size: ExtendedLoadSize, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_LD as u8) | (size as u8) | (BPF_ABS as u8),
			ExtendedRegister::r0,
			ExtendedRegister::r0,
			0,
			immediate,
		)
	}
	
	/// `BPF_LD_IND(SIZE, SRC, IMM)`.
	///
	/// Indirect packet access, `R0 = *(uint *) (skb->data + src_reg + immediate)`.
	#[inline(always)]
	const fn load_indirect(size: ExtendedLoadSize, source_register: ExtendedRegister, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_LD as u8) | size as u8 | (BPF_IND as u8),
			ExtendedRegister::r0,
			source_register,
			0,
			immediate
		)
	}
	
	/// `BPF_LDX_MEM(SIZE, DST, SRC, OFF)`.
	///
	/// "Memory load, `dst_reg = *(uint *) (src_reg + off16)`".
	#[inline(always)]
	const fn load_memory(size: ExtendedLoadSize, destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::new
		(
			(BPF_LDX as u8) | (size as u8) | (BPF_MEM as u8),
			destination_register,
			source_register,
			offset,
			0,
		)
	}
	
	/// `BPF_STX_MEM(SIZE, DST, SRC, OFF)`.
	///
	/// "Memory store, `*(uint *) (dst_reg + off16) = src_reg`".
	#[inline(always)]
	const fn store_memory(size: ExtendedLoadSize, destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::new
		(
			(BPF_STX as u8) | (size as u8) | (BPF_MEM as u8),
			destination_register,
			source_register,
			offset,
			0,
		)
	}
	
	/// `BPF_STX_XADD(SIZE, DST, SRC, OFF)`.
	///
	/// "Atomic memory add, `*(uint *)(dst_reg + off16) += src_reg`".
	#[inline(always)]
	const fn store_memory_atomic_add(size: ExtendedLoadSize, destination_register: ExtendedRegister, source_register: ExtendedRegister, offset: i16) -> Self
	{
		Self::new
		(
			(BPF_STX as u8) | (size as u8) | (BPF_XADD as u8),
			destination_register,
			source_register,
			offset,
			0,
		)
	}
	
	/// `BPF_ST_MEM(SIZE, DST, OFF, IMM)`.
	///
	/// "Memory store, `*(uint *) (destination_register + offset) = immediate`".
	#[inline(always)]
	const fn store_memory_immediate(size: ExtendedLoadSize, destination_register: ExtendedRegister, offset: i16, immediate: i32) -> Self
	{
		Self::new
		(
			(BPF_ST as u8) | (size as u8) | (BPF_MEM as u8),
			destination_register,
			ExtendedRegister::r0,
			offset,
			immediate,
		)
	}
	
	/// `BPF_RAW_INSN(CODE, DST, SRC, OFF, IMM)`.
	#[inline(always)]
	const fn new
	(
		extended_opcode: u8,
		destination_register: ExtendedRegister,
		source_register: ExtendedRegister,
		offset: i16,
		immediate: i32,
	) -> Self
	{
		Self
		{
			code: extended_opcode,
			destination_and_source_registers: DestinationAndSourceExtendedRegisters::new(destination_register, source_register),
			off: offset,
			imm: immediate,
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An eBPF program.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BpfProgram(Vec<sock_filter>);

impl Deref for BpfProgram
{
	type Target = Vec<sock_filter>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for BpfProgram
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

#[allow(missing_docs)]
impl BpfProgram
{
	/// Useful if one needs to make a jump longer than 256 instructions; simply jump to this instruction, then jump the arbitrary value needed.
	#[inline(always)]
	pub fn jump_always(&mut self, number_of_statements_to_jump: u32)
	{
		self.BPF_STMT(BPF_JMP + BPF_JA, number_of_statements_to_jump)
	}

	/// `pc += (Accumulator == compare_to) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_equal_to_constant(&mut self, compare_to: u32, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, compare_to, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator > compare_to) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_greater_than_constant(&mut self, compare_to: u32, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JGT + BPF_K, compare_to, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator >= compare_to) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_greater_than_or_equal_to_constant(&mut self, compare_to: u32, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JGE + BPF_K, compare_to, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator & IndexRegister) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_bits_set_constant(&mut self, compare_to: u32, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JSET + BPF_K, compare_to, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator == IndexRegister) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_equal_to_index_register(&mut self, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_X, 0, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator > IndexRegister) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_greater_than_index_register(&mut self, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JGT + BPF_X, 0, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator >= IndexRegister) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_greater_than_or_equal_to_index_register(&mut self, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JGE + BPF_X, 0, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `pc += (Accumulator & IndexRegister) ? number_of_statements_to_jump_if_true : number_of_statements_to_jump_if_false`.
	#[inline(always)]
	pub fn jump_if_bits_set_index_register(&mut self, number_of_statements_to_jump_if_true: u8, number_of_statements_to_jump_if_false: u8)
	{
		self.BPF_JUMP(BPF_JMP + BPF_JSET + BPF_X, 0, number_of_statements_to_jump_if_true, number_of_statements_to_jump_if_false)
	}

	/// `Accumulator <- -Accumulator`.
	#[inline(always)]
	pub fn negate_accumulator(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_NEG, 0)
	}

	/// `Accumulator <- Accumulator >> IndexRegister`.
	#[inline(always)]
	pub fn accumulator_right_shift_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_RSH + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator << IndexRegister`.
	#[inline(always)]
	pub fn accumulator_left_shift_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_LSH + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator ⊕ IndexRegister`.
	#[inline(always)]
	pub fn accumulator_xor_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_XOR + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator ∨ IndexRegister`.
	#[inline(always)]
	pub fn accumulator_or_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_OR + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator ∧ IndexRegister`.
	#[inline(always)]
	pub fn accumulator_and_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_AND + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator mod IndexRegister`.
	#[inline(always)]
	pub fn accumulator_modulo_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_MOD + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator ÷ IndexRegister`.
	#[inline(always)]
	pub fn accumulator_divide_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_DIV + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator × IndexRegister`.
	#[inline(always)]
	pub fn accumulator_multiply_with_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_MUL + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator - IndexRegister`.
	#[inline(always)]
	pub fn accumulator_subtract_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_SUB + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator + IndexRegister`.
	#[inline(always)]
	pub fn accumulator_add_index_register(&mut self)
	{
		self.BPF_STMT(BPF_ALU + BPF_ADD + BPF_X, 0)
	}

	/// `Accumulator <- Accumulator >> constant`.
	#[inline(always)]
	pub fn accumulator_right_shift_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_RSH + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator << constant`.
	#[inline(always)]
	pub fn accumulator_left_shift_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_LSH + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator ⊕ constant`.
	#[inline(always)]
	pub fn accumulator_xor_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_XOR + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator ∨ constant`.
	#[inline(always)]
	pub fn accumulator_or_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_OR + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator ∧ constant`.
	#[inline(always)]
	pub fn accumulator_and_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_AND + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator mod constant`.
	#[inline(always)]
	pub fn accumulator_modulo_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_DIV + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator ÷ constant`.
	#[inline(always)]
	pub fn accumulator_divide_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_DIV + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator × constant`.
	#[inline(always)]
	pub fn accumulator_multiply_with_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_MUL + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator - constant`.
	#[inline(always)]
	pub fn accumulator_subtract_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_SUB + BPF_K, constant)
	}

	/// `Accumulator <- Accumulator + constant`.
	#[inline(always)]
	pub fn accumulator_add_constant(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_ALU + BPF_ADD + BPF_K, constant)
	}

	/// `ScratchMemory[scratch_memory_index] <- Accumulator`.
	#[inline(always)]
	pub fn store_accumulator_in_scratch_memory(&mut self, scratch_memory_index: ScratchMemoryIndex)
	{
		self.BPF_STMT(BPF_ST, scratch_memory_index.into())
	}

	/// `ScratchMemory[scratch_memory_index] <- IndexRegister`.
	#[inline(always)]
	pub fn store_index_register_in_scratch_memory(&mut self, scratch_memory_index: ScratchMemoryIndex)
	{
		self.BPF_STMT(BPF_STX, scratch_memory_index.into())
	}

	/// `Accumulator <- IndexRegister`.
	#[inline(always)]
	pub fn copy_index_register_to_accumulator(&mut self)
	{
		self.BPF_STMT(BPF_MISC + BPF_TXA, 0)
	}

	/// `IndexRegister <- Accumulator`.
	#[inline(always)]
	pub fn copy_accumulator_to_index_register(&mut self)
	{
		self.BPF_STMT(BPF_MISC + BPF_TAX, 0)
	}

	/// `IndexRegister <- constant`.
	#[inline(always)]
	pub fn load_index_register_with_constant_u32(&mut self, constant: u32)
	{
		self.BPF_STMT(BPF_LDX + BPF_W + BPF_IMM, constant);
	}

	#[inline(always)]
	pub fn load_index_register_from_scratch_memory_u32(&mut self, scratch_memory_index: ScratchMemoryIndex)
	{
		self.BPF_STMT(BPF_LDX + BPF_W + BPF_MEM, scratch_memory_index.into());
	}

	/// `IndexRegister <- length`.
	///
	/// For seccomp programs, this is the actual size of `seccomp_data` as returned in `seccomp_notif_sizes.seccomp_data as u32`.
	#[inline(always)]
	pub fn load_index_register_with_length_u32(&mut self)
	{
		self.BPF_STMT(BPF_LDX + BPF_W + BPF_LEN, 0);
	}

	/// `4*(P[k:1]&0xf)`
	#[inline(always)]
	pub fn load_index_register_with_ip_header_length(&mut self, k: u32)
	{
		self.BPF_STMT(BPF_LDX + BPF_B + BPF_MSH, k);
	}

	#[inline(always)]
	pub fn load_accumulator_with_fixed_offset_struct_field(&mut self, fixed_offset: usize)
	{
		self.load_accumulator_with_fixed_offset_u32(fixed_offset as u32)
	}

	#[inline(always)]
	pub fn load_accumulator_with_fixed_offset_u32(&mut self, fixed_offset: u32)
	{
		self.load_accumulator_with_fixed_offset(fixed_offset, BPF_W)
	}

	/// Not supported for seccomp programs.
	#[inline(always)]
	pub fn load_accumulator_with_fixed_offset_u16(&mut self, fixed_offset: u16)
	{
		self.load_accumulator_with_fixed_offset(fixed_offset as u32, BPF_H)
	}

	/// Not supported for seccomp programs.
	#[inline(always)]
	pub fn load_accumulator_with_fixed_offset_u8(&mut self, fixed_offset: u8)
	{
		self.load_accumulator_with_fixed_offset(fixed_offset as u32, BPF_B)
	}

	#[inline(always)]
	fn load_accumulator_with_fixed_offset(&mut self, fixed_offset: u32, size_flag: u16)
	{
		self.load_accumulator_with_(size_flag, BPF_ABS, fixed_offset)
	}

	/// Base of the offset is in the index register.
	#[inline(always)]
	pub fn load_accumulator_with_variable_offset_u32(&mut self, variable_offset: u32)
	{
		self.load_accumulator_with_variable_offset(variable_offset, BPF_W)
	}

	/// Base of the offset is in the index register.
	///
	/// Not supported for seccomp programs.
	#[inline(always)]
	pub fn load_accumulator_with_variable_offset_u16(&mut self, variable_offset: u16)
	{
		self.load_accumulator_with_variable_offset(variable_offset as u32, BPF_H)
	}

	/// Base of the offset is in the index register.
	///
	/// Not supported for seccomp programs.
	#[inline(always)]
	pub fn load_accumulator_with_variable_offset_u8(&mut self, variable_offset: u8)
	{
		self.load_accumulator_with_variable_offset(variable_offset as u32, BPF_B)
	}

	#[inline(always)]
	fn load_accumulator_with_variable_offset(&mut self, variable_offset: u32, size_flag: u16)
	{
		self.load_accumulator_with_(size_flag, BPF_IND, variable_offset)
	}

	#[inline(always)]
	pub fn load_accumulator_with_constant_u32(&mut self, constant: u32)
	{
		self.load_accumulator_with_(BPF_W, BPF_IMM, constant)
	}

	/// For seccomp programs, this is the actual size of `seccomp_data` as returned in `seccomp_notif_sizes.seccomp_data as u32`.
	#[inline(always)]
	pub fn load_accumulator_with_length_u32(&mut self)
	{
		self.load_accumulator_with_(0, BPF_LEN, 0)
	}

	#[inline(always)]
	pub fn load_accumulator_from_scratch_memory_u32(&mut self, scratch_memory_index: u32)
	{
		self.load_accumulator_with_(0, BPF_MEM, scratch_memory_index)
	}

	#[inline(always)]
	fn load_accumulator_with_(&mut self, size_flag: u16, addressing_mode: u16, value: u32)
	{
		self.BPF_STMT(BPF_LD + size_flag + addressing_mode, value)
	}

	#[inline(always)]
	pub fn return_constant(&mut self, top_bits: u32, bottom_bits_used_for_error_number_if_appropriate: u16)
	{
		self.BPF_STMT(BPF_RET + BPF_K, top_bits | (bottom_bits_used_for_error_number_if_appropriate as u32))
	}

	#[inline(always)]
	pub fn return_accumulator(&mut self)
	{
		self.BPF_STMT(BPF_RET + BPF_A, 0)
	}

	#[inline(always)]
	fn BPF_STMT(&mut self, code: u16, k: u32)
	{
		let line = BPF_STMT(code, k);
		self.line(line)
	}

	#[inline(always)]
	fn BPF_JUMP(&mut self, code: u16, k: u32, jt: u8, jf: u8)
	{
		let line = BPF_JUMP(code, k, jt, jf);
		self.line(line)
	}

	#[inline(always)]
	fn line(&mut self, line: sock_filter)
	{
		self.0.push(line)
	}
}

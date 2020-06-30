// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Instruction class.
/// 
/// jmp mode in word width.
pub(crate) const BPF_JMP32: u8 = 0x06;

/// Instruction class.
/// 
/// alu mode in double word width.
pub(crate) const BPF_ALU64: u8 = 0x07;

/// ld/ldx fields.
///
/// double word (64-bit).
///
/// Extends the range of classic BPF sizes `BPF_W`, `BPF_H` and `BPF_B`.
pub(crate) const BPF_DW: u8 = 0x18;

/// exclusive add.
pub(crate) const BPF_XADD: u8 = 0xC0;

/// alu/jmp fields.
///
/// mov reg to reg.
pub(crate) const BPF_MOV: u8 = 0xB0;

/// alu/jmp fields.
///
/// sign extending arithmetic shift right.
pub(crate) const BPF_ARSH: u8 = 0xC0;

/// change endianness of a register.
///
/// flags for endianness conversion.
pub(crate) const BPF_END: u8 = 0xD0;

/// change endianness of a register.
///
/// convert to little-endian.
pub(crate) const BPF_TO_LE: u8 = 0x00;

/// change endianness of a register.
///
/// convert to big-endian.
pub(crate) const BPF_TO_BE: u8 = 0x08;

/// change endianness of a register.
pub(crate) const BPF_FROM_LE: u8 = BPF_TO_LE;

/// change endianness of a register.
pub(crate) const BPF_FROM_BE: u8 = BPF_TO_BE;

/// jmp encodings.
///
/// jump !=.
pub(crate) const BPF_JNE: u8 = 0x50;

/// jmp encodings.
///
/// LT is unsigned, '<'.
pub(crate) const BPF_JLT: u8 = 0xA0;

/// jmp encodings.
///
/// LE is unsigned, '<='.
pub(crate) const BPF_JLE: u8 = 0xB0;

/// jmp encodings.
///
/// SGT is signed '>', GT in x86.
pub(crate) const BPF_JSGT: u8 = 0x60;

/// jmp encodings.
///
/// SGE is signed '>=', GE in x86.
pub(crate) const BPF_JSGE: u8 = 0x70;

/// jmp encodings.
///
/// SLT is signed, '<'.
pub(crate) const BPF_JSLT: u8 = 0xC0;

/// jmp encodings.
///
/// SLE is signed, '<='.
pub(crate) const BPF_JSLE: u8 = 0xD0;

/// jmp encodings.
///
/// function call.
pub(crate) const BPF_CALL: u8 = 0x80;

/// jmp encodings.
///
/// function return.
pub(crate) const BPF_EXIT: u8 = 0x90;


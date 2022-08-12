// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Sourced from `linux/bpf_common.h` and `linux/filter.h`.

/// Instruction class.
#[allow(dead_code)]
#[inline(always)]
pub(crate) const fn BPF_CLASS(code: u16) -> u16
{
	code & 0x07
}

/// Instruction class.
pub(crate) const BPF_LD: u16 = 0x00;

/// Instruction class.
pub(crate) const BPF_LDX: u16 = 0x01;

/// Instruction class.
pub(crate) const BPF_ST: u16 = 0x02;

/// Instruction class.
pub(crate) const BPF_STX: u16 = 0x03;

/// Instruction class.
pub(crate) const BPF_ALU: u16 = 0x04;

/// Instruction class.
pub(crate) const BPF_JMP: u16 = 0x05;

/// Instruction class.
pub(crate) const BPF_RET: u16 = 0x06;

/// Instruction class.
pub(crate) const BPF_MISC: u16 = 0x07;

/// ld/ldx field (size).
#[inline(always)]
pub(crate)const fn BPF_SIZE(code: u16) -> u16
{
	code & 0x18
}

/// ld/ldx field (size).
/// 
/// 32-bit.
pub(crate) const BPF_W: u16 = 0x00;

/// ld/ldx field (size).
///
/// 16-bit.
pub(crate) const BPF_H: u16 = 0x08;

/// ld/ldx field (size).
///
/// 8-bit.
pub(crate) const BPF_B: u16 = 0x10;

/// ld/ldx field (addressing mode).
#[allow(dead_code)]
#[inline(always)]
pub(crate)const fn BPF_MODE(code: u16) -> u16
{
	code & 0xE0
}

/// ld/ldx field (addressing mode).
///
/// Addressing Mode: Constant.
pub(crate) const BPF_IMM: u16 = 0x00;

/// ld/ldx field (addressing mode).
///
/// Addressing Mode: Struct (Packet) Data at a Fixed Offset.
pub(crate) const BPF_ABS: u16 = 0x20;

/// ld/ldx field (addressing mode).
///
/// Addressing Mode: Struct (Packet) Data at a Variable Offset.
pub(crate) const BPF_IND: u16 = 0x40;

/// ld/ldx field (addressing mode).
///
/// Addressing Mode: Word in the Scratch Memory store.
pub(crate) const BPF_MEM: u16 = 0x60;

/// ld/ldx field (addressing mode).
///
/// Addressing Mode: Struct (Packet) Length.
pub(crate) const BPF_LEN: u16 = 0x80;

/// ld/ldx field (addressing mode).
///
/// Addressing Mode: Hack for efficiently loading the IP header length.
pub(crate) const BPF_MSH: u16 = 0xA0;

/// alu/jmp field (op).
#[allow(dead_code)]
#[inline(always)]
pub(crate) const fn BPF_OP(code: u16) -> u16
{
	code & 0xF0
}

/// alu/jmp field (op).
pub(crate) const BPF_ADD: u16 = 0x00;

/// alu/jmp field (op).
pub(crate) const BPF_SUB: u16 = 0x10;

/// alu/jmp field (op).
pub(crate) const BPF_MUL: u16 = 0x20;

/// alu/jmp field (op).
pub(crate) const BPF_DIV: u16 = 0x30;

/// alu/jmp field (op).
pub(crate) const BPF_OR: u16 = 0x40;

/// alu/jmp field (op).
pub(crate) const BPF_AND: u16 = 0x50;

/// alu/jmp field (op).
pub(crate) const BPF_LSH: u16 = 0x60;

/// alu/jmp field (op).
pub(crate) const BPF_RSH: u16 = 0x70;

/// alu/jmp field (op).
pub(crate) const BPF_NEG: u16 = 0x80;

/// alu/jmp field (op).
pub(crate) const BPF_MOD: u16 = 0x90;

/// alu/jmp field (op).
pub(crate) const BPF_XOR: u16 = 0xA0;

/// alu/jmp field (op).
pub(crate) const BPF_JA: u16 = 0x00;

/// alu/jmp field (op).
pub(crate) const BPF_JEQ: u16 = 0x10;

/// alu/jmp field (op).
pub(crate) const BPF_JGT: u16 = 0x20;

/// alu/jmp field (op).
pub(crate) const BPF_JGE: u16 = 0x30;

/// alu/jmp field (op).
pub(crate) const BPF_JSET: u16 = 0x40;

/// alu/jmp field source.
#[inline(always)]
pub(crate)const fn BPF_SRC(code: u16) -> u16
{
	code & 0x08
}

/// alu/jmp field source or return value.
///
/// Source is constant.
pub(crate) const BPF_K: u16 = 0x00;

/// alu/jmp field source only.
///
/// Source is IndexRegister (`X`).
pub(crate) const BPF_X: u16 = 0x08;

/// return value only.
#[allow(dead_code)]
#[inline(always)]
pub(crate) const fn BPF_RVAL(code: u16) -> u16
{
	code & 0x18
}

/// alu/jmp field return value only.
///
/// Source is Accumulator (`X`).
pub(crate) const BPF_A: u16 = 0x10;

/// Miscellaneous operation.
#[allow(dead_code)]
#[inline(always)]
pub(crate) const fn BPF_MISCOP(code: u16) -> u16
{
	code & 0xF8
}

/// Miscellaneous operation.
pub(crate) const BPF_TAX: u16 = 0x00;

/// Miscellaneous operation.
pub(crate) const BPF_TXA: u16 = 0x80;

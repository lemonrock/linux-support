// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;

// See https://github.com/Xilinx-CNS/ebpf_asm

pub struct Immediate()





pub enum Instruction
{
	// Register-to-register: 32-bit or 64-bit
	// immediate-to-register: 32-bit or 64-bit
		// immediate-to-register 32-bit:-
			// immediate is either a map name or an UNSIGNED 64-bit integer.
	
	Load
	{
		size: Size,
		destination: RegisterOrOther,
		source: RegisterOrOther,
	},
}

pub enum RegisterOrOther
{
	Register,
	
	// Must fit in a signed 32-bit integer apart from `ld reg.q, imm` (ie load a 64-bit immediate into `reg`).
	LiteralValue,
	// Must fit in a signed 32-bit integer apart from `ld reg.q, imm` (ie load a 64-bit immediate into `reg`).
	LiterateEquateName,
}

pub enum Register
{
	r0,
	r1,
	r2,
	r3,
	r4,
	r5,
	r6,
	r7,
	r8,
	r9,
	
	/// Also known as `fp`.
	r10,
}

impl Register
{
	/// An alias to `r10`.
	pub const fp: Self = Register::r10;
}



pub enum Size
{
	/// Byte.
	EightBit,
	
	/// Word.
	SixteenBit,
	
	/// Long.
	ThirtyTwoBit,
	
	/// Quad.
	SixtyFourBit,
}

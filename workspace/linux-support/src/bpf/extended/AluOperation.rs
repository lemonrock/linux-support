// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Operation.
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum AluOperation
{
	/// Add.
	Add = BPF_OP(BPF_ADD) as u8,
	
	/// Subtract.
	Subtract = BPF_OP(BPF_SUB) as u8,
	
	/// Multiply.
	Multiply = BPF_OP(BPF_MUL) as u8,
	
	/// Divide.
	Divide = BPF_OP(BPF_DIV) as u8,
	
	/// Or.
	Or = BPF_OP(BPF_OR) as u8,
	
	/// And.
	And = BPF_OP(BPF_AND) as u8,
	
	/// Left-shift.
	LeftShift = BPF_OP(BPF_LSH) as u8,
	
	/// Right-shift.
	RightShift = BPF_OP(BPF_RSH) as u8,
	
	/// Negate.
	Negate = BPF_OP(BPF_NEG) as u8,
	
	/// Modulus.
	Modulus = BPF_OP(BPF_MOD) as u8,
	
	/// Exclusive or.
	ExclusiveOr = BPF_OP(BPF_XOR) as u8,
	
	/// Sign-extending arithmetic right-shift.
	SignExtendingArithmeticRightShift = BPF_OP(BPF_ARSH as u16) as u8,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Operation.
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum JumpOperation
{
	/// Not equal (`!=`).
	NotEqual = BPF_OP(BPF_JNE as u16) as u8,
	
	/// Less-than unsigned (`<`).
	LessThanUnsigned = BPF_OP(BPF_JLT as u16) as u8,
	
	/// Less-than-or-equal-to unsigned (`<=`).
	LessThanOrEqualToUnsigned = BPF_OP(BPF_JLE as u16) as u8,
	
	/// Greater-than signed (`>`).
	GreaterThanSigned = BPF_OP(BPF_JSGT as u16) as u8,
	
	/// Greater-than-or-equal-to signed (`>=`).
	GreaterThanOrEqualToSigned = BPF_OP(BPF_JSGE as u16) as u8,
	
	/// Less-than signed (`<`).
	LessThanSigned = BPF_OP(BPF_JSLT as u16) as u8,
	
	/// Less-than-or-equal-to signed (`<=`).
	LessThanOrEqualToSigned = BPF_OP(BPF_JSLE as u16) as u8,
}

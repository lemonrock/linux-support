// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The first instruction in a BPF relative function must have a `bpf_line_info`.
///
/// See the linux kernel function `check_btf_line()` in `linux/bpf/verifier.c`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct bpf_line_info
{
	/// Valid values are `[0, bpf_attr.program_load.insn_cnt - 1]`.
	///
	/// Essentially an absolute program counter.
	pub(crate) insn_off: u32,
	
	/// Offset to string table for the filename.
	pub(crate) file_name_off: u32,
	
	/// Offset to string table for the source line.
	pub(crate) line_off: u32,
	
	/// Line number and column number.
	pub(crate) line_col: u32,
}

impl bpf_line_info
{
	/// 4194303.
	pub(crate) const InclusiveMaximumLineNumber: u32 = (2 ^ 22) - 1;
	
	/// 1023.
	pub(crate) const InclusiveMaximumColumnNumber: u32 = 0x3FF;
	
	pub(crate) const MaximumNumberOfProgramLines: usize = (Self::InclusiveMaximumLineNumber as usize) + 1;
	
	#[allow(dead_code)]
	#[inline(always)]
	const fn BPF_LINE_INFO_LINE_NUM(line_col: u32) -> u32
	{
		line_col >> 10
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	const fn BPF_LINE_INFO_LINE_COL(line_col: u32) -> u32
	{
		line_col & Self::InclusiveMaximumColumnNumber
	}
}

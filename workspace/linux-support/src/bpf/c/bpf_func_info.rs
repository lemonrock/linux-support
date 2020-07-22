// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See the linux kernel function `check_btf_func()` in `linux/bpf/verifier.c`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct bpf_func_info
{
	/// Valid values are `[0, bpf_attr.program_load.insn_cnt - 1]`.
	///
	/// In an array of `bpf_func_info`, the first element (at index `0`) should have `insn_off = 0`.
	///
	/// Subsequent values must increase and be at function boundaries.
	///
	/// Essentially an absolute program counter.
	pub(crate) insn_off: u32,
	
	/// Relative pointer to a `struct btf_type` of type `BTF_KIND_FUNC`.
	///
	/// Increments from `1`; a value of `0` is always the type `void`.
	pub(crate) type_identifier: BpfTypeFormatTypeIdentifier,
}

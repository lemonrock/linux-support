// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum bpf_stack_build_id_status
{
	/// user space need an empty entry to identify end of a trace.
	BPF_STACK_BUILD_ID_EMPTY = 0,
	
	/// with valid `build_id` and `offset`.
	BPF_STACK_BUILD_ID_VALID = 1,
	
	/// couldn't get `build_id`, fallback to instruction pointer (IP).
	BPF_STACK_BUILD_ID_IP = 2,
}

impl Default for bpf_stack_build_id_status
{
	#[inline(always)]
	fn default() -> Self
	{
		bpf_stack_build_id_status::BPF_STACK_BUILD_ID_EMPTY
	}
}

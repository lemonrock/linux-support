// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A stack frame.
#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct bpf_stack_build_id
{
	/// How to interpret `offset_or_instruction_pointer`:-
	///
	/// * `BPF_STACK_BUILD_ID_EMPTY`: `offset_or_instruction_pointer` is unused.
	/// * `BPF_STACK_BUILD_ID_VALID`: `offset_or_instruction_pointer.offset` is valid.
	/// * `BPF_STACK_BUILD_ID_IP`: `offset_or_instruction_pointer.ip` (instruction pointer) is valid.
	status: bpf_stack_build_id_status,
	
	pub(crate) build_id: [c_uchar; Self::BPF_BUILD_ID_SIZE],
	
	offset_or_instruction_pointer: OffsetOrInstructionPointer,
}

impl Debug for bpf_stack_build_id
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "bpf_stack_build_id {{ status: {:?}, build_id: {:?}, offset_or_instruction_pointer: {:?} }}", self.status, self.build_id, self.offset_or_instruction_pointer)
	}
}

impl bpf_stack_build_id
{
	pub(crate) const BPF_BUILD_ID_SIZE: usize = 20;
	
	/// Left is offset; right is instruction pointer.
	#[allow(dead_code)]
	#[inline(always)]
	pub fn offset_or_instruction_pointer(&self) -> Option<Either<u64, InstructionPointer>>
	{
		use self::bpf_stack_build_id_status::*;
		
		match self.status
		{
			BPF_STACK_BUILD_ID_EMPTY => None,
			BPF_STACK_BUILD_ID_VALID => Some(Left(unsafe { self.offset_or_instruction_pointer.offset })),
			BPF_STACK_BUILD_ID_IP => Some(Right(unsafe { self.offset_or_instruction_pointer.ip })),
		}
	}
}

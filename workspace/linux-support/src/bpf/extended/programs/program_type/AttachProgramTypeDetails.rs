// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program type details.
///
/// Only permitted for the program types:-
///
/// * `BPF_PROG_TYPE_TRACING`.
/// * `BPF_PROG_TYPE_EXT`.
///
/// See the function `bpf_prog_load_check_attach()` in the linux source `kernel/bpf/syscall.c`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AttachProgramTypeDetails
{
	/// In-kernel BTF type id to attach to.
	pub attach_bpf_type_identifier: AttachBpfTypeIdentifier,
	
	/// A file descriptor; use `0` to attach to vmlinux.
	pub attach_prog_fd: RawFd,
}

impl AttachProgramTypeDetails
{
	#[inline(always)]
	pub(crate) fn to_values(&self, program_type: bpf_prog_type, expected_attached_type: bpf_attach_type) -> (bpf_prog_type, bpf_attach_type, u32, u32, u32, Option<NetworkInterfaceIndex>)
	{
		(program_type, expected_attached_type, self.attach_bpf_type_identifier.0, self.attach_prog_fd as u32, 0, None)
	}
}

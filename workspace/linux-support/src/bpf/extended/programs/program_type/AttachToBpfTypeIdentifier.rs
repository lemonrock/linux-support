// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// In-kernel BTF type id to attach to.
///
/// This is called in the Linux kernel code `btf_id` which must be `btf_id <= BTF_MAX_TYPE`.
///
/// Only permitted for the program types:-
///
/// * `BPF_PROG_TYPE_TRACING`.
/// * `BPF_PROG_TYPE_LSM`.
/// * `BPF_PROG_TYPE_STRUCT_OPS`.
/// * `BPF_PROG_TYPE_EXT`.
///
/// See the function `bpf_prog_load_check_attach()` in the linux source `kernel/bpf/syscall.c`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct AttachToBpfTypeIdentifier(pub BtfTypeIdentifier);

impl Into<u32> for AttachToBpfTypeIdentifier
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.into()
	}
}

impl AttachToBpfTypeIdentifier
{
	#[inline(always)]
	pub(crate) fn to_values(&self, program_type: bpf_prog_type, expected_attached_type: bpf_attach_type) -> Result<(bpf_prog_type, bpf_attach_type, u32, u32, u32, Option<NetworkInterfaceIndex>), ProgramError>
	{
		Ok((program_type, expected_attached_type, self.0.into(), 0, 0, None))
	}
}

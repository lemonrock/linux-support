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
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct AttachProgramTypeDetails
{
	/// In-kernel BTF type id to attach to.
	pub attach_to_bpf_type_identifier: AttachToBpfTypeIdentifier,
	
	/// A file descriptor; use `None` to attach to vmlinux.
	pub attach_to_bpf_program: Option<ProgramName>,
}

impl AttachProgramTypeDetails
{
	#[inline(always)]
	pub(crate) fn to_values(&self, program_type: bpf_prog_type, expected_attached_type: bpf_attach_type, extended_bpf_program_file_descriptors: &FileDescriptorsMap<ExtendedBpfProgramFileDescriptor>) -> Result<(bpf_prog_type, bpf_attach_type, BpfTypeFormatTypeIdentifier, RawFd, u32, Option<NetworkInterfaceIndex>), ProgramError>
	{
		let attach_prog_fd = match self.attach_to_bpf_program
		{
			None => 0,
			Some(ref program_name) => extended_bpf_program_file_descriptors.resolve(program_name)?,
		};
		
		Ok((program_type, expected_attached_type, self.attach_to_bpf_type_identifier.0, attach_prog_fd, 0, None))
	}
}

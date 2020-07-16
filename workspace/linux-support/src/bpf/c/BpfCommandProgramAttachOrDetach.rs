// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the commands `BPF_PROG_ATTACH` and `BPF_PROG_DETACH`.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandProgramAttachOrDetach
{
	/// container object to attach to.
	pub(crate) target_fd: RawFd,
	
	/// eBPF program to attach.
	pub(crate) attach_bpf_fd: RawFd,
	
	pub(crate) attach_type: bpf_attach_type,
	
	/// This must be zero for detach.
	pub(crate) attach_flags: BPF_PROG_ATTACH_flags,
	
	/// Previously attached eBPF program to replace if `BPF_F_REPLACE` is used.
	///
	/// This must be zero for detach.
	pub(crate) replace_bpf_fd: RawFd,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the commands `BPF_PROG_ATTACH` and `BPF_PROG_DETACH`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandProgramAttachOrDetach
{
	/// container object to attach to.
	pub(crate) target_fd: u32,
	
	/// eBPF program to attach.
	pub(crate) attach_bpf_fd: u32,
	
	pub(crate) attach_type: u32,
	
	pub(crate) attach_flags: u32,
	
	/// previously attached eBPF program to replace if `BPF_F_REPLACE` is used.
	pub(crate) replace_bpf_fd: u32,
}

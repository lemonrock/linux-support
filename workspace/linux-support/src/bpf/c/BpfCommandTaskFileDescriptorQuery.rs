// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for command `BPF_TASK_FD_QUERY`.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandTaskFileDescriptorQuery
{
	/// Process identifier.
	///
	/// Used for input.
	pub(crate) pid: pid_t,
	
	/// Used for input.
	pub(crate) fd: RawFd,
	
	/// Used for input.
	///
	/// Currently must be zero.
	pub(crate) flags: u32,
	
	/// Used for input as length of data pointed to by `buf`.
	/// Used for output as length of data pointed to by `buf`.
	pub(crate) buf_len: u32,
	
	/// Pointer to data of various kinds:-
	///
	/// * tp_name for tracepoint
	/// * symbol for kprobe
	/// * filename for uprobe
	pub(crate) buf: AlignedU64,
	
	/// Used for output.
	pub(crate) prog_id: ExtendedBpfProgramIdentifier,
	
	/// Used for output.
	pub(crate) fd_type: bpf_task_fd_type,
	
	/// Used for output.
	pub(crate) probe_offset: u64,
	
	/// Used for output.
	pub(crate) probe_addr: u64,
}

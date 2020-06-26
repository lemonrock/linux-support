// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for command `BPF_TASK_FD_QUERY`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandTaskFileDescriptorQuery
{
	/// Process identifier.
	///
	/// Used for input.
	pub(crate) pid: u32,
	
	/// Used for input.
	pub(crate) fd: u32,
	
	/// Used for input.
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
	pub(crate) prog_id: u32,
	
	/// Used for output.
	///
	/// A value `BPF_FD_TYPE_*`.
	pub(crate) fd_type: u32,
	
	/// Used for output.
	pub(crate) probe_offset: u64,
	
	/// Used for output.
	pub(crate) probe_addr: u64,
}

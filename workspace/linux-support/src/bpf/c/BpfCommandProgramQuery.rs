// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the command `BPF_PROG_QUERY`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandProgramQuery
{
	/// container object to query.
	pub(crate) target_fd: RawFd,
	
	pub(crate) attach_type: bpf_attach_type,
	
	pub(crate) query_flags: BPF_PROG_QUERY_flags,
	
	pub(crate) attach_flags: BPF_PROG_ATTACH_flags,
	
	pub(crate) prog_ids: AlignedU64,
	
	pub(crate) prog_cnt: u32,
}

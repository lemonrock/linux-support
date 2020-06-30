// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the commands `BPF_MAP_LOOKUP_BATCH`, `BPF_MAP_LOOKUP_AND_DELETE_BATCH`, `BPF_MAP_UPDATE_BATCH` and `BPF_MAP_DELETE_BATCH`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandMapBatch
{
	/// 'start batch', `NULL` to start from beginning.
	pub(crate) in_batch: AlignedU64,
	
	/// Output only (ignored on input): next 'start batch'.
	pub(crate) out_batch: AlignedU64,
	
	pub(crate) keys: AlignedU64,
	
	pub(crate) values: AlignedU64,
	
	/// On input, must be number of key or value elements.
	/// On output, contains number of filled elements.
	pub(crate) count: u32,
	
	pub(crate) map_fd: u32,
	
	pub(crate) elem_flags: u64,
	
	pub(crate) flags: u64,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Offsets for `mmap()` for CompletionQueue.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(super) struct io_cqring_offsets
{
	pub(super) head: u32,

	pub(super) tail: u32,

	pub(super) ring_mask: u32,

	pub(super) ring_entries: u32,

	pub(super) overflow: u32,

	pub(super) cqes: u32,

	resv: [u64; 2],
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(super) struct file_dedupe_range
{
	pub(super) src_offset: u64,
	pub(super) src_length: u64,
	pub(super) dest_count: u16,
	pub(super) reserved1: u16,
	pub(super) reserved2: u32,

	/// Actually a variable-length end-of-struct array field.
	pub(super) info: [file_dedupe_range_info; 0],
}

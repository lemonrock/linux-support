// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct fiemap
{
	pub(crate) fm_start: u64,
	pub(crate) fm_length: u64,
	pub(crate) fm_flags: u32,
	pub(crate) fm_mapped_extents: u32,
	pub(crate) fm_extent_count: u32,
	pub(crate) fm_reserved: u32,

	/// Actually a variable-length end-of-struct array field.
	pub(crate) fm_extents: [fiemap_extent; 0],
}

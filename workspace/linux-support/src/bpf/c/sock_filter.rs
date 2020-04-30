// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Sourced from `linux/filter.h`.

/// Filter block.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct sock_filter
{
	/// Actual filter code.
	pub(crate) code: u16,

	/// Jump true.
	pub(crate) jt: u8,

	/// Jump false.
	pub(crate) jf: u8,

	/// Generic multi-use field.
	pub(crate) k: u32,
}

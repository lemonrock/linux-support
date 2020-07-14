// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `BPF_MAP_` batch command.
	#[allow(missing_docs)]
	pub(crate) struct elem_flags: u64
	{
		const BPF_F_LOCK = 4;
	}
}

impl Default for elem_flags
{
	#[inline(always)]
	fn default() -> Self
	{
		elem_flags::empty()
	}
}

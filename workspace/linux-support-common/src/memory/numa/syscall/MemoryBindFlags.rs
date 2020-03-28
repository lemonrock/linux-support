// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `mbind()`.
	#[allow(missing_docs)]
	pub(crate) struct MemoryBindFlags: u32
	{
		/// Verify existing pages in the mapping.
		const MPOL_MF_STRICT = 1 << 0;

		/// Move pages owned by this process to conform to mapping.
		const MPOL_MF_MOVE = 1 << 1;

		/// Move every page to conform to mapping.
		const MPOL_MF_MOVE_ALL = 1 << 2;
	}
}

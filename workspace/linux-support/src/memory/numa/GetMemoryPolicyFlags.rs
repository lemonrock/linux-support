// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `get_mempolicy()`.
	#[allow(missing_docs)]
	pub(crate) struct GetMemoryPolicyFlags: u32
	{
		/// Return next 'il' node or node of address (whatever that means).
		///
		/// Unsupported and possibly subject to change (although it's been around a long time).
		const MPOL_F_NODE = 1 << 0;

		/// Look up vma using address.
		const MPOL_F_ADDR = 1 << 1;

		/// Query nodes allowed in cpuset.
		///
		/// Can not be combined with either MPOL_F_NODE or MPOL_F_ADDR.
		const MPOL_F_MEMS_ALLOWED = 1 << 2;
	}
}

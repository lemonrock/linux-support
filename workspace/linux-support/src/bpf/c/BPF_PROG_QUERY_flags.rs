// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for BPF_PROG_QUERY.
	#[allow(missing_docs)]
	pub(crate) struct BPF_PROG_QUERY_flags: u32
	{
		/// Query effective (directly attached + inherited from ancestor cgroups) programs that will be executed for events within a cgroup.
		/// `attach_flags` with this flag are returned only for directly attached programs.
		const BPF_F_QUERY_EFFECTIVE = 1;
	}
}

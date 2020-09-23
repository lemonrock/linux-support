// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `BPF_MAP_UPDATE_ELEM` command.
	#[allow(missing_docs)]
	pub(crate) struct BPF_MAP_UPDATE_ELEM_flags: u64
	{
		const BPF_ANY = 0;
		
		const BPF_NOEXIST = 1;
		
		const BPF_EXIST = 2;
		
		const BPF_F_LOCK = 4;
	}
}

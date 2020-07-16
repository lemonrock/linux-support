// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Query flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CgroupProgramQueryFlags
{
	/// Normal.
	Normal = BPF_PROG_QUERY_flags::empty().bits(),
	
	/// Effective.
	Effective = BPF_PROG_QUERY_flags::BPF_F_QUERY_EFFECTIVE.bits(),
}

impl ProgramQueryFlags for CgroupProgramQueryFlags
{
	#[inline(always)]
	fn to_query_flags(self) -> BPF_PROG_QUERY_flags
	{
		unsafe { transmute(self as u32) }
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub(crate) enum LockFlags
{
	Lock = BPF_MAP_UPDATE_ELEM_flags::BPF_F_LOCK.bits(),
	
	DoNotLock = BPF_MAP_UPDATE_ELEM_flags::empty().bits(),
}

impl Default for LockFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		LockFlags::DoNotLock
	}
}

impl LockFlags
{
	#[inline(always)]
	pub(crate) fn to_update_flags(self) -> BPF_MAP_UPDATE_ELEM_flags
	{
		unsafe { transmute(self as u64) }
	}
	
	#[inline(always)]
	pub(crate) fn to_elem_flags(self) -> elem_flags
	{
		unsafe { transmute(self as u64) }
	}
}

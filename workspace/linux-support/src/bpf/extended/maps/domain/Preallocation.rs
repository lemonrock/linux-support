// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Preallocate or do not preallocate this map?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum Preallocation
{
	/// Preallocate (the default for all hash super-types).
	Preallocate = BPF_MAP_CREATE_flags::empty().bits(),
	
	/// Do not preallocate.
	DoNotPreallocate = BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC.bits(),
}

impl Default for Preallocation
{
	#[inline(always)]
	fn default() -> Self
	{
		Preallocation::Preallocate
	}
}

impl Preallocation
{
	#[inline(always)]
	pub(super) fn to_map_flags(self) -> BPF_MAP_CREATE_flags
	{
		unsafe { transmute(self as u32) }
	}
}

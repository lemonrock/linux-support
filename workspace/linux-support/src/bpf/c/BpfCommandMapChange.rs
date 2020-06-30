// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the commands `BPF_MAP_LOOKUP_ELEM`, `BPF_MAP_UPDATE_ELEM` and `BPF_MAP_DELETE_ELEM`.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BpfCommandMapChange
{
	pub(crate) map_fd: u32,
	pub(crate) key: AlignedU64,
	pub(crate) value_or_next_key: BpfCommandMapChangeValueOrNextKey,
	pub(crate) flags: u64,
}

impl Default for BpfCommandMapChange
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for BpfCommandMapChange
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "BpfCommandMapChange {{ map_fd: {:?}, key: {:?}, value_or_next_key {:?}, flags: {:?} }}", self.map_fd, self.key, self.value_or_next_key, self.flags)
	}
}

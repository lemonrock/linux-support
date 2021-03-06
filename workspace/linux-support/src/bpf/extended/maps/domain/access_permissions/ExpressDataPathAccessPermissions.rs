// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Access permissions specialized for use with XDP-specific map types:-
///
/// * `BPF_MAP_TYPE_DEVMAP`.
/// * `BPF_MAP_TYPE_DEVMAP_HASH`.
/// * `BPF_MAP_TYPE_XSKMAP`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum ExpressDataPathAccessPermissions
{
	#[allow(missing_docs)]
	KernelReadWriteUserspaceReadWrite = BPF_MAP_CREATE_flags::empty().bits(),
	
	#[allow(missing_docs)]
	KernelWriteUserspaceReadWrite = BPF_MAP_CREATE_flags::BPF_F_WRONLY.bits(),
	
	#[allow(missing_docs)]
	KernelReadUserspaceReadWrite = BPF_MAP_CREATE_flags::BPF_F_RDONLY.bits(),
}

impl Default for ExpressDataPathAccessPermissions
{
	#[inline(always)]
	fn default() -> Self
	{
		ExpressDataPathAccessPermissions::KernelReadWriteUserspaceReadWrite
	}
}

impl ExpressDataPathAccessPermissions
{
	#[inline(always)]
	pub(crate) const fn to_map_flags(self) -> BPF_MAP_CREATE_flags
	{
		unsafe { BPF_MAP_CREATE_flags::from_bits_unchecked(self as u32) }
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Access permissions specialized for use with the map types:-
///
/// * `BPF_MAP_TYPE_STACK_TRACE`.
/// * `BPF_MAP_TYPE_SOCKMAP`.
/// * `BPF_MAP_TYPE_SOCKHASH`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum KernelOnlyAccessPermissions
{
	#[allow(missing_docs)]
	KernelReadUserspaceReadWrite = BPF_MAP_CREATE_flags::BPF_F_RDONLY.bits(),
	
	/// Note this differs in value to `XdpAccessPermissions::KernelWriteUserspaceRead`.
	KernelWriteUserspaceReadWrite = BPF_MAP_CREATE_flags::BPF_F_WRONLY.bits(),
	
	#[allow(missing_docs)]
	KernelReadAndWriteUserspaceReadWrite = BPF_MAP_CREATE_flags::empty().bits(),
}

impl Default for KernelOnlyAccessPermissions
{
	#[inline(always)]
	fn default() -> Self
	{
		KernelOnlyAccessPermissions::KernelReadAndWriteUserspaceReadWrite
	}
}

impl KernelOnlyAccessPermissions
{
	#[inline(always)]
	pub(crate) const fn to_map_flags(self) -> BPF_MAP_CREATE_flags
	{
		unsafe { BPF_MAP_CREATE_flags::from_bits_unchecked(self as u32) }
	}
}

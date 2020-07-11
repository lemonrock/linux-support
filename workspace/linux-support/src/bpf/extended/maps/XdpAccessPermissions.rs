// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Access permissions specialized for use with XDPs-specific map types:-
///
/// * `BPF_MAP_TYPE_DEVMAP`.
/// * `BPF_MAP_TYPE_DEVMAP_HASH`.
/// * `BPF_MAP_TYPE_XSKMAP`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum XdpAccessPermissions
{
	/// Note this differs in value to `AccessPermissions::KernelWriteUserspaceRead` and `KernelOnlyAccessPermissions::KernelWriteUserspaceRead` (which both have the same value).
	KernelWriteUserspaceRead = BPF_MAP_CREATE_flags::BPF_F_WRONLY.bits(),
	
	#[allow(missing_docs)]
	KernelReadAndWriteUserspaceRead = BPF_MAP_CREATE_flags::BPF_F_RDONLY_PROG.bits(),
}

impl Default for XdpAccessPermissions
{
	#[inline(always)]
	fn default() -> Self
	{
		XdpAccessPermissions::KernelReadAndWriteUserspaceRead
	}
}

impl XdpAccessPermissions
{
	#[inline(always)]
	const fn to_map_flags(self) -> BPF_MAP_CREATE_flags
	{
		unsafe { BPF_MAP_CREATE_flags::from_bits_unchecked(self as u32) }
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read, write or both?
///
/// See the checks in the Linux kernel function `bpf_get_file_flag()`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AccessPermissions
{
	KernelReadUserspaceWrite = BPF_MAP_CREATE_flags::BPF_F_RDONLY.bits() | BPF_MAP_CREATE_flags::BPF_F_WRONLY_PROG.bits(),
	KernelReadUserspaceReadWrite = BPF_MAP_CREATE_flags::BPF_F_RDONLY.bits(),
	
	KernelWriteUserspaceRead = BPF_MAP_CREATE_flags::BPF_F_WRONLY.bits() | BPF_MAP_CREATE_flags::BPF_F_RDONLY_PROG.bits(),
	KernelWriteUserspaceReadWrite = BPF_MAP_CREATE_flags::BPF_F_WRONLY.bits(),
	
	KernelReadAndWriteUserspaceRead = BPF_MAP_CREATE_flags::BPF_F_RDONLY_PROG.bits(),
	KernelReadAndWriteUserspaceWrite = BPF_MAP_CREATE_flags::BPF_F_WRONLY_PROG.bits(),
	KernelReadAndWriteUserspaceReadWrite = BPF_MAP_CREATE_flags::empty().bits(),
}

impl Default for AccessPermissions
{
	#[inline(always)]
	fn default() -> Self
	{
		AccessPermissions::KernelReadAndWriteUserspaceReadWrite
	}
}

impl AccessPermissions
{
	#[inline(always)]
	const fn to_map_flags(self) -> BPF_MAP_CREATE_flags
	{
		unsafe { BPF_MAP_CREATE_flags::from_bits_unchecked(self as u32) }
	}
}

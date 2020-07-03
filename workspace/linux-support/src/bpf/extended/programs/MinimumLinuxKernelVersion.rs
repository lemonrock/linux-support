// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Only of relevance to loading BPF programs with `prog_type` of `bpf_prog_type::BPF_PROG_TYPE_KPROBE`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MinimumLinuxKernelVersion
{
	/// Minimum.
	Minimum(LinuxKernelVersionNumber),
	
	/// Any.
	Any,
}

impl Default for MinimumLinuxKernelVersion
{
	#[inline(always)]
	fn default() -> Self
	{
		MinimumLinuxKernelVersion::Any
	}
}

impl MinimumLinuxKernelVersion
{
	#[inline(always)]
	fn to_u32(self) -> u32
	{
		use self::MinimumLinuxKernelVersion::*;
		
		match self
		{
			Minimum(linux_kernel_version) =>
			{
				let major: u8 = linux_kernel_version.major.try_into().unwrap();
				let minor: u8 = linux_kernel_version.minor.try_into().unwrap();
				let revision: u8 = linux_kernel_version.revision.try_into().unwrap();
				
				(major as u32) << 16 | (minor as u32) << 8 | (revision as u32)
			}
			
			Any => 0xFFFFFFFE,
		}
	}
}

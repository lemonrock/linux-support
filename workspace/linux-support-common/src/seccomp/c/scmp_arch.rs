// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum scmp_arch
{
	SCMP_ARCH_NATIVE = 0x0,
	SCMP_ARCH_X86 = 0x40000003,
	SCMP_ARCH_X86_64 = 0xc000003e,
	SCMP_ARCH_X32 = 0x4000003e,
	SCMP_ARCH_ARM = 0x40000028,
	SCMP_ARCH_AARCH64 = 0xc00000b7,
	SCMP_ARCH_MIPS = 0x8,
	SCMP_ARCH_MIPS64 = 0x80000008,
	SCMP_ARCH_MIPS64N32 = 0xa0000008,
	SCMP_ARCH_MIPSEL = 0x40000008,
	SCMP_ARCH_MIPSEL64 = 0xc0000008,
	SCMP_ARCH_MIPSEL64N32 = 0xe0000008,
	SCMP_ARCH_PPC = 0x14,
	SCMP_ARCH_PPC64 = 0x80000015,
	SCMP_ARCH_PPC64LE = 0xc0000015,
	SCMP_ARCH_S390 = 0x16,
	SCMP_ARCH_S390X = 0x80000016,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Audit architectures.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AuditArchitecture
{
	/// `AUDIT_ARCH_AARCH64`.
	AARCH64 = Self::AARCH64_,

	/// `AUDIT_ARCH_ALPHA`.
	ALPHA = Self::ALPHA_,

	/// `AUDIT_ARCH_ARCOMPACT`.
	ARCOMPACT = Self::ARCOMPACT_,

	/// `AUDIT_ARCH_ARCOMPACTBE`.
	ARCOMPACTBE = Self::ARCOMPACTBE_,

	/// `AUDIT_ARCH_ARCV2`.
	ARCV2 = Self::ARCV2_,

	/// `AUDIT_ARCH_ARCV2BE`.
	ARCV2BE = Self::ARCV2BE_,

	/// `AUDIT_ARCH_ARM`.
	ARM = Self::ARM_,

	/// `AUDIT_ARCH_ARMEB`.
	ARMEB = Self::ARMEB_,

	/// `AUDIT_ARCH_C6X`.
	C6X = Self::C6X_,

	/// `AUDIT_ARCH_C6XBE`.
	C6XBE = Self::C6XBE_,

	/// `AUDIT_ARCH_CRIS`.
	CRIS = Self::CRIS_,

	/// `AUDIT_ARCH_CSKY`.
	CSKY = Self::CSKY_,

	/// `AUDIT_ARCH_FRV`.
	FRV = Self::FRV_,

	/// `AUDIT_ARCH_H8300`.
	H8300 = Self::H8300_,

	/// `AUDIT_ARCH_HEXAGON`.
	HEXAGON = Self::HEXAGON_,

	/// `AUDIT_ARCH_I386`.
	I386 = Self::I386_,

	/// `AUDIT_ARCH_IA64`.
	IA64 = Self::IA64_,

	/// `AUDIT_ARCH_M32R`.
	M32R = Self::M32R_,

	/// `AUDIT_ARCH_M68K`.
	M68K = Self::M68K_,

	/// `AUDIT_ARCH_MICROBLAZE`.
	MICROBLAZE = Self::MICROBLAZE_,

	/// `AUDIT_ARCH_MIPS`.
	MIPS = Self::MIPS_,

	/// `AUDIT_ARCH_MIPSEL`.
	MIPSEL = Self::MIPSEL_,

	/// `AUDIT_ARCH_MIPS64`.
	MIPS64 = Self::MIPS64_,

	/// `AUDIT_ARCH_MIPS64N32`.
	MIPS64N32 = Self::MIPS64N32_,

	/// `AUDIT_ARCH_MIPSEL64`.
	MIPSEL64 = Self::MIPSEL64_,

	/// `AUDIT_ARCH_MIPSEL64N32`.
	MIPSEL64N32 = Self::MIPSEL64N32_,

	/// `AUDIT_ARCH_NDS32`.
	NDS32 = Self::NDS32_,

	/// `AUDIT_ARCH_NDS32BE`.
	NDS32BE = Self::NDS32BE_,

	/// `AUDIT_ARCH_NIOS2`.
	NIOS2 = Self::NIOS2_,

	/// `AUDIT_ARCH_OPENRISC`.
	OPENRISC = Self::OPENRISC_,

	/// `AUDIT_ARCH_PARISC`.
	PARISC = Self::PARISC_,

	/// `AUDIT_ARCH_PARISC64`.
	PARISC64 = Self::PARISC64_,

	/// `AUDIT_ARCH_PPC`.
	PPC = Self::PPC_,

	/// `AUDIT_ARCH_PPC64`.
	PPC64 = Self::PPC64_,

	/// `AUDIT_ARCH_PPC64LE`.
	PPC64LE = Self::PPC64LE_,

	/// `AUDIT_ARCH_RISCV32`.
	RISCV32 = Self::RISCV32_,

	/// `AUDIT_ARCH_RISCV64`.
	RISCV64 = Self::RISCV64_,

	/// `AUDIT_ARCH_S390`.
	S390 = Self::S390_,

	/// `AUDIT_ARCH_S390X`.
	S390X = Self::S390X_,

	/// `AUDIT_ARCH_SH`.
	SH = Self::SH_,

	/// `AUDIT_ARCH_SHEL`.
	SHEL = Self::SHEL_,

	/// `AUDIT_ARCH_SH64`.
	SH64 = Self::SH64_,

	/// `AUDIT_ARCH_SHEL64`.
	SHEL64 = Self::SHEL64_,

	/// `AUDIT_ARCH_SPARC`.
	SPARC = Self::SPARC_,

	/// `AUDIT_ARCH_SPARC64`.
	SPARC64 = Self::SPARC64_,

	/// `AUDIT_ARCH_TILEGX`.
	TILEGX = Self::TILEGX_,

	/// `AUDIT_ARCH_TILEGX32`.
	TILEGX32 = Self::TILEGX32_,

	/// `AUDIT_ARCH_TILEPRO`.
	TILEPRO = Self::TILEPRO_,

	/// `AUDIT_ARCH_UNICORE`.
	UNICORE = Self::UNICORE_,

	/// `AUDIT_ARCH_X86_64`.
	X86_64 = Self::X86_64_,

	/// `AUDIT_ARCH_XTENSA`.
	XTENSA = Self::XTENSA_,
}

impl TryFrom<u32> for AuditArchitecture
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		use self::AuditArchitecture::*;

		match value
		{
			Self::AARCH64_ => Ok(AARCH64),
			Self::ALPHA_ => Ok(ALPHA),
			Self::ARCOMPACT_ => Ok(ARCOMPACT),
			Self::ARCOMPACTBE_ => Ok(ARCOMPACTBE),
			Self::ARCV2_ => Ok(ARCV2),
			Self::ARCV2BE_ => Ok(ARCV2BE),
			Self::ARM_ => Ok(ARM),
			Self::ARMEB_ => Ok(ARMEB),
			Self::C6X_ => Ok(C6X),
			Self::C6XBE_ => Ok(C6XBE),
			Self::CRIS_ => Ok(CRIS),
			Self::CSKY_ => Ok(CSKY),
			Self::FRV_ => Ok(FRV),
			Self::H8300_ => Ok(H8300),
			Self::HEXAGON_ => Ok(HEXAGON),
			Self::I386_ => Ok(I386),
			Self::IA64_ => Ok(IA64),
			Self::M32R_ => Ok(M32R),
			Self::M68K_ => Ok(M68K),
			Self::MICROBLAZE_ => Ok(MICROBLAZE),
			Self::MIPS_ => Ok(MIPS),
			Self::MIPSEL_ => Ok(MIPSEL),
			Self::MIPS64_ => Ok(MIPS64),
			Self::MIPS64N32_ => Ok(MIPS64N32),
			Self::MIPSEL64_ => Ok(MIPSEL64),
			Self::MIPSEL64N32_ => Ok(MIPSEL64N32),
			Self::NDS32_ => Ok(NDS32),
			Self::NDS32BE_ => Ok(NDS32BE),
			Self::NIOS2_ => Ok(NIOS2),
			Self::OPENRISC_ => Ok(OPENRISC),
			Self::PARISC_ => Ok(PARISC),
			Self::PARISC64_ => Ok(PARISC64),
			Self::PPC_ => Ok(PPC),
			Self::PPC64_ => Ok(PPC64),
			Self::PPC64LE_ => Ok(PPC64LE),
			Self::RISCV32_ => Ok(RISCV32),
			Self::RISCV64_ => Ok(RISCV64),
			Self::S390_ => Ok(S390),
			Self::S390X_ => Ok(S390X),
			Self::SH_ => Ok(SH),
			Self::SHEL_ => Ok(SHEL),
			Self::SH64_ => Ok(SH64),
			Self::SHEL64_ => Ok(SHEL64),
			Self::SPARC_ => Ok(SPARC),
			Self::SPARC64_ => Ok(SPARC64),
			Self::TILEGX_ => Ok(TILEGX),
			Self::TILEGX32_ => Ok(TILEGX32),
			Self::TILEPRO_ => Ok(TILEPRO),
			Self::UNICORE_ => Ok(UNICORE),
			Self::X86_64_ => Ok(X86_64),
			Self::XTENSA_ => Ok(XTENSA),

			_ => Err(ParseNumberError::OutOfRange)
		}
	}
}

impl AuditArchitecture
{
	/// Is sixty-four bit?
	#[inline(always)]
	pub const fn is_sixty_four_bit(self) -> bool
	{
		(self as u32) & Self::SixtyFourBit != 0
	}

	/// Is little endian?
	#[inline(always)]
	pub const fn is_little_endian(self) -> bool
	{
		(self as u32) & Self::LittleEndian != 0
	}

	/// Has MIPS64N32 convention?
	#[inline(always)]
	pub const fn has_mips_n32_convention(self) -> bool
	{
		(self as u32) & Self::CONVENTION_MIPS64_N32 != 0
	}

	/// ELF machine.
	#[inline(always)]
	pub const fn elf_machine(self) -> ElfMachine
	{
		unsafe { transmute(((self as u32) & 0x0000_FFFF) as u16) }
	}

	#[inline(always)]
	pub(crate) fn optional_architecture(architecture: u32) -> Option<Self>
	{
		if architecture == 0
		{
			None
		}
		else
		{
			AuditArchitecture::try_from(architecture).ok()
		}
	}

	/// `__AUDIT_ARCH_64BIT`.
	const SixtyFourBit: u32 = 0x80000000;

	/// `__AUDIT_ARCH_LE`.
	const LittleEndian: u32 = 0x40000000;

	/// `__AUDIT_ARCH_CONVENTION_MIPS64_N32`.
	const CONVENTION_MIPS64_N32: u32 = 0x20000000;

	const AARCH64_: u32 = ElfMachine::AARCH64 as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const ALPHA_: u32 = ElfMachine::ALPHA as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const ARCOMPACT_: u32 = ElfMachine::ARCOMPACT as u16 as u32 | Self::LittleEndian;

	const ARCOMPACTBE_: u32 = ElfMachine::ARCOMPACT as u16 as u32;

	const ARCV2_: u32 = ElfMachine::ARCV2 as u16 as u32 | Self::LittleEndian;

	const ARCV2BE_: u32 = ElfMachine::ARCV2 as u16 as u32;

	const ARM_: u32 = ElfMachine::ARM as u16 as u32 | Self::LittleEndian;

	const ARMEB_: u32 = ElfMachine::ARM as u16 as u32;

	const C6X_: u32 = ElfMachine::TI_C6000 as u16 as u32 | Self::LittleEndian;

	const C6XBE_: u32 = ElfMachine::TI_C6000 as u16 as u32;

	const CRIS_: u32 = ElfMachine::CRIS as u16 as u32 | Self::LittleEndian;

	const CSKY_: u32 = ElfMachine::CSKY as u16 as u32 | Self::LittleEndian;

	const FRV_: u32 = ElfMachine::FRV as u16 as u32;

	const H8300_: u32 = ElfMachine::H8_300 as u16 as u32;

	const HEXAGON_: u32 = ElfMachine::HEXAGON as u16 as u32;

	const I386_: u32 = ElfMachine::_386 as u16 as u32 | Self::LittleEndian;

	const IA64_: u32 = ElfMachine::IA_64 as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const M32R_: u32 = ElfMachine::M32R as u16 as u32;

	const M68K_: u32 = ElfMachine::_68K as u16 as u32;

	const MICROBLAZE_: u32 = ElfMachine::MICROBLAZE as u16 as u32;

	const MIPS_: u32 = ElfMachine::MIPS as u16 as u32;

	const MIPSEL_: u32 = ElfMachine::MIPS as u16 as u32 | Self::LittleEndian;

	const MIPS64_: u32 = ElfMachine::MIPS as u16 as u32 | Self::SixtyFourBit;

	const MIPS64N32_: u32 = ElfMachine::MIPS as u16 as u32 | Self::SixtyFourBit | Self::CONVENTION_MIPS64_N32;

	const MIPSEL64_: u32 = ElfMachine::MIPS as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const MIPSEL64N32_: u32 = ElfMachine::MIPS as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian | Self::CONVENTION_MIPS64_N32;

	const NDS32_: u32 = ElfMachine::NDS32 as u16 as u32 | Self::LittleEndian;

	const NDS32BE_: u32 = ElfMachine::NDS32 as u16 as u32;

	const NIOS2_: u32 = ElfMachine::ALTERA_NIOS2 as u16 as u32 | Self::LittleEndian;

	const OPENRISC_: u32 = ElfMachine::OPENRISC as u16 as u32;

	const PARISC_: u32 = ElfMachine::PARISC as u16 as u32;

	const PARISC64_: u32 = ElfMachine::PARISC as u16 as u32 | Self::SixtyFourBit;

	const PPC_: u32 = ElfMachine::PPC as u16 as u32;

	const PPC64_: u32 = ElfMachine::PPC64 as u16 as u32 | Self::SixtyFourBit;

	const PPC64LE_: u32 = ElfMachine::PPC64 as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const RISCV32_: u32 = ElfMachine::RISCV as u16 as u32 | Self::LittleEndian;

	const RISCV64_: u32 = ElfMachine::RISCV as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const S390_: u32 = ElfMachine::S390 as u16 as u32;

	const S390X_: u32 = ElfMachine::S390 as u16 as u32 | Self::SixtyFourBit;

	const SH_: u32 = ElfMachine::SH as u16 as u32;

	const SHEL_: u32 = ElfMachine::SH as u16 as u32 | Self::LittleEndian;

	const SH64_: u32 = ElfMachine::SH as u16 as u32 | Self::SixtyFourBit;

	const SHEL64_: u32 = ElfMachine::SH as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const SPARC_: u32 = ElfMachine::SPARC as u16 as u32;

	const SPARC64_: u32 = ElfMachine::SPARCV9 as u16 as u32 | Self::SixtyFourBit;

	const TILEGX_: u32 = ElfMachine::TILEGX as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const TILEGX32_: u32 = ElfMachine::TILEGX as u16 as u32 | Self::LittleEndian;

	const TILEPRO_: u32 = ElfMachine::TILEPRO as u16 as u32 | Self::LittleEndian;

	const UNICORE_: u32 = ElfMachine::UNICORE as u16 as u32 | Self::LittleEndian;

	const X86_64_: u32 = ElfMachine::X86_64 as u16 as u32 | Self::SixtyFourBit | Self::LittleEndian;

	const XTENSA_: u32 = ElfMachine::XTENSA as u16 as u32;
}

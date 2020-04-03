// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// These constants define the various ELF target machines.
///
/// This list is a subset from Linux; for a full set, see <https://github.com/bminor/musl/blob/524e76f17b296ce36c2fb1a1469e814a045ec957/include/elf.h>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ElfMachine
{
	/// No architecture
	NONE = 0,

	/// AT&T WE 32100.
	///
	/// `EM_M32`.
	M32 = 1,

	/// Sparc.
	///
	/// `EM_SPARC`.
	SPARC = 2,

	/// Intel i386.
	///
	/// `EM_386`.
	_386 = 3,

	/// Motorola 68000 series.
	///
	/// `EM_68K`.
	_68K = 4,

	/// Motorola 88000 series.
	///
	/// `EM_88K`.
	_88K = 5,

	/// Perhaps disused.
	///
	/// `EM_486`.
	_486 = 6,

	/// Intel 80860 (?i860).
	///
	/// `EM_860`.
	_860 = 7,

	/// MIPS R3000 (officially, big-endian only).
	///
	/// `EM_MIPS`.
	MIPS = 8,

	/// MIPS R3000 little-endian and MIPS R4000 big-endian.
	///
	/// Historical.
	///
	/// `EM_MIPS_RS3_LE` and `EM_MIPS_RS4_BE`.
	MIPS_RS3_LE = 10,

	/// HPPA.
	///
	/// `EM_PARISC`.
	PARISC = 15,

	/// Sun's "V8plus".
	///
	/// `EM_SPARC32PLUS`.
	SPARC32PLUS = 18,

	/// PowerPC.
	///
	/// `EM_PPC`.
	PPC = 20,

	/// PowerPC64.
	///
	/// `EM_PPC64`.
	PPC64 = 21,

	/// Cell BE SPU.
	///
	/// `EM_SPU`.
	SPU = 23,

	/// ARM 32 bit.
	///
	/// `EM_ARM`.
	ARM = 40,

	/// SuperH.
	///
	/// `EM_SH`.
	SH = 42,

	/// SPARC v9 64-bit.
	///
	/// `EM_SPARCV9`.
	SPARCV9 = 43,

	/// Renesas H8/300.
	///
	/// `EM_H8_300`.
	H8_300 = 46,

	/// HP/Intel IA-64.
	///
	/// `EM_IA_64`.
	IA_64 = 50,

	/// AMD x86-64.
	///
	/// `EM_X86_64`.
	X86_64 = 62,

	/// IBM S/390.
	///
	/// `EM_S390`.
	S390 = 22,

	/// Axis Communications 32-bit embedded processor.
	///
	/// `EM_CRIS`.
	CRIS = 76,

	/// Renesas M32R.
	///
	/// `EM_M32R`.
	M32R = 88,

	/// Panasonic/MEI MN10300, AM33.
	///
	/// `EM_MN10300`.
	MN10300 = 89,

	/// OpenRISC 32-bit embedded processor.
	///
	/// `EM_OPENRISC`.
	OPENRISC = 92,

	/// ARCompact processor.
	///
	/// `EM_ARCOMPACT`.
	ARCOMPACT = 93,

	/// Tensilica Xtensa Architecture.
	///
	/// `EM_XTENSA`.
	XTENSA = 94,

	/// ADI Blackfin Processor.
	///
	/// `EM_BLACKFIN`.
	BLACKFIN = 106,

	/// UniCore-32.
	///
	/// `EM_UNICORE`.
	UNICORE = 110,

	/// Altera Nios II soft-core processor.
	///
	/// `EM_ALTERA_NIOS2`.
	ALTERA_NIOS2 = 113,

	/// TI C6X DSPs.
	///
	/// `EM_TI_C6000`.
	TI_C6000 = 140,

	/// QUALCOMM Hexagon.
	///
	/// `EM_HEXAGON`.
	HEXAGON = 164,

	/// Andes Technology compact code size embedded RISC processor family.
	///
	/// `EM_NDS32`.
	NDS32 = 167,

	/// ARM 64 bit.
	///
	/// `EM_AARCH64`.
	AARCH64 = 183,

	/// Tilera TILEPro.
	///
	/// `EM_TILEPRO`.
	TILEPRO = 188,

	/// Xilinx MicroBlaze.
	///
	/// `EM_MICROBLAZE`.
	MICROBLAZE = 189,

	/// Tilera TILE-Gx.
	///
	/// `EM_TILEGX`.
	TILEGX = 191,

	/// ARCv2 Cores.
	///
	/// `EM_ARCV2`.
	ARCV2 = 195,

	/// RISC-V.
	///
	/// `EM_RISCV`.
	RISCV = 243,

	/// Linux BPF.
	///
	/// `EM_BPF`.
	BPF = 247,

	/// C-SKY.
	///
	/// `EM_CSKY`.
	CSKY = 252,

	/// Fujitsu FR-V.
	///
	/// `EM_FRV`.
	FRV = 0x5441,

	/// DEC Alpha.
	///
	/// `EM_ALPHA`.
	ALPHA = 0x9026,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Programmable Interrupt Controller (PIC).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum ProgrammableInterruptControllerGenericSystemPeripheralProgrammingInterface
{
	/// Generic 8259.
	Generic8259 = 0x00,

	/// ISA.
	Isa = 0x01,

	/// EISA.
	Eisa = 0x02,

	/// I/O APIC.
	///
	/// The Base Address register at offset 10h is used to request a minimum of 32 bytes of non-prefetch-able memory.
	/// Two registers within that space are located at Base+00h (I/O Select register) and Base+10h (I/O Window register).
	IoApic = 0x10,

	/// I/O(x) APIC.
	IoXApic = 0x20,
}

impl ProgrammableInterruptControllerGenericSystemPeripheralProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::ProgrammableInterruptControllerGenericSystemPeripheralProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(Generic8259),
			0x01 => Some(Isa),
			0x02 => Some(Eisa),
			0x10 => Some(IoApic),
			0x20 => Some(IoXApic),
			_ => None,
		}
	}
}

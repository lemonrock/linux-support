// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Intelligent Input-Output (I²0) Programming Interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum IntelligentInputOutputProgrammingInterface
{
	/// ?
	MessageFifoAtOffset0x00,

	/// Intelligent I/O (I2O) Architecture Specification 1.0
	ArchitectureSpecification1DotZero(NonZeroU8),
}

impl IntelligentInputOutputProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::IntelligentInputOutputProgrammingInterface::*;

		match self
		{
			MessageFifoAtOffset0x00 => 0x00,
			ArchitectureSpecification1DotZero(non_zero) => non_zero.get(),
		}
	}

	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::IntelligentInputOutputProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(MessageFifoAtOffset0x00),
			_ => Some(ArchitectureSpecification1DotZero(unsafe { NonZeroU8::new_unchecked(programming_interface) })),
		}
	}
}

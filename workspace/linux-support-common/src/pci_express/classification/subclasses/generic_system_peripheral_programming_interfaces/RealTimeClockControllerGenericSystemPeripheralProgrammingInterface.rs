// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Real Time Clock (RTC) controller.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum RealTimeClockControllerGenericSystemPeripheralProgrammingInterface
{
	/// Generic.
	Generic = 0x00,

	/// ISA.
	Isa = 0x01,
}

impl RealTimeClockControllerGenericSystemPeripheralProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::RealTimeClockControllerGenericSystemPeripheralProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(Generic),
			0x01 => Some(Isa),
			_ => None,
		}
	}
}

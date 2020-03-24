// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Serial.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum SerialCommunicationControllerProgrammingInterface
{
	/// Generic XT compatible serial controller.
	GenericXtCompatible = 0x00,

	/// Generic 16450 compatible serial controller.
	Generic16450Compatible = 0x01,

	/// Generic 16550 compatible serial controller.
	Generic16550Compatible = 0x02,

	/// Generic 16650 compatible serial controller.
	Generic16650Compatible = 0x03,

	/// Generic 16750 compatible serial controller.
	Generic16750Compatible = 0x04,

	/// Generic 16850 compatible serial controller.
	Generic16850Compatible = 0x05,

	/// Generic 16950 compatible serial controller.
	Generic16950Compatible = 0x06,
}

impl SerialCommunicationControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::SerialCommunicationControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(GenericXtCompatible),
			0x01 => Some(Generic16450Compatible),
			0x02 => Some(Generic16550Compatible),
			0x03 => Some(Generic16650Compatible),
			0x04 => Some(Generic16750Compatible),
			0x05 => Some(Generic16850Compatible),
			0x06 => Some(Generic16950Compatible),
			_ => None,
		}
	}
}

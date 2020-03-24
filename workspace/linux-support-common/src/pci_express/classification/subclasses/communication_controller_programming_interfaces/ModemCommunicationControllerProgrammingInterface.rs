// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Modem.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum ModemCommunicationControllerProgrammingInterface
{
	/// Generic modem.
	GenericModem = 0x00,

	/// Hayes compatible modem, 16450-compatible interface.
	Hayes16450CompatibleModem = 0x01,

	/// Hayes compatible modem, 16550-compatible interface.
	Hayes16550CompatibleModem = 0x02,

	/// Hayes compatible modem, 16650-compatible interface.
	Hayes16650CompatibleModem = 0x03,

	/// Hayes compatible modem, 16750-compatible interface.
	Hayes16750CompatibleModem = 0x04,
}

impl ModemCommunicationControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::ModemCommunicationControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(GenericModem),
			0x01 => Some(Hayes16450CompatibleModem),
			0x02 => Some(Hayes16550CompatibleModem),
			0x03 => Some(Hayes16650CompatibleModem),
			0x04 => Some(Hayes16750CompatibleModem),
			_ => None,
		}
	}
}

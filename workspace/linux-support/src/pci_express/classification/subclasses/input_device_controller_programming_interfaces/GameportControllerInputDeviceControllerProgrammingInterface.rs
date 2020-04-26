// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Gameport Controller.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum GameportControllerInputDeviceControllerProgrammingInterface
{
	/// Generic.
	Generic = 0x00,

	/// Any Base Address registers in this Function that request/assign I/O address space, the registers in that I/O space conform to the standard “legacy” game ports.
	/// The byte at offset 00h in an I/O region behaves as a legacy gameport interface where reads to the byte return joystick/gamepad information, and writes to the byte start the RC timer.
	/// The byte at offset 01h is an alias of the byte at offset 00h.
	/// All other bytes in an I/O region are unspecified and can be used in vendor unique ways.
	Legacy = 0x10,
}

impl GameportControllerInputDeviceControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::GameportControllerInputDeviceControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(Generic),
			0x10 => Some(Legacy),
			_ => None,
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// IEEE 1394 (FireWire).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum FireWireSerialBusControllerProgrammingInterface
{
	/// IEEE 1394 (FireWire).
	FireWire = 0x00,

	/// IEEE 1394 following the 1394 OpenHCI specification.
	OpenHci = 0x10,
}

impl FireWireSerialBusControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::FireWireSerialBusControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(FireWire),
			0x10 => Some(OpenHci),
			_ => None,
		}
	}
}

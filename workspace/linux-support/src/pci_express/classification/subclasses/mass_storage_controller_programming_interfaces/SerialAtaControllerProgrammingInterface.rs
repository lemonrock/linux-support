// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Serial ATA Controller Programming Interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum SerialAtaControllerProgrammingInterface
{
	/// Vendor-specific.
	VendorSpecific = 0x00,

	/// Advanced Host Controller Interface (AHCI).
	AdvancedHostControllerInterface = 0x01,

	/// Serial Storage Bus Interface.
	SerialStorageBusInterface = 0x02,
}

impl SerialAtaControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::SerialAtaControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(VendorSpecific),
			0x01 => Some(AdvancedHostControllerInterface),
			0x02 => Some(SerialStorageBusInterface),
			_ => None,
		}
	}
}

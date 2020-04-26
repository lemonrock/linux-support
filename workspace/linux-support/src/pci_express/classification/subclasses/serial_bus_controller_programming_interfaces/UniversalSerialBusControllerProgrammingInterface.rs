// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Universal Serial Bus (USB).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum UniveralSerialBusControllerProgrammingInterface
{
	/// Universal Host Controller Specification (UHCI).
	///
	/// Intel proprietary USB 1.x.
	UniversalHostControllerInterface = 0x00,

	/// Open Host Controller Specification (OHCI).
	///
	/// USB 1.1.
	OpenHostControllerInterface = 0x10,

	/// Enhanced Host Controller Interface Specification (EHCI).
	///
	/// USB 2.0.
	EnhancedHostControllerInterface = 0x20,

	/// eXtensible Host Controller Interface (xHCI) Specification.
	///
	/// USB 3.0 and 3.1.
	ExtensibleHostControllerInterface = 0x30,

	/// Vendor-specific.
	VendorSpecificHostControllerInterface = 0x80,

	/// Universal Serial Bus device (not host controller).
	Device = 0xFE,
}

impl UniveralSerialBusControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::UniveralSerialBusControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(UniversalHostControllerInterface),
			0x10 => Some(OpenHostControllerInterface),
			0x20 => Some(EnhancedHostControllerInterface),
			0x30 => Some(ExtensibleHostControllerInterface),
			0x80 => Some(VendorSpecificHostControllerInterface),
			0xFE => Some(Device),
			_ => None,
		}
	}
}

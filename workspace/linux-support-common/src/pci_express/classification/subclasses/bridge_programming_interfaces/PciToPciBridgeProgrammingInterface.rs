// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI-to-PCI bridge.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum PciToPciBridgeBridgeProgrammingInterface
{
	/// PCI-to-PCI bridge.
	PciToPciBridge = 0x00,

	/// Subtractive Decode PCI-to-PCI bridge.
	///
	/// This interface code identifies the PCI-to-PCI bridge as a device that supports subtractive decoding in addition to all the currently defined functions of a PCI-to-PCI bridge.
	SubtractiveDecodePciToPciBridge = 0x01,
}

impl PciToPciBridgeBridgeProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::PciToPciBridgeBridgeProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(PciToPciBridge),
			0x01 => Some(SubtractiveDecodePciToPciBridge),
			_ => None,
		}
	}
}

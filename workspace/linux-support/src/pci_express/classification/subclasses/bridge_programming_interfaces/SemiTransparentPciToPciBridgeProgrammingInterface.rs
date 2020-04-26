// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Semi-transparent PCI-to-PCI bridge.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum SemiTransparentPciToPciBridgeProgrammingInterface
{
	/// Semi-transparent PCI-to-PCI bridge with the primary PCI bus side facing the system host processor.
	PrimaryPciBusFacesSystemHostProcessor = 0x40,

	/// Semi-transparent PCI-to-PCI bridge with the secondary PCI bus side facing the system host processor.
	SecondaryPciBusFacesSystemHostProcessor = 0x80,
}

impl SemiTransparentPciToPciBridgeProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::SemiTransparentPciToPciBridgeProgrammingInterface::*;

		match programming_interface
		{
			0x40 => Some(PrimaryPciBusFacesSystemHostProcessor),
			0x80 => Some(SecondaryPciBusFacesSystemHostProcessor),
			_ => None,
		}
	}
}

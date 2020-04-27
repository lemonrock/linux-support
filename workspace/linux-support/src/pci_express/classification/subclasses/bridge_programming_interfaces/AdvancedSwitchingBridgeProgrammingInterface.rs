// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Semi-transparent PCI-to-PCI bridge.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum AdvancedSwitchingBridgeProgrammingInterface
{
	/// Advanced Switching to PCI host bridge–Custom Interface.
	CustomInterface = 0x00,

	/// Advanced Switching to PCI host bridge–ASI-SIG Defined Portal Interface.
	DefinedPortalInterface = 0x01,
}

impl AdvancedSwitchingBridgeProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::AdvancedSwitchingBridgeProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(CustomInterface),
			0x01 => Some(DefinedPortalInterface),
			_ => None,
		}
	}
}

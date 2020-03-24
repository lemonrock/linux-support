// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// InfraRed / Ultra Wide Band (UWB) Radio Controller.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum InfraRedAndUltraWideBandRadioWirelessControllerProgrammingInterface
{
	/// Consumer InfraRed.
	ConsumerInfraRed = 0x00,

	/// Ultra Wide Band (UWB) Radio.
	UltraWideBandRadio = 0x10,
}

impl InfraRedAndUltraWideBandRadioWirelessControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::InfraRedAndUltraWideBandRadioWirelessControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(ConsumerInfraRed),
			0x10 => Some(UltraWideBandRadio),
			_ => None,
		}
	}
}

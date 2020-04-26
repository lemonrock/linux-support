// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum WirelessController
{
	/// iRDA.
	IrdaCompatible,
	
	InfraRedAndUltraWideBandRadio(InfraRedAndUltraWideBandRadioWirelessControllerProgrammingInterface),
	
	RfController,
	
	Bluetooth,
	
	Broadband,

	/// Ethernet 5GHz (802.11a).
	Ethernet5Ghz,
	
	/// Ethernet 2.4GHz (802.11b).
	Ethernet2Dot4Ghz,

	/// Cellular modem controller.
	CellularModem,

	/// Cellular modem controller plus Ethernet (802.11).
	CellularModemPlusEthernet,

	/// Other
	Other,
}

impl WirelessController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::WirelessController::*;

		match self
		{
			InfraRedAndUltraWideBandRadio(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::WirelessController::*;

		match value
		{
			0x00 => zero_programming_interface!(IrdaCompatible, programming_interface),
			0x01 => programming_interface!(InfraRedAndUltraWideBandRadio, programming_interface, InfraRedAndUltraWideBandRadioWirelessControllerProgrammingInterface),
			0x10 => zero_programming_interface!(RfController, programming_interface),
			0x11 => zero_programming_interface!(Bluetooth, programming_interface),
			0x12 => zero_programming_interface!(Broadband, programming_interface),
			0x20 => zero_programming_interface!(Ethernet5Ghz, programming_interface),
			0x21 => zero_programming_interface!(Ethernet2Dot4Ghz, programming_interface),
			0x40 => zero_programming_interface!(CellularModem, programming_interface),
			0x41 => zero_programming_interface!(CellularModemPlusEthernet, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::WirelessController::*;

		match self
		{
			IrdaCompatible => 0x00,
			InfraRedAndUltraWideBandRadio(_) => 0x01,
			RfController => 0x10,
			Bluetooth => 0x11,
			Broadband => 0x12,
			Ethernet5Ghz => 0x20,
			Ethernet2Dot4Ghz => 0x21,
			CellularModem => 0x40,
			CellularModemPlusEthernet => 0x41,
			Other => 0x80,
		}
	}
}

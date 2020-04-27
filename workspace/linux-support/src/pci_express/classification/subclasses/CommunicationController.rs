// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum CommunicationController
{
	Serial(SerialCommunicationControllerProgrammingInterface),

	Parallel(ParallelCommunicationControllerProgrammingInterface),

	MultiportSerial,

	Modem(ModemCommunicationControllerProgrammingInterface),
	
	/// GPIB (IEEE 488.1/2) controller.
	Gpib,

	SmartCard,
	
	/// No effective sub class.
	Other,
}

impl CommunicationController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::CommunicationController::*;

		match self
		{
			Serial(programming_interface) => programming_interface as u8,
			Parallel(programming_interface) => programming_interface as u8,
			Modem(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::CommunicationController::*;

		match value
		{
			0x00 => programming_interface!(Serial, programming_interface, SerialCommunicationControllerProgrammingInterface),
			0x01 => programming_interface!(Parallel, programming_interface, ParallelCommunicationControllerProgrammingInterface),
			0x02 => zero_programming_interface!(MultiportSerial, programming_interface),
			0x03 => programming_interface!(Modem, programming_interface, ModemCommunicationControllerProgrammingInterface),
			0x04 => zero_programming_interface!(Gpib, programming_interface),
			0x05 => zero_programming_interface!(SmartCard, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::CommunicationController::*;

		match self
		{
			Serial(_) => 0x00,
			Parallel(_) => 0x01,
			MultiportSerial => 0x02,
			Modem(_) => 0x03,
			Gpib => 0x04,
			SmartCard => 0x05,
			Other => 0x80,
		}
	}
}

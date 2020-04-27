// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Serial bus.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SerialBusController
{
	/// IEEE 1394.
	FireWire(FireWireSerialBusControllerProgrammingInterface),
	
	/// ACCESS.bus (sic).
	AccessBus,
	
	/// SSA.
	SSA,
	
	/// USB.
	UniversalSerialBus(UniveralSerialBusControllerProgrammingInterface),

	/// Fibre Channel.
	FibreChannel,
	
	/// SMBus
	SmBus,

	/// InfiniBand.
	///
	/// Deprecated; use NetworkController.
	InfiniBand,
	
	/// IPMI.
	Ipmi(IpmiSerialBusControllerProgrammingInterface),
	
	/// SERCOS Interface Standard (IEC61491).
	SercosInterface,
	
	/// CANbus.
	Canbus,

	/// [MIPI I3C℠  Host Controller Interface](http://software.mipi.org).
	I3C,

	/// Other.
	Other,
}

impl SerialBusController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::SerialBusController::*;

		match self
		{
			FireWire(programming_interface) => programming_interface as u8,
			UniversalSerialBus(programming_interface) => programming_interface as u8,
			Ipmi(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::SerialBusController::*;

		match value
		{
			0x00 => programming_interface!(FireWire, programming_interface, FireWireSerialBusControllerProgrammingInterface),
			0x01 => zero_programming_interface!(AccessBus, programming_interface),
			0x02 => zero_programming_interface!(SSA, programming_interface),
			0x03 => programming_interface!(UniversalSerialBus, programming_interface, UniveralSerialBusControllerProgrammingInterface),
			0x04 => zero_programming_interface!(FibreChannel, programming_interface),
			0x05 => zero_programming_interface!(SmBus, programming_interface),
			0x06 => zero_programming_interface!(InfiniBand, programming_interface),
			0x07 => programming_interface!(Ipmi, programming_interface, IpmiSerialBusControllerProgrammingInterface),
			0x08 => zero_programming_interface!(SercosInterface, programming_interface),
			0x09 => zero_programming_interface!(Canbus, programming_interface),
			0x0A => zero_programming_interface!(I3C, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::SerialBusController::*;

		match self
		{
			FireWire(_) => 0x00,
			AccessBus => 0x01,
			SSA => 0x02,
			UniversalSerialBus(_) => 0x03,
			FibreChannel => 0x04,
			SmBus => 0x05,
			#[allow(deprecated)] InfiniBand => 0x06,
			Ipmi(_) => 0x07,
			SercosInterface => 0x08,
			Canbus => 0x09,
			I3C => 0x0A,
			Other => 0x80,
		}
	}
}

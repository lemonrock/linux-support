// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Bridge.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum Bridge
{
	/// Host.
	Host,

	/// ISA.
	Isa,

	/// EISA.
	Eisa,

	/// MCA.
	MicroChannel,

	/// PCI to PCI bridge.
	PciToPci(PciToPciBridgeBridgeProgrammingInterface),
	
	/// PCMCIA.
	Pcmcia,

	/// NuBus.
	NuBus,

	/// CardBus.
	CardBus,
	
	/// RACEway is an ANSI standard (ANSI/VITA 5-1994) switching fabric.
	Raceway(RacewayBridgeProgrammingInterface),

	/// Semi-transparent PCI-to-PCI bridge.
	SemiTransparentPciToPci(SemiTransparentPciToPciBridgeProgrammingInterface),

	/// InfiniBand-to-PCI host bridge.
	InfiniBandToPciHost,

	/// Advanced Switching.
	AdvancedSwitching(AdvancedSwitchingBridgeProgrammingInterface),

	/// Other
	Other,
}

impl Bridge
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::Bridge::*;

		match self
		{
			PciToPci(programming_interface) => programming_interface as u8,
			Raceway(programming_interface) => programming_interface as u8,
			SemiTransparentPciToPci(programming_interface) => programming_interface as u8,
			AdvancedSwitching(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::Bridge::*;

		match value
		{
			0x00 => zero_programming_interface!(Host, programming_interface),
			0x01 => zero_programming_interface!(Isa, programming_interface),
			0x02 => zero_programming_interface!(Eisa, programming_interface),
			0x03 => zero_programming_interface!(MicroChannel, programming_interface),
			0x04 => programming_interface!(PciToPci, programming_interface, PciToPciBridgeBridgeProgrammingInterface),
			0x05 => zero_programming_interface!(Pcmcia, programming_interface),
			0x06 => zero_programming_interface!(NuBus, programming_interface),
			0x07 => zero_programming_interface!(CardBus, programming_interface),
			0x08 => programming_interface!(Raceway, programming_interface, RacewayBridgeProgrammingInterface),
			0x09 => programming_interface!(SemiTransparentPciToPci, programming_interface, SemiTransparentPciToPciBridgeProgrammingInterface),
			0x0A => zero_programming_interface!(InfiniBandToPciHost, programming_interface),
			0x0B => programming_interface!(AdvancedSwitching, programming_interface, AdvancedSwitchingBridgeProgrammingInterface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::Bridge::*;
		
		match self
		{
			Host => 0x00,
			Isa => 0x01,
			Eisa => 0x02,
			MicroChannel => 0x03,
			PciToPci(_) => 0x04,
			Pcmcia => 0x05,
			NuBus => 0x06,
			CardBus => 0x07,
			Raceway(_) => 0x08,
			SemiTransparentPciToPci(_) => 0x09,
			InfiniBandToPciHost => 0x0A,
			AdvancedSwitching(_) => 0x0B,
			Other => 0x80,
		}
	}
}

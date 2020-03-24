// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum NetworkController
{
	Ethernet,

	TokenRing,
	
	/// FDDI ('fibre').
	Fddi,
	
	/// ATM.
	Atm,
	
	/// ISDN.
	Isdn,
	
	WorldFip,
	
	/// PICMG Multi-Computing.
	Picmg(u8),

	Infiniband,

	Fabric,
	
	/// Other.
	Other,
}

impl NetworkController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::NetworkController::*;

		match self
		{
			Picmg(programming_interface) => programming_interface,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::NetworkController::*;

		match value
		{
			0x00 => zero_programming_interface!(Ethernet, programming_interface),
			0x01 => zero_programming_interface!(TokenRing, programming_interface),
			0x02 => zero_programming_interface!(Fddi, programming_interface),
			0x03 => zero_programming_interface!(Atm, programming_interface),
			0x04 => zero_programming_interface!(Isdn, programming_interface),
			0x05 => zero_programming_interface!(WorldFip, programming_interface),
			0x06 => Some(Picmg(programming_interface)),
			0x07 => zero_programming_interface!(Infiniband, programming_interface),
			0x08 => zero_programming_interface!(Fabric, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::NetworkController::*;

		match self
		{
			Ethernet => 0x00,
			TokenRing => 0x01,
			Fddi => 0x02,
			Atm => 0x03,
			Isdn => 0x04,
			WorldFip => 0x05,
			Picmg(_) => 0x06,
			Infiniband => 0x07,
			Fabric => 0x08,
			Other => 0x80,
		}
	}
}

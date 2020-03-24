// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum SatelliteCommunicationsController
{
	Television = 0x01,

	Audio = 0x02,

	Voice = 0x03,

	Data = 0x04,

	Other = 0x80,
}

impl SatelliteCommunicationsController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		0x00
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::SatelliteCommunicationsController::*;

		match value
		{
			0x00 => zero_programming_interface!(Television, programming_interface),
			0x01 => zero_programming_interface!(Audio, programming_interface),
			0x02 => zero_programming_interface!(Voice, programming_interface),
			0x03 => zero_programming_interface!(Data, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::SatelliteCommunicationsController::*;

		match self
		{
			Television => 0x00,
			Audio => 0x01,
			Voice => 0x02,
			Data => 0x03,
			Other => 0x80,
		}
	}
}

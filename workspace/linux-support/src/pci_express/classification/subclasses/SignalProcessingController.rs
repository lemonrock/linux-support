// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum SignalProcessingController
{
	DpioModule = 0x00,

	PerformanceCounters = 0x01,

	CommunicationSynchronizer = 0x10,

	SignalProcessingManagement = 0x20,
	
	/// Other.
	Other = 0x80,
}

impl SignalProcessingController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		0x00
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::SignalProcessingController::*;

		match value
		{
			0x00 => zero_programming_interface!(DpioModule, programming_interface),
			0x01 => zero_programming_interface!(PerformanceCounters, programming_interface),
			0x10 => zero_programming_interface!(CommunicationSynchronizer, programming_interface),
			0x20 => zero_programming_interface!(SignalProcessingManagement, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::SignalProcessingController::*;

		match self
		{
			DpioModule => 0x00,
			PerformanceCounters => 0x01,
			CommunicationSynchronizer => 0x10,
			SignalProcessingManagement => 0x20,
			Other => 0x80,
		}
	}
}

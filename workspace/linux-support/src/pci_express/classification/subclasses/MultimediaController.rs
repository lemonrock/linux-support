// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MultimediaController
{
	MultimediaVideoController,

	MultimediaAudioController,

	ComputerTelephonyDevice,

	HighDefinitionAudio(HighDefinitionAudioProgrammingInterface),
	
	/// Other.
	Other,
}

impl MultimediaController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::MultimediaController::*;

		match self
		{
			HighDefinitionAudio(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::MultimediaController::*;

		match value
		{
			0x00 => zero_programming_interface!(MultimediaVideoController, programming_interface),
			0x01 => zero_programming_interface!(MultimediaAudioController, programming_interface),
			0x02 => zero_programming_interface!(ComputerTelephonyDevice, programming_interface),
			0x03 => programming_interface!(HighDefinitionAudio, programming_interface, HighDefinitionAudioProgrammingInterface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::MultimediaController::*;

		match self
		{
			MultimediaVideoController => 0x00,
			MultimediaAudioController => 0x01,
			ComputerTelephonyDevice => 0x02,
			HighDefinitionAudio(_) => 0x03,
			Other => 0x80,
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum InputDeviceController
{
	KeyboardController,
	
	DigitizerPen,

	MouseController,

	ScannerController,

	GameportController(GameportControllerInputDeviceControllerProgrammingInterface),
	
	/// Other.
	Other,
}

impl InputDeviceController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::InputDeviceController::*;

		match self
		{
			GameportController(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::InputDeviceController::*;

		match value
		{
			0x00 => zero_programming_interface!(KeyboardController, programming_interface),
			0x01 => zero_programming_interface!(DigitizerPen, programming_interface),
			0x02 => zero_programming_interface!(MouseController, programming_interface),
			0x03 => zero_programming_interface!(ScannerController, programming_interface),
			0x04 => programming_interface!(GameportController, programming_interface, GameportControllerInputDeviceControllerProgrammingInterface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::InputDeviceController::*;

		match self
		{
			KeyboardController => 0x00,
			DigitizerPen => 0x01,
			MouseController => 0x02,
			ScannerController => 0x03,
			GameportController(_) => 0x04,
			Other => 0x80,
		}
	}
}

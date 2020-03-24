// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum DisplayController
{
	/// VGA or 8514.
	VgaOr8514CompatibleController(VgaOr8514DisplayControllerProgrammingInterface),
	
	/// XGA.
	XgaCompatibleController,
	
	/// 3D.
	_3DController,
	
	/// Other.
	Other,
}

impl DisplayController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::DisplayController::*;

		match self
		{
			VgaOr8514CompatibleController(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::DisplayController::*;

		match value
		{
			0x00 => programming_interface!(VgaOr8514CompatibleController, programming_interface, VgaOr8514DisplayControllerProgrammingInterface),
			0x01 => zero_programming_interface!(XgaCompatibleController, programming_interface),
			0x02 => zero_programming_interface!(_3DController, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::DisplayController::*;

		match self
		{
			VgaOr8514CompatibleController(_) => 0x00,
			XgaCompatibleController => 0x01,
			_3DController => 0x02,
			Other => 0x80,
		}
	}
}

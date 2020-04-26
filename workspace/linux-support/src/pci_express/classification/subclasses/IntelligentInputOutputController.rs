// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Intelligent Input-Output (I²0) Controller.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum IntelligentInputOutputController
{
	/// Intelligent Input-Output (I²0) Controller.
	IntelligentInputOutput(IntelligentInputOutputProgrammingInterface),
}

impl IntelligentInputOutputController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::IntelligentInputOutputController::*;

		match self
		{
			IntelligentInputOutput(programming_interface) => programming_interface.programming_interface(),
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::IntelligentInputOutputController::*;

		match value
		{
			0x00 => programming_interface!(IntelligentInputOutput, programming_interface, IntelligentInputOutputProgrammingInterface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::IntelligentInputOutputController::*;

		match self
		{
			IntelligentInputOutput(_) => 0x00,
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory Controller.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum MemoryController
{
	/// RAM.
	RamMemory = 0x00,

	/// Flash.
	FlashMemory = 0x01,
	
	/// Other.
	Other = 0x80,
}

impl MemoryController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		0x00
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::MemoryController::*;

		match value
		{
			0x00 => zero_programming_interface!(RamMemory, programming_interface),
			0x01 => zero_programming_interface!(FlashMemory, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::MemoryController::*;

		match self
		{
			RamMemory => 0x00,
			FlashMemory => 0x01,
			Other => 0x80,
		}
	}
}

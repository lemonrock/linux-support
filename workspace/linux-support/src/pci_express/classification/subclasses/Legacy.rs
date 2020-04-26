// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This base class is defined to provide backward compatibility for devices that were built before the Class Code field was defined.
///
/// No new devices should use this value and existing devices should switch to a more appropriate value if possible.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum Legacy
{
	NonVgaUnclassifiedDevice = 0x00,
	VgaCompatibleUnclassifiedDevice = 0x01,
}

impl Legacy
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		0x00
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::Legacy::*;

		match value
		{
			0x00 => zero_programming_interface!(NonVgaUnclassifiedDevice, programming_interface),
			0x01 => zero_programming_interface!(VgaCompatibleUnclassifiedDevice, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::Legacy::*;

		match self
		{
			NonVgaUnclassifiedDevice => 0x00,
			VgaCompatibleUnclassifiedDevice => 0x01,
		}
	}
}

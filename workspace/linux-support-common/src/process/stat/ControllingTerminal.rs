// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A controlling terminal.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControllingTerminal
{
	/// Character device major.
	pub character_device_major: u8,

	/// Character device minor (20 bits used).
	pub character_device_minor_u20: u32,
}

impl ParseNumber for ControllingTerminal
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let value = i32::parse_number(bytes, radix, parse_byte)?;
		let value = value as u32;

		Ok
		(
			Self
			{
				character_device_major: (value >> 8) as u8,
				character_device_minor_u20: ((value & 0b1111_1111) as u32) | ((value >> 20) as u32)
			}
		)
	}
}

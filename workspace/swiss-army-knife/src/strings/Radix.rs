// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Radix
{
	Binary = 2,
	Octal = 8,
	Decimal = 10,
	Hexadecimal = 16,
	Base32 = 32,
	Base36 = 36,
}

impl Radix
{
	#[inline(always)]
	fn parse_byte(self, byte: u8, non_numeric_digit_case: NonNumericDigitCase) -> Result<u8, ParseNumberError>
	{
		if let Some(value) = self.is_numeric(byte)
		{
			Ok(value)
		}
		else if let Some(value) = non_numeric_digit_case.contains(byte, self)
		{
			Ok(value)
		}
		else
		{
			Err(ParseNumberError::InvalidByte { byte })
		}
	}

	#[inline(always)]
	fn parse_byte_either_case(self, byte: u8) -> Result<u8, ParseNumberError>
	{
		if let Some(value) = self.is_numeric(byte)
		{
			Ok(value)
		}
		else if let Some(value) = NonNumericDigitCase::Upper.contains(byte, self)
		{
			Ok(value)
		}
		else if let Some(value) = NonNumericDigitCase::Lower.contains(byte, self)
		{
			Ok(value)
		}
		else
		{
			Err(ParseNumberError::InvalidByte { byte })
		}
	}

	#[inline(always)]
	fn is_numeric(self, byte: u8) -> Option<u8>
	{
		if byte >= b'0' && byte < self.ten_or_less()
		{
			Some(byte - b'0')
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn ten_or_less(self) -> u8
	{
		min(self as u8, 10)
	}
}

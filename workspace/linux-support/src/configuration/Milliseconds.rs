// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Milliseconds.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Milliseconds(pub u32);

impl From<u32> for Milliseconds
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		Self(value)
	}
}

impl Into<u32> for Milliseconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl IntegerIntoLineFeedTerminatedByteString for Milliseconds
{
	#[inline(always)]
	fn unpadded_octal(self) -> Cow<'static, [u8]>
	{
		self.0.unpadded_octal()
	}
	
	#[inline(always)]
	fn unpadded_decimal(self) -> Cow<'static, [u8]>
	{
		self.0.unpadded_decimal()
	}
	
	#[inline(always)]
	fn unpadded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
	{
		self.0.unpadded_lower_case_hexadecimal()
	}
	
	#[inline(always)]
	fn zero_padded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
	{
		self.0.zero_padded_lower_case_hexadecimal()
	}
	
	#[inline(always)]
	fn unpadded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
	{
		self.0.unpadded_upper_case_hexadecimal()
	}
	
	#[inline(always)]
	fn zero_padded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
	{
		self.0.zero_padded_upper_case_hexadecimal()
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for Milliseconds
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(self.0).into_line_feed_terminated_byte_string()
	}
}

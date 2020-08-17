// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A percentage.
///
/// Why Linux models this (a) as something that can be negative and (b) as something that can be far, far greater than 100% is beyond me.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Percentage(pub i32);

impl From<i32> for Percentage
{
	#[inline(always)]
	fn from(value: i32) -> Self
	{
		Self(value)
	}
}

impl Into<i32> for Percentage
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0
	}
}

impl IntegerIntoLineFeedTerminatedByteString for Percentage
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

impl<'a> IntoLineFeedTerminatedByteString<'a> for Percentage
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(self.0).into_line_feed_terminated_byte_string()
	}
}

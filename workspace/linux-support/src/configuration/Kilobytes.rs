// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Kilobytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Kilobytes(pub usize);

impl From<usize> for Kilobytes
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		Self(value)
	}
}

impl Into<usize> for Kilobytes
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl IntegerIntoLineFeedTerminatedByteString for Kilobytes
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

impl<'a> IntoLineFeedTerminatedByteString<'a> for Kilobytes
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(Self.0).into_line_feed_terminated_byte_string()
	}
}

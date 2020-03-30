// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Write bytes
pub trait NumberAsBytes: Sized
{
	/// Octal.
	#[inline(always)]
	fn octal(self, bytes_index: usize, bytes: &mut [u8]) -> usize
	{
		self.number_as_bytes(bytes_index, bytes, Radix::Octal, NonNumericDigitCase::Lower)
	}

	/// Decimal.
	#[inline(always)]
	fn decimal(self, bytes_index: usize, bytes: &mut [u8]) -> usize
	{
		self.number_as_bytes(bytes_index, bytes, Radix::Decimal, NonNumericDigitCase::Lower)
	}

	/// Hexadecimal using lower case alphabetic digits.
	#[inline(always)]
	fn lowercase_hexadecimal(self, bytes_index: usize, bytes: &mut [u8]) -> usize
	{
		self.number_as_bytes(bytes_index, bytes, Radix::Hexdecimal, NonNumericDigitCase::Upper)
	}

	/// Hexadecimal using upper case alphabetic digits.
	#[inline(always)]
	fn uppercase_hexadecimal(self, bytes_index: usize, bytes: &mut [u8]) -> usize
	{
		self.number_as_bytes(bytes_index, bytes, Radix::Hexdecimal, NonNumericDigitCase::Lower)
	}

	/// Number as bytes.
	fn number_as_bytes(self, bytes_index: usize, bytes: &mut [u8], radix: Radix, non_numeric_digit_case: NonNumericDigitCase) -> usize;
}

macro_rules! unsigned_number_as_bytes
{
	($type: ty) =>
	{
		impl NumberAsBytes for $type
		{
			fn number_as_bytes(self, mut bytes_index: usize, bytes: &mut [u8], radix: Radix, non_numeric_digit_case: NonNumericDigitCase) -> usize
			{
				let radix = radix as u8 as Self;
				let mut x: Self = self;
				loop
				{
					let remainder = x % radix;
					x = x / radix;

					bytes[bytes_index] = if remainder < 10
					{
						b'0' + (remainder as u8)
					}
					else
					{
						(non_numeric_digit_case as u8) + ((remainder - 10) as u8)
					};

					bytes_index -= 1;
					if x == 0
					{
						return bytes_index
					}
				}
			}
		}
	}
}
unsigned_number_as_bytes!(u8);
unsigned_number_as_bytes!(u16);
unsigned_number_as_bytes!(u32);
unsigned_number_as_bytes!(u64);
unsigned_number_as_bytes!(u128);
unsigned_number_as_bytes!(usize);

macro_rules! signed_number_as_bytes
{
	($signed_type: ty, $unsigned_type: ty) =>
	{
		impl NumberAsBytes for $signed_type
		{
			#[inline(always)]
			fn number_as_bytes(self, bytes_index: usize, bytes: &mut [u8], radix: Radix, non_numeric_digit_case: NonNumericDigitCase) -> usize
			{
				if self < 0
				{
					let bytes_index = ((-self) as $unsigned_type).number_as_bytes(bytes_index, bytes, radix, non_numeric_digit_case);
					bytes[bytes_index] = b'-';
					bytes_index - 1
				}
				else
				{
					(self as $unsigned_type).number_as_bytes(bytes_index, bytes, radix, non_numeric_digit_case)
				}
			}
		}
	}
}
signed_number_as_bytes!(i8, u8);
signed_number_as_bytes!(i16, u16);
signed_number_as_bytes!(i32, u32);
signed_number_as_bytes!(i64, u64);
signed_number_as_bytes!(i128, u128);
signed_number_as_bytes!(isize, usize);

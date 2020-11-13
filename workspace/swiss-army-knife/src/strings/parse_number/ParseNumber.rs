// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parse a number from bytes.
pub trait ParseNumber: Sized
{
	/// Parses a lower-case hexadecimal number which should be of fixed width.
	#[inline(always)]
	fn parse_hexadecimal_number_lower_case_fixed_width(bytes: &[u8], fixed_width: usize) -> Result<Self, ParseNumberError>
	{
		if unlikely!(bytes.len() != fixed_width)
		{
			return Err(ParseNumberError::HexadecimalFixedWidthNumberHasWrongNumberOfBytes { fixed_width })
		}

		Self::parse_hexadecimal_number_lower_case(bytes)
	}

	/// Parses a lower-case hexadecimal number which should be of fixed width and should start `0x`.
	#[inline(always)]
	fn parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(bytes: &[u8], fixed_width: usize) -> Result<Self, ParseNumberError>
	{
		let bytes = ParseNumberError::validate_prefix_minimum_width(bytes, b"0x", fixed_width)?;
		if unlikely!(bytes.len() != fixed_width)
		{
			return Err(ParseNumberError::HexadecimalFixedWidthNumberHasWrongNumberOfBytes { fixed_width })
		}

		Self::parse_hexadecimal_number_lower_case(bytes)
	}

	/// Parses an octal number.
	#[inline(always)]
	fn parse_octal_number_fixed_width(bytes: &[u8], fixed_width: usize) -> Result<Self, ParseNumberError>
	{
		if unlikely!(bytes.len() != fixed_width)
		{
			return Err(ParseNumberError::OctalFixedWidthNumberHasWrongNumberOfBytes { fixed_width })
		}

		Self::parse_octal_number(bytes)
	}

	/// Parses an octal number which should start '0o'.
	#[inline(always)]
	fn parse_binary_number_case_with_0b_prefix(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		let bytes = ParseNumberError::validate_prefix(bytes, b"0b")?;
		Self::parse_octal_number(bytes)
	}

	/// Parses an octal number which should start '0o'.
	#[inline(always)]
	fn parse_octal_number_case_with_0o_prefix(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_octal_number_with_prefix(bytes, b"0o")
	}

	/// Parses an octal number which should start '0'.
	#[inline(always)]
	fn parse_octal_number_case_with_0_prefix(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_octal_number_with_prefix(bytes, b"0")
	}

	#[doc(hidden)]
	#[inline(always)]
	fn parse_octal_number_with_prefix(bytes: &[u8], prefix: &'static [u8]) -> Result<Self, ParseNumberError>
	{
		let bytes = ParseNumberError::validate_prefix(bytes, prefix)?;
		Self::parse_octal_number(bytes)
	}

	/// Parses a lower-case hexadecimal number which should start '0x'.
	#[inline(always)]
	fn parse_hexadecimal_number_lower_case_with_0x_prefix(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		let bytes = ParseNumberError::validate_prefix(bytes, b"0x")?;
		Self::parse_hexadecimal_number_lower_case(bytes)
	}

	/// Parses an upper-case hexadecimal number which should start '0x'.
	#[inline(always)]
	fn parse_hexadecimal_number_upper_case_with_0x_prefix(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		let bytes = ParseNumberError::validate_prefix(bytes, b"0x")?;
		Self::parse_hexadecimal_number_upper_case(bytes)
	}

	/// Parses a hexadecimal number with any mixture of alphabet casing which should start '0x'.
	#[inline(always)]
	fn parse_hexadecimal_number_upper_or_lower_case_with_0x_prefix(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		let bytes = ParseNumberError::validate_prefix(bytes, b"0x")?;
		Self::parse_hexadecimal_number_upper_or_lower_case(bytes)
	}

	/// Parses an octal number.
	#[inline(always)]
	fn parse_octal_number(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_number_known_case(bytes, Radix::Octal, NonNumericDigitCase::Lower)
	}

	/// Parses a decimal number.
	#[inline(always)]
	fn parse_decimal_number(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_number_known_case(bytes, Radix::Decimal, NonNumericDigitCase::Lower)
	}

	/// Parses a lower-case hexadecimal number.
	#[inline(always)]
	fn parse_hexadecimal_number_lower_case(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_number_known_case(bytes, Radix::Decimal, NonNumericDigitCase::Lower)
	}

	/// Parses an upper-case hexadecimal number.
	#[inline(always)]
	fn parse_hexadecimal_number_upper_case(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_number_known_case(bytes, Radix::Decimal, NonNumericDigitCase::Upper)
	}

	/// Parses a hexadecimal number with any mixture of alphabet casing.
	#[inline(always)]
	fn parse_hexadecimal_number_upper_or_lower_case(bytes: &[u8]) -> Result<Self, ParseNumberError>
	{
		Self::parse_number_upper_or_lower_case(bytes, Radix::Hexdecimal)
	}

	/// Parses a number with a known (upper or lower) case or returns an error.
	#[inline(always)]
	fn parse_number_known_case(bytes: &[u8], radix: Radix, non_numeric_digit_case: NonNumericDigitCase) -> Result<Self, ParseNumberError>
	{
		Self::parse_number(bytes, radix, |radix, byte| radix.parse_byte(byte, non_numeric_digit_case))
	}

	/// Parses a number with any mixture of alphabet casing or returns an error.
	#[inline(always)]
	fn parse_number_upper_or_lower_case(bytes: &[u8], radix: Radix) -> Result<Self, ParseNumberError>
	{
		Self::parse_number(bytes, radix, Radix::parse_byte_either_case)
	}

	#[doc(hidden)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>;
}

macro_rules! unsigned_parse_number
{
	($type: ty) =>
	{
		impl ParseNumber for $type
		{
			fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
			{
				use self::ParseNumberError::*;

				if unlikely!(bytes.len() < 1)
				{
					return Err(TooShort)
				}

				let mut parsed_number: Self = 0;
				let scalar = radix as Self;
				for &byte in bytes
				{
					let scaled = parsed_number.checked_mul(scalar).ok_or(ScalingOverflow)?;
					parsed_number = scaled.checked_add(parse_byte(radix, byte)? as Self).ok_or(AddOverflow)?;
				}
				Ok(parsed_number)
			}
		}
	}
}
unsigned_parse_number!(u8);
unsigned_parse_number!(u16);
unsigned_parse_number!(u32);
unsigned_parse_number!(u64);
unsigned_parse_number!(u128);
unsigned_parse_number!(usize);

macro_rules! signed_parse_number
{
	($type: ty) =>
	{
		impl ParseNumber for $type
		{
			fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
			{
				use self::ParseNumberError::*;

				if unlikely!(bytes.len() < 1)
				{
					return Err(TooShort)
				}

				let first_byte = bytes[0];
				let (bytes, sign) = if first_byte == b'-'
				{
					const SizeOfMinusSign: usize = 1;
					if unlikely!(bytes.len() < (1 + SizeOfMinusSign))
					{
						return Err(TooShortWithMinusSign)
					}
					(&bytes[SizeOfMinusSign .. ], -1)
				}
				else
				{
					(bytes, 1)
				};

				let mut parsed_number: Self = 0;
				let scalar = radix as Self;
				for &byte in bytes
				{
					let scaled = parsed_number.checked_mul(scalar).ok_or(ScalingOverflow)?;
					parsed_number = scaled.checked_add(parse_byte(radix, byte)? as Self).ok_or(AddOverflow)?;
				}
				Ok(parsed_number * sign)
			}
		}
	}
}
signed_parse_number!(i8);
signed_parse_number!(i16);
signed_parse_number!(i32);
signed_parse_number!(i64);
signed_parse_number!(i128);
signed_parse_number!(isize);

macro_rules! parse_non_zero_number
{
	($non_zero_type: ty, $type: ident, $constructor: path) =>
	{
		impl ParseNumber for $non_zero_type
		{
			#[inline(always)]
			fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
			{
				let value = $type::parse_number(bytes, radix, parse_byte)?;
				if unlikely!(value == 0)
				{
					return Err(ParseNumberError::WasZero)
				}
				Ok($constructor(value))
			}
		}

		impl ParseNumber for Option<$non_zero_type>
		{
			#[inline(always)]
			fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
			{
				let value = $type::parse_number(bytes, radix, parse_byte)?;
				if unlikely!(value == 0)
				{
					Ok(None)
				}
				else
				{
					Ok(Some($constructor(value)))
				}
			}
		}
	}
}
parse_non_zero_number!(std::num::NonZeroU8, u8, new_non_zero_u8);
parse_non_zero_number!(std::num::NonZeroU16, u16, new_non_zero_u16);
parse_non_zero_number!(std::num::NonZeroU32, u32, new_non_zero_u32);
parse_non_zero_number!(std::num::NonZeroU64, u64, new_non_zero_u64);
parse_non_zero_number!(std::num::NonZeroU128, u128, new_non_zero_u128);
parse_non_zero_number!(std::num::NonZeroUsize, usize, new_non_zero_usize);
parse_non_zero_number!(std::num::NonZeroI8, i8, new_non_zero_i8);
parse_non_zero_number!(std::num::NonZeroI16, i16, new_non_zero_i16);
parse_non_zero_number!(std::num::NonZeroI32, i32, new_non_zero_i32);
parse_non_zero_number!(std::num::NonZeroI64, i64, new_non_zero_i64);
parse_non_zero_number!(std::num::NonZeroI128, i128, new_non_zero_i128);
parse_non_zero_number!(std::num::NonZeroIsize, isize, new_non_zero_isize);

impl<P: ParseNumberOption> ParseNumber for Option<P>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		P::parse_number_option(bytes, radix, parse_byte)
	}
}

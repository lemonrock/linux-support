// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct PercentEncodedByteProvider;

impl ByteProvider for PercentEncodedByteProvider
{
	type Error = PercentDecodeError;
	
	const OneSliceLength: NonZeroUsize = Self::slice_length_non_zero_usize(One);
	
	const TwoSliceLength: NonZeroUsize = Self::slice_length_non_zero_usize(Two);
	
	const ThreeSliceLength: NonZeroUsize = Self::slice_length_non_zero_usize(Three);
	
	const FourSliceLength: NonZeroUsize = Self::slice_length_non_zero_usize(Four);
	
	#[inline(always)]
	fn first(bytes: &[u8]) -> Result<u8, InvalidUtf8ParseError<Self::Error>>
	{
		use InvalidUtf8ParseError::*;
		
		if bytes.len() < Self::OneSliceLength.get()
		{
			return Err(DidNotExpectEndParsing(One))
		}
		
		Self::byte::<{One}>(bytes).map_err(Inner)
	}
	
	#[inline(always)]
	fn two(bytes: &[u8]) -> Result<u8, Self::Error>
	{
		Self::percent_then_byte::<{Two}>(bytes)
	}
	
	#[inline(always)]
	fn three(bytes: &[u8]) -> Result<(u8, u8), Self::Error>
	{
		Ok
		(
			(
				Self::percent_then_byte::<{Two}>(bytes)?,
				Self::percent_then_byte::<{Three}>(bytes)?,
			)
		)
	}
	
	#[inline(always)]
	fn four(bytes: &[u8]) -> Result<(u8, u8, u8), Self::Error>
	{
		Ok
		(
			(
				Self::percent_then_byte::<{Two}>(bytes)?,
				Self::percent_then_byte::<{Three}>(bytes)?,
				Self::percent_then_byte::<{Four}>(bytes)?,
			)
		)
	}
}

impl PercentEncodedByteProvider
{
	#[inline(always)]
	fn percent_then_byte<const decoded_byte_number: Utf8CharacterLength>(bytes: &[u8]) -> Result<u8, PercentDecodeError>
	{
		debug_assert!(bytes.len() >= Self::slice_length_non_zero_usize(decoded_byte_number).get());
		
		// Rust does not permit this value to be const, yet.
		let index = Self::index_of_first_hex_byte::<decoded_byte_number>() - 1;
		let potential_percent = bytes.get_unchecked_value_safe(index);
		if potential_percent != Percent
		{
			return Err(PercentDecodeError::MissingPercentSign { decoded_byte_number: decoded_byte_number as u8, invalid: potential_percent })
		}
		Self::byte::<decoded_byte_number>(bytes)
	}
	
	#[inline(always)]
	fn byte<const decoded_byte_number: Utf8CharacterLength>(bytes: &[u8]) -> Result<u8, PercentDecodeError>
	{
		debug_assert!(bytes.len() >= Self::slice_length_non_zero_usize(decoded_byte_number).get());
		
		let upper_nibble = Self::hex_digit::<decoded_byte_number, 0>(bytes)?;
		let lower_nibble = Self::hex_digit::<decoded_byte_number, 1>(bytes)?;
		let byte = upper_nibble | lower_nibble;
		Ok(byte)
	}
	
	#[inline(always)]
	fn hex_digit<const decoded_byte_number: Utf8CharacterLength, const relative_index: u8>(bytes: &[u8]) -> Result<u8, PercentDecodeError>
	{
		// Rust does not permit these values to be const, yet.
		let shift = Self::shift::<relative_index>();
		let index = Self::index_of_first_hex_byte::<decoded_byte_number>() + relative_index;
		
		let potential_hex_digit = bytes.get_unchecked_value_safe(index);
		let correction = match potential_hex_digit
		{
			_0 ..= _9 => _0,
			
			A ..= F => A - 10,
			
			a ..= f => a - 10,
			
			_ => return Err(PercentDecodeError::InvalidHexDigit { decoded_byte_number: decoded_byte_number as u8, relative_index, invalid: potential_hex_digit }),
		};
		let value = potential_hex_digit - correction;
		
		Ok(value << shift)
	}
	
	/*
		decoded_byte_number => index
		1 => ((1 * 3) - 1) - 2 + ri => 0 + relative_index
		X X
		0 1
		
		2 => ((2 * 3) - 1) - 2 + ri => 3 + relative_index
		X X % X X
		0 1 2 3 4
		
		3 => ((3 * 3) - 1) - 2 + ri => 6 + relative_index
		X X % X X % X X
		0 1 2 3 4 5 6 7
		
		4 => ((4 * 3) - 1) - 2 + ri => 9 + relative_index
		X X % X X % X X % X X
		0 1 2 3 4 5 6 7 8 9 10
	 */
	#[inline(always)]
	const fn index_of_first_hex_byte<const decoded_byte_number: Utf8CharacterLength>() -> u8
	{
		Self::slice_length_u8(decoded_byte_number).get() - 2
	}
	
	#[inline(always)]
	const fn shift<const relative_index: u8>() -> u8
	{
		const NibbleInBits: u8 = 4;
		
		if relative_index > 1
		{
			panic!("relative_index too large")
		}
		
		(1 - relative_index) * NibbleInBits
	}
	
	#[inline(always)]
	const fn slice_length_non_zero_usize(decoded_byte_number: Utf8CharacterLength) -> NonZeroUsize
	{
		new_non_zero_usize(Self::slice_length_u8(decoded_byte_number).get() as usize)
	}
	
	#[inline(always)]
	const fn slice_length_u8(decoded_byte_number: Utf8CharacterLength) -> NonZeroU8
	{
		new_non_zero_u8(((decoded_byte_number as u8) * 3) - 1)
	}
}

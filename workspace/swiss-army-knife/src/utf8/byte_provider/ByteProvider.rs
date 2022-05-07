// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) trait ByteProvider: Sized
{
	type Error: error::Error;
	
	const OneSliceLength: NonZeroUsize;
	
	const TwoSliceLength: NonZeroUsize;
	
	const ThreeSliceLength: NonZeroUsize;
	
	const FourSliceLength: NonZeroUsize;
	
	fn first(bytes: &[u8]) -> Result<u8, InvalidUtf8ParseError<Self::Error>>;
	
	fn two(bytes: &[u8]) -> Result<u8, Self::Error>;
	
	fn three(bytes: &[u8]) -> Result<(u8, u8), Self::Error>;
	
	fn four(bytes: &[u8]) -> Result<(u8, u8, u8), Self::Error>;
	
	#[inline(always)]
	fn decode_internal_utf8_validity_already_checked(string: &str) -> (Utf8SequenceAndCharacter, &str)
	{
		let bytes = string.as_bytes();
		
		let first =
		{
			let first = Self::first(bytes);
			unsafe { first.unwrap_unchecked() }
		};
		
		#[inline(always)]
		fn parse_more_than_one<BP: ByteProvider, U8SNC: Utf8SequenceNonConst>(first: u8, bytes: &[u8]) -> (Utf8SequenceAndCharacter, NonZeroUsize)
		where Utf8SequenceEnum: From<U8SNC>
		{
			let slice_length = U8SNC::slice_length::<BP>();
			debug_assert!(bytes.len() >= slice_length.get());
			
			let remainder = U8SNC::parse::<BP>(bytes);
			let remainder = unsafe { remainder.unwrap_unchecked() };
			let utf8_sequence = U8SNC::construct(first, remainder);
			debug_assert!(utf8_sequence.try_into_char().is_ok());
			
			(utf8_sequence.into_unchecked_utf8_sequence_and_character(), slice_length)
		}
		
		let (utf8_sequence_and_character, slice_length) = if Utf8Sequence1::is(first)
		{
			(unsafe { Utf8SequenceAndCharacter::from_unchecked([first]) }, Utf8Sequence1::slice_length::<Self>())
		}
		else if Utf8Sequence2::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence2>(first, bytes)
		}
		else if Utf8Sequence3::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence3>(first, bytes)
		}
		else
		{
			debug_assert!(Utf8Sequence4::is(first));
			parse_more_than_one::<Self, Utf8Sequence4>(first, bytes)
		};
		
		(utf8_sequence_and_character, unsafe { from_utf8_unchecked(Self::postamble(bytes, slice_length)) })
	}
	
	#[inline(always)]
	fn decode_internal<const to_ascii_lower_case: bool>(bytes: &[u8]) -> Result<(Utf8SequenceAndCharacter, &[u8]), InvalidUtf8ParseError<Self::Error>>
	{
		#[inline(always)]
		fn parse_more_than_one<BP: ByteProvider, U8SNC: Utf8SequenceNonConst>(first: u8, bytes: &[u8]) -> Result<(Utf8SequenceAndCharacter, NonZeroUsize), InvalidUtf8ParseError<BP::Error>>
		{
			let slice_length = U8SNC::slice_length::<BP>();
			
			if bytes.len() < slice_length.get()
			{
				return Err(InvalidUtf8ParseError::DidNotExpectEndParsing(U8SNC::Length))
			}
			
			let remainder = U8SNC::parse::<BP>(bytes).map_err(InvalidUtf8ParseError::Inner)?;
			let utf8_sequence = U8SNC::construct(first, remainder);
			Ok((utf8_sequence.try_into_utf8_sequence_and_character()?, slice_length))
		}
		
		let first = Self::first(bytes)?;
		
		let (utf8_sequence_and_character, slice_length) = if Utf8Sequence1::is(first)
		{
			// Different logic to if branches below as:-
			// (a) there is no need to check the slice length
			// (b) is always valid as a `char`.
			(unsafe { Utf8SequenceAndCharacter::from_unchecked([first]) }, Utf8Sequence1::slice_length::<Self>())
		}
		else if Utf8Sequence2::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence2>(first, bytes)?
		}
		else if Utf8Sequence3::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence3>(first, bytes)?
		}
		else if Utf8Sequence4::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence4>(first, bytes)?
		}
		else
		{
			return Err(InvalidUtf8ParseError::OverlyLongUtf8Sequence)
		};
		
		Ok((utf8_sequence_and_character, Self::postamble(bytes, slice_length)))
	}
	
	#[inline(always)]
	fn postamble(bytes: &[u8], slice_length: NonZeroUsize) -> &[u8]
	{
		bytes.get_unchecked_range_safe(slice_length.get() .. )
	}
}

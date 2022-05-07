// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// UTF-8 sequence of 2 bytes.
pub type Utf8Sequence2 = [u8; 2];

impl const Utf8Sequence for Utf8Sequence2
{
	const Length: Utf8CharacterLength = Two;
	
	type Remainder = u8;
	
	#[inline(always)]
	fn construct(first: u8, remainder: Self::Remainder) -> Self
	{
		let second = remainder;
		[first, second]
	}
	
	#[inline(always)]
	fn is(first: u8) -> bool
	{
		first & xE0 == 0xC0
	}
	
	#[inline(always)]
	fn into_raw_unicode_code_point(self) -> u32
	{
		let first = self[0];
		let second = self[1];
		((first as u32) & x1F) << Shift6 | (second as u32)
	}
	
	#[inline(always)]
	fn try_into_char(self) -> Result<char, CharTryFromError>
	{
		char::try_from(self.into_raw_unicode_code_point())
	}
	
	#[inline(always)]
	unsafe fn unchecked_into_char(self) -> char
	{
		char::from_u32_unchecked(self.into_raw_unicode_code_point())
	}
	
	#[inline(always)]
	fn encode_character(character: char) -> Self
	{
		Self::encode_u32(character as u32)
	}
	
	#[inline(always)]
	fn encode_u32(code: u32) -> Self
	{
		[
			(code >> Shift6 & x1F) as u8 | TAG_TWO_B,
			(code & x3F) as u8 | TAG_CONT
		]
	}
	
	#[inline(always)]
	fn write_unchecked(self, to: NonNull<u8>)
	{
		let pointer = to.as_ptr().cast::<Self>();
		unsafe { pointer.write(self) }
	}
	
	#[inline(always)]
	fn into_unchecked_utf8_sequence_and_character(self) -> Utf8SequenceAndCharacter
	{
		unsafe { Utf8SequenceAndCharacter::from_unchecked(self) }
	}
	
	#[inline(always)]
	fn try_into_utf8_sequence_and_character(self) -> Result<Utf8SequenceAndCharacter, CharTryFromError>
	{
		Utf8SequenceAndCharacter::try_from(self)
	}
}

impl const Utf8SequenceCrate for Utf8Sequence2
{
	#[inline(always)]
	fn slice_length<BP: ByteProvider>() -> NonZeroUsize
	{
		BP::TwoSliceLength
	}
}

impl Utf8SequenceNonConst for Utf8Sequence2
{
	#[inline(always)]
	fn parse<BP: ByteProvider>(bytes: &[u8]) -> Result<<Self as Utf8Sequence>::Remainder, BP::Error>
	{
		BP::two(bytes)
	}
}

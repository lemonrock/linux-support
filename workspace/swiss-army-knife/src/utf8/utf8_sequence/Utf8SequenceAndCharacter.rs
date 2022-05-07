// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Contains an UTF-8 sequence and its decoded character form, for efficiency.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Utf8SequenceAndCharacter(pub Utf8SequenceEnum, pub char);

impl const Default for Utf8SequenceAndCharacter
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(Utf8SequenceEnum::default(), 0x00 as char)
	}
}

impl From<char> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	fn from(character: char) -> Self
	{
		Self(Utf8SequenceEnum::from(character), character)
	}
}

impl const FromUnchecked<Utf8Sequence1> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(utf8_sequence: Utf8Sequence1) -> Self
	{
		Self
		(
			Utf8SequenceEnum::from(utf8_sequence),
			utf8_sequence.unchecked_into_char()
		)
	}
}

impl const FromUnchecked<Utf8Sequence2> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(utf8_sequence: Utf8Sequence2) -> Self
	{
		Self
		(
			Utf8SequenceEnum::from(utf8_sequence),
			utf8_sequence.unchecked_into_char()
		)
	}
}

impl const FromUnchecked<Utf8Sequence3> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(utf8_sequence: Utf8Sequence3) -> Self
	{
		Self
		(
			Utf8SequenceEnum::from(utf8_sequence),
			utf8_sequence.unchecked_into_char()
		)
	}
}

impl const FromUnchecked<Utf8Sequence4> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(utf8_sequence: Utf8Sequence4) -> Self
	{
		Self
		(
			Utf8SequenceEnum::from(utf8_sequence),
			utf8_sequence.unchecked_into_char()
		)
	}
}

impl const FromUnchecked<u8> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(ascii_byte: u8) -> Self
	{
		Self
		(
			Utf8SequenceEnum::One([ascii_byte]),
			ascii_byte as char
		)
	}
}

impl const TryFrom<Utf8Sequence1> for Utf8SequenceAndCharacter
{
	type Error = CharTryFromError;
	
	#[inline(always)]
	fn try_from(utf8_sequence: Utf8Sequence1) -> Result<Self, Self::Error>
	{
		Ok
		(
			Self
			(
				Utf8SequenceEnum::from(utf8_sequence),
				utf8_sequence.try_into_char()?
			)
		)
	}
}

impl const TryFrom<Utf8Sequence2> for Utf8SequenceAndCharacter
{
	type Error = CharTryFromError;
	
	#[inline(always)]
	fn try_from(utf8_sequence: Utf8Sequence2) -> Result<Self, Self::Error>
	{
		Ok
		(
			Self
			(
				Utf8SequenceEnum::from(utf8_sequence),
				utf8_sequence.try_into_char()?
			)
		)
	}
}

impl const TryFrom<Utf8Sequence3> for Utf8SequenceAndCharacter
{
	type Error = CharTryFromError;
	
	#[inline(always)]
	fn try_from(utf8_sequence: Utf8Sequence3) -> Result<Self, Self::Error>
	{
		Ok
		(
			Self
			(
				Utf8SequenceEnum::from(utf8_sequence),
				utf8_sequence.try_into_char()?
			)
		)
	}
}

impl const TryFrom<Utf8Sequence4> for Utf8SequenceAndCharacter
{
	type Error = CharTryFromError;
	
	#[inline(always)]
	fn try_from(utf8_sequence: Utf8Sequence4) -> Result<Self, Self::Error>
	{
		Ok
		(
			Self
			(
				Utf8SequenceEnum::from(utf8_sequence),
				utf8_sequence.try_into_char()?
			)
		)
	}
}

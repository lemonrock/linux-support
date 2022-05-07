// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An UTF-8 sequence.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Utf8SequenceEnum
{
	#[allow(missing_docs)]
	One(Utf8Sequence1) = One.into(),
	
	#[allow(missing_docs)]
	Two(Utf8Sequence2) = Two.into(),
	
	#[allow(missing_docs)]
	Three(Utf8Sequence3) = Three.into(),
	
	#[allow(missing_docs)]
	Four(Utf8Sequence4) = Four.into(),
}

impl const Default for Utf8SequenceEnum
{
	#[inline(always)]
	fn default() -> Self
	{
		Utf8SequenceEnum::One([0x00])
	}
}

impl From<char> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(character: char) -> Self
	{
		use Utf8SequenceEnum::*;
		
		encode_utf8_callback(character, (), |_, utf8_sequence_1| One(utf8_sequence_1), |_, utf8_sequence_2| Two(utf8_sequence_2), |_, utf8_sequence_3| Three(utf8_sequence_3), |_, utf8_sequence_4| Four(utf8_sequence_4))
	}
}

impl const From<Utf8Sequence1> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence1) -> Self
	{
		Utf8SequenceEnum::One(utf8_sequence)
	}
}

impl const From<Utf8Sequence2> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence2) -> Self
	{
		Utf8SequenceEnum::Two(utf8_sequence)
	}
}

impl const From<Utf8Sequence3> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence3) -> Self
	{
		Utf8SequenceEnum::Three(utf8_sequence)
	}
}

impl const From<Utf8Sequence4> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence4) -> Self
	{
		Utf8SequenceEnum::Four(utf8_sequence)
	}
}

impl Utf8SequenceEnum
{
	/// UTF-8 character length.
	#[inline(always)]
	pub const fn utf8_character_length(&self) -> Utf8CharacterLength
	{
		unsafe { transmute(discriminant(self)) }
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A validated slice of UTF-8 sequences; typically a Rust `str`.
pub trait ValidatedDecodeUtf8Sequences: UnvalidatedDecodeUtf8Sequences
{
	/// Decode the next UTF-8 sequence whose validity has already been checked.
	fn decode_next_utf8_validity_already_checked(&mut self) -> Option<Utf8SequenceAndCharacter>;
}

impl<'a> ValidatedDecodeUtf8Sequences for &'a str
{
	#[inline(always)]
	fn decode_next_utf8_validity_already_checked(&mut self) -> Option<Utf8SequenceAndCharacter>
	{
		let string = *self;
		
		if string.is_empty()
		{
			return None
		}
		
		let (utf8_sequence_and_character, remaining_string) = BytesByteProvider::decode_internal_utf8_validity_already_checked(string);
		*self = remaining_string;
		Some(utf8_sequence_and_character)
	}
}

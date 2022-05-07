// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An extension trait for pushing and encoding UTF-8 onto a `String`.
pub trait StringUtf8Encoding: Sized
{
	#[allow(missing_docs)]
	#[inline(always)]
	fn push_utf8_sequence_enum(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		use Utf8SequenceEnum::*;
		
		match utf8_sequence
		{
			One(utf8_sequence) => self.push_utf8_sequence(utf8_sequence),
			
			Two(utf8_sequence) => self.push_utf8_sequence(utf8_sequence),
			
			Three(utf8_sequence) => self.push_utf8_sequence(utf8_sequence),
			
			Four(utf8_sequence) => self.push_utf8_sequence(utf8_sequence),
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn push_utf8_ascii_byte(&mut self, ascii_byte: u8) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_byte(ascii_byte));
		self.push_utf8_sequence([ascii_byte])
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn push_utf8_character(&mut self, character: char) -> Result<(), TryReserveError>
	{
		encode_utf8_callback
		(
			character,
			self,
			Self::push_utf8_sequence,
			Self::push_utf8_sequence,
			Self::push_utf8_sequence,
			Self::push_utf8_sequence
		)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn from_utf8_unchecked_with_character(utf8_sequences: &[u8], character: char) -> Result<Self, TryReserveError>
	{
		encode_utf8_callback
		(
			character,
			utf8_sequences,
			Self::from_utf8_unchecked_with_utf8_sequence,
			Self::from_utf8_unchecked_with_utf8_sequence,
			Self::from_utf8_unchecked_with_utf8_sequence,
			Self::from_utf8_unchecked_with_utf8_sequence
		)
	}
	
	#[allow(missing_docs)]
	fn push_utf8_sequence<U8S: Utf8Sequence>(&mut self, utf8_sequence: U8S) -> Result<(), TryReserveError>;
	
	#[allow(missing_docs)]
	fn from_utf8_unchecked_with_utf8_sequence<U8S: Utf8Sequence>(utf8_sequences: &[u8], utf8_sequence: U8S) -> Result<Self, TryReserveError>;
}

impl StringUtf8Encoding for String
{
	#[allow(missing_docs)]
	#[inline(always)]
	fn push_utf8_sequence<U8S: Utf8Sequence>(&mut self, utf8_sequence: U8S) -> Result<(), TryReserveError>
	{
		let old_length = self.len();
		
		let buffer = unsafe { self.as_mut_vec() };
		buffer.try_reserve(U8S::UsizeLength)?;
		
		let mut_ptr = buffer.as_mut_ptr();
		utf8_sequence.write_unchecked(new_non_null(unsafe { mut_ptr.add(old_length) }));
		
		let new_length = old_length + U8S::UsizeLength;
		unsafe { buffer.set_len(new_length) };
		Ok(())
	}
	
	#[inline(always)]
	fn from_utf8_unchecked_with_utf8_sequence<U8S: Utf8Sequence>(utf8_sequences: &[u8], utf8_sequence: U8S) -> Result<Self, TryReserveError>
	{
		let old_length = utf8_sequences.len();
		let minimum_capacity = old_length + U8S::UsizeLength;
		
		let mut string = String::new();
		string.try_reserve(minimum_capacity)?;
		
		let to_pointer = string.as_mut_ptr();
		{
			let from_pointer = utf8_sequences.as_ptr();
			unsafe { to_pointer.copy_from_nonoverlapping(from_pointer, old_length) }
		}
		
		utf8_sequence.write_unchecked(new_non_null(unsafe { to_pointer.add(old_length) }));
		unsafe { string.as_mut_vec().set_len(minimum_capacity) };
		
		Ok(string)
	}
}

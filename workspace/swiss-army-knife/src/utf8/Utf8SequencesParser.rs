// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a string being parsed from a byte slice of UTF-8 sequences, where some bytes may have been encoded (e.g. using percent encoding or C-escape sequences).
///
/// Allows re-use of the byte slice if no encoded sequences are present, thus avoiding memory allocation from the heap.
///
/// Converts to a `Cow` when parsing is done.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Utf8SequencesParser<'a>
{
	#[doc(hidden)]
	Heap(String),
	
	#[doc(hidden)]
	Stack
	{
		from: NonNull<u8>,
		
		slice_length: usize,
	
		marker: PhantomData<&'a [u8]>,
	},
}

impl<'a> From<Utf8SequencesParser<'a>> for Cow<'a, str>
{
	#[inline(always)]
	fn from(value: Utf8SequencesParser<'a>) -> Self
	{
		value.to_cow()
	}
}

impl<'a> From<Utf8SequencesParser<'a>> for Cow<'a, [u8]>
{
	#[inline(always)]
	fn from(value: Utf8SequencesParser<'a>) -> Self
	{
		value.to_cow_bytes()
	}
}

impl<'a> Utf8SequencesParser<'a>
{
	/// New stack instance.
	#[inline(always)]
	pub fn new_stack(remaining_utf8_bytes: &mut impl AsRef<[u8]>) -> Self
	{
		let from = (*remaining_utf8_bytes).as_ref().as_ptr();
		Self::new_stack_raw(new_non_null(from as *mut u8), 0)
	}
	
	/// New heap instance.
	#[inline(always)]
	pub fn new_heap(utf8_sequence: Utf8SequenceEnum) -> Result<Self, TryReserveError>
	{
		let mut string = String::new();
		string.push_utf8_sequence_enum(utf8_sequence)?;
		Ok(Utf8SequencesParser::Heap(string))
	}
	
	/// New stack instance from raw pointers.
	#[inline(always)]
	pub const fn new_stack_raw(from: NonNull<u8>, slice_length: usize) -> Self
	{
		Utf8SequencesParser::Stack
		{
			from,
			slice_length,
			marker: PhantomData,
		}
	}
	
	/// To a `Cow` as a str.
	#[inline(always)]
	pub fn to_cow(self) -> Cow<'a, str>
	{
		use Utf8SequencesParser::*;
		
		match self
		{
			Heap(string) => Cow::Owned(string),
			
			Stack { from, slice_length, .. } => Cow::Borrowed(Self::to_str(from, slice_length))
		}
	}
	
	/// To a `Cow` as bytes.
	#[inline(always)]
	pub fn to_cow_bytes(self) -> Cow<'a, [u8]>
	{
		use Utf8SequencesParser::*;
		
		match self
		{
			Heap(string) => Cow::Owned(string.into_bytes()),
			
			Stack { from, slice_length, .. } => Cow::Borrowed(Self::to_bytes(from, slice_length))
		}
	}
	
	/// Push a `ascii_character` which is known to be ASCII.
	///
	/// `ascii_character` only validated with an assertion in debug builds.
	#[inline(always)]
	pub fn push_ascii_character(&mut self, ascii_character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_character(ascii_character));
		self.push_ascii_byte(ascii_character as u8)
	}
	
	/// Push a `ascii_byte` which is known to be ASCII.
	///
	/// `ascii_byte` only validated with an assertion in debug builds.
	#[inline(always)]
	pub fn push_ascii_byte(&mut self, ascii_byte: u8) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_byte(ascii_byte));
		
		use Utf8SequencesParser::*;
		match self
		{
			Heap(string) => string.push_utf8_ascii_byte(ascii_byte),
			
			Stack { slice_length, .. } =>
			{
				*slice_length = *slice_length + 1;
				Ok(())
			},
		}
	}
	
	/// Push an UTF8 sequence enum.
	///
	/// ***Use of this method implies that your surrounding logic is not performant***.
	///
	/// Prefer more specific `push_*()` methods, such as `push_utf8_sequence()`, over this since an `Utf8SequenceEnum` should only be necessary as the result of decoding some bytes (returned by a method).
	/// And, if data is decoded, it should be pushed to force a heap (`push_forcing_heap_utf8_sequence()`).
	#[inline(always)]
	pub fn push_utf8_sequence_enum(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
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
	
	/// Push an UTF8 sequence enum which is known to be a `Utf8SequenceEnum::One`.
	///
	/// `utf8_sequence` only validated with an assertion in debug builds.
	/// Prefer the method `push_ascii_byte()` over this.
	#[inline(always)]
	pub fn push_utf8_sequence_enum_1(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, One)
	}
	
	/// Push an UTF8 sequence enum which is known to be a `Utf8SequenceEnum::Two`.
	///
	/// `utf8_sequence` only validated with an assertion in debug builds.
	#[inline(always)]
	pub fn push_utf8_sequence_enum_2(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, Two)
	}
	
	/// Push an UTF8 sequence enum which is known to be a `Utf8SequenceEnum::Three`.
	///
	/// `utf8_sequence` only validated with an assertion in debug builds.
	#[inline(always)]
	pub fn push_utf8_sequence_enum_3(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, Three)
	}
	
	/// Push an UTF8 sequence enum which is known to be a `Utf8SequenceEnum::Four`.
	///
	/// `utf8_sequence` only validated with an assertion in debug builds.
	#[inline(always)]
	pub fn push_utf8_sequence_enum_4(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, Four)
	}
	
	/// Push an UTF8 sequence.
	#[inline(always)]
	pub fn push_utf8_sequence<U8S: Utf8Sequence>(&mut self, utf8_sequence: U8S) -> Result<(), TryReserveError>
	{
		use Utf8SequencesParser::*;
		match self
		{
			Heap(string) => string.push_utf8_sequence(utf8_sequence),
			
			Stack { slice_length, .. } =>
			{
				let old_slice_length = *slice_length;
				*slice_length = old_slice_length + U8S::UsizeLength;
				Ok(())
			}
		}
	}
	
	/// Push an `ascii_character`.
	///
	/// `ascii_character` only validated with an assertion in debug builds.
	///
	/// Forces conversion to a heap (String).
	#[inline(always)]
	pub fn push_forcing_heap_ascii_character<const to_ascii_lower_case: bool>(&mut self, ascii_character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_character(ascii_character));
		self.push_forcing_heap_ascii_byte::<to_ascii_lower_case>(ascii_character as u8)
	}
	
	/// Push an `ascii_byte`.
	///
	/// `ascii_byte` only validated with an assertion in debug builds.
	///
	/// Forces conversion to a heap (String).
	#[inline(always)]
	pub fn push_forcing_heap_ascii_byte<const to_ascii_lower_case: bool>(&mut self, ascii_byte: u8) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_byte(ascii_byte));
		
		let ascii_byte = if to_ascii_lower_case
		{
			to_lower_case_ascii_byte(ascii_byte)
		}
		else
		{
			ascii_byte
		};
		
		use Utf8SequencesParser::*;
		match self
		{
			Heap(string) => string.push_utf8_sequence([ascii_byte]),
			
			Stack { from, slice_length, .. } =>
			{
				let utf8_sequences = Self::utf8_sequences(from, slice_length);
				let string = String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, [ascii_byte])?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	/// Push an UTF8 sequence decoded from encoded or escaped data.
	///
	/// Forces conversion to a heap (String).
	#[inline(always)]
	pub fn push_forcing_heap_utf8_sequence_enum(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		use Utf8SequencesParser::*;
		
		match self
		{
			Heap(string) => string.push_utf8_sequence_enum(utf8_sequence)?,
			
			Stack { from, slice_length, .. } =>
			{
				let utf8_sequences = Self::utf8_sequences(from, slice_length);
				use Utf8SequenceEnum::*;
				let string = match utf8_sequence
				{
					One(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
					
					Two(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
					
					Three(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
					
					Four(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
				}?;
				
				*self = Heap(string);
			}
		}
		Ok(())
	}
	
	/// Push a character.
	///
	/// Use this method if `character` is the result of decoding an escape sequence, say.
	///
	/// Forces conversion to a heap (String).
	#[inline(always)]
	pub fn push_forcing_heap(&mut self, character: char) -> Result<(), TryReserveError>
	{
		use Utf8SequencesParser::*;
		match self
		{
			Heap(string) => string.push_utf8_character(character),
			
			Stack { from, slice_length, .. } =>
			{
				let utf8_sequences = Self::utf8_sequences(from, slice_length);
				let string = String::from_utf8_unchecked_with_character(utf8_sequences, character)?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	fn utf8_sequences(from: &mut NonNull<u8>, slice_length: &mut usize) -> &'a [u8]
	{
		let from = (*from).as_ptr();
		let slice_length = *slice_length;
		unsafe { from_raw_parts(from, slice_length) }
	}
	
	#[inline(always)]
	fn to_str(from: NonNull<u8>, slice_length: usize) -> &'a str
	{
		let range = Self::to_bytes(from, slice_length);
		unsafe { from_utf8_unchecked(range) }
	}
	
	#[inline(always)]
	fn to_bytes(from: NonNull<u8>, slice_length: usize) -> &'a [u8]
	{
		unsafe { from_raw_parts(from.as_ptr(), slice_length) }
	}
}

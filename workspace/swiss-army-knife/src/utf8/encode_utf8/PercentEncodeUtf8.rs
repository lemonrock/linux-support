// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct PercentEncodeUtf8;

impl PercentEncodeUtf8
{
	#[inline(always)]
	pub(super) fn encode(string: &str, percent_encode_ascii: impl Copy + FnOnce(u8) -> bool) -> Result<Cow<str>, TryReserveError>
	{
		let bytes_length = string.len();
		for index in 0 .. bytes_length
		{
			let byte = string.get_unchecked_value_safe(index);
			if Utf8Sequence1::is(byte)
			{
				if percent_encode_ascii(byte)
				{
					// TODO: We get byte again and call percent_encode_ascii() again, unnecessarily.
					return Self::encode_assuming_remainder_should_be_percent_encoded(index, string, percent_encode_ascii)
				}
			}
			else
			{
				// TODO: We get byte again and call do Utf8Sequence1::is() again, unnecessarily.
				return Self::encode_assuming_remainder_should_be_percent_encoded(index, string, percent_encode_ascii)
			}
		}
		Ok(Cow::Borrowed(string))
	}
	
	#[inline(always)]
	fn encode_assuming_remainder_should_be_percent_encoded(from_index: usize, string: &str, percent_encode_ascii: impl Copy + FnOnce(u8) -> bool) -> Result<Cow<str>, TryReserveError>
	{
		let mut remaining_string = string.get_unchecked_range_safe(from_index .. );
		
		let ascii_bytes_length = from_index;
		
		let mut buffer =
		{
			let mut buffer = Vec::new();
			let maximum_memory_if_every_byte_needs_percent_encoding = remaining_string.len() * PercentEncodedUtf8ByteSize;
			buffer.try_reserve(ascii_bytes_length + maximum_memory_if_every_byte_needs_percent_encoding)?;
			buffer
		};
		
		let original_pointer =
		{
			let copy_to = buffer.as_mut_ptr();
			unsafe { string.as_ptr().copy_to(copy_to, ascii_bytes_length) };
			new_non_null(unsafe { copy_to.add(ascii_bytes_length) })
		};
		
		let remaining_utf8_bytes = &mut remaining_string;
		let mut current_pointer = original_pointer;
		loop
		{
			match remaining_utf8_bytes.decode_next_utf8_validity_already_checked()
			{
				None => break,
				
				Some(utf8_sequence_and_character) =>
				{
					current_pointer = Self::write_character(current_pointer, percent_encode_ascii, utf8_sequence_and_character);
				}
			}
		}
		
		{
			let length = ascii_bytes_length + Self::pointer_difference_in_bytes(current_pointer, original_pointer);
			buffer.set_length(length);
		}
		
		buffer.shrink_to_fit();
		Ok(Cow::Owned(unsafe { String::from_utf8_unchecked(buffer) }))
	}
	
	#[inline(always)]
	fn write_character(current_pointer: NonNull<u8>, percent_encode_ascii: impl Copy + FnOnce(u8) -> bool, utf8_sequence_and_character: Utf8SequenceAndCharacter) -> NonNull<u8>
	{
		let Utf8SequenceAndCharacter(utf8_sequence, _) = utf8_sequence_and_character;
		
		use Utf8SequenceEnum::*;
		match utf8_sequence
		{
			One(utf8_sequence) =>
			{
				let byte = utf8_sequence[0];
				if percent_encode_ascii(byte)
				{
					Self::write_percent_encoded_byte(current_pointer, byte)
				}
				else
				{
					Self::write_ascii_byte(current_pointer, byte)
				}
			}
			
			Two(utf8_sequence) =>
			{
				let current_pointer = Self::write_percent_encoded_byte(current_pointer, utf8_sequence[0]);
				Self::write_percent_encoded_byte(current_pointer, utf8_sequence[1])
			}
			
			Three(utf8_sequence) =>
			{
				let current_pointer = Self::write_percent_encoded_byte(current_pointer, utf8_sequence[0]);
				let current_pointer = Self::write_percent_encoded_byte(current_pointer, utf8_sequence[1]);
				Self::write_percent_encoded_byte(current_pointer, utf8_sequence[2])
			}
			
			Four(utf8_sequence) =>
			{
				let current_pointer = Self::write_percent_encoded_byte(current_pointer, utf8_sequence[0]);
				let current_pointer = Self::write_percent_encoded_byte(current_pointer, utf8_sequence[1]);
				let current_pointer = Self::write_percent_encoded_byte(current_pointer, utf8_sequence[2]);
				Self::write_percent_encoded_byte(current_pointer, utf8_sequence[3])
			}
		}
	}
	
	#[inline(always)]
	const fn write_ascii_byte(current_pointer: NonNull<u8>, byte: u8) -> NonNull<u8>
	{
		Self::write_to_pointer(current_pointer, byte)
	}
	
	#[inline(always)]
	const fn write_percent_encoded_byte(current_pointer: NonNull<u8>, byte: u8) -> NonNull<u8>
	{
		let val = Self::byte_to_percent_encoded_bytes(byte);
		Self::write_to_pointer(current_pointer, val)
	}
	
	#[inline(always)]
	const fn write_to_pointer<T: Sized>(current_pointer: NonNull<u8>, val: T) -> NonNull<u8>
	{
		#[inline(always)]
		const fn increment_current_pointer<T>(current_pointer: NonNull<u8>) -> NonNull<u8>
		{
			let current_pointer = current_pointer.as_ptr();
			let bytes_written = size_of::<T>();
			unsafe { new_non_null(current_pointer.add(bytes_written)) }
		}
		
		let to_pointer = current_pointer.as_ptr().cast::<T>();
		unsafe { to_pointer.write(val) };
		increment_current_pointer::<T>(current_pointer)
	}
	
	#[inline(always)]
	fn pointer_difference_in_bytes(current_pointer: NonNull<u8>, original_pointer: NonNull<u8>) -> usize
	{
		(current_pointer.as_ptr() as usize) - (original_pointer.as_ptr() as usize)
	}
	
	#[inline(always)]
	const fn byte_to_percent_encoded_bytes(byte: u8) -> [u8; PercentEncodedUtf8ByteSize]
	{
		#[inline(always)]
		const fn nibble_to_byte(nibble: u8) -> u8
		{
			let correction = match nibble
			{
				0x0 ..= 0x9 => _0,
				
				0xA ..= 0xF => A,
				
				_ => unreachable_code_const("nibble"),
			};
			nibble + correction
		}
		
		[Percent, nibble_to_byte(byte >> 4), nibble_to_byte(byte & 0b1111)]
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Unescapes a Linux string escaped using the `seq_file_path()` function.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxStringEscapeSequence
{
	unescaped_byte: u8,
	octal_sequence_left_padded_with_zero: [u8; 3]
}

impl LinuxStringEscapeSequence
{
	/// Horizontal Tab (TAB), `\t`.
	pub const HorizontalTab: Self = Self
	{
		unescaped_byte: b'\t',
		octal_sequence_left_padded_with_zero: *b"011",
	};

	/// Line Feed (LF), `\n`.
	pub const LineFeed: Self = Self
	{
		unescaped_byte: b'\n',
		octal_sequence_left_padded_with_zero: *b"012",
	};

	/// Space (SP), ` `.
	pub const Space: Self = Self
	{
		unescaped_byte: b' ',
		octal_sequence_left_padded_with_zero: *b"040",
	};

	const StartOfEscapeSequenceLength: usize = 1;

	const RemainingEscapeSequenceLength: usize = 3;

	const EscapeSequenceLength: usize = Self::StartOfEscapeSequenceLength + Self::RemainingEscapeSequenceLength;

	/// New sequence.
	#[inline(always)]
	pub fn new(unescaped_byte: u8) -> Self
	{
		#[allow(deprecated)]
		let mut octal_sequence_left_padded_with_zero: [u8; 3] = unsafe_uninitialized();
		let mut byte_index = unescaped_byte.octal(2, &mut octal_sequence_left_padded_with_zero);
		while byte_index != 0
		{
			octal_sequence_left_padded_with_zero[byte_index] = b'0';
			byte_index -= 1;
		}

		Self
		{
			unescaped_byte,
			octal_sequence_left_padded_with_zero,
		}
	}

	/// Works backwards as that requires copy operations to move fewer bytes in total.
	pub fn unescape_linux_string(mut linux_string_with_escaped_characters: Vec<u8>, escape_sequences: &[Self]) -> Vec<u8>
	{
		let mut unescaped_length = linux_string_with_escaped_characters.len();
		{
			let mut remaining_bytes = &mut linux_string_with_escaped_characters[..];
			while likely!(remaining_bytes.len() >= Self::EscapeSequenceLength)
			{
				const StartOfEscapeSequence: u8 = b'\\';
				let index = match (&remaining_bytes[ .. remaining_bytes.len() - Self::RemainingEscapeSequenceLength]).memchr(StartOfEscapeSequence)
				{
					None => break,
					Some(index) => index,
				};

				Self::unescape_if_escaped(escape_sequences, index, remaining_bytes, &mut unescaped_length);

				remaining_bytes = &mut remaining_bytes[ .. index]
			}
		}

		unsafe { linux_string_with_escaped_characters.set_len(unescaped_length) };
		linux_string_with_escaped_characters.shrink_to_fit();
		linux_string_with_escaped_characters
	}

	#[inline(always)]
	fn unescape_if_escaped(escape_sequences: &[Self], index: usize, remaining_bytes: &mut [u8], unescaped_length: &mut usize)
	{
		let inclusive_start_index = index + 1;
		let exclusive_end_index = inclusive_start_index + Self::RemainingEscapeSequenceLength;

		let potentially_escaped: &[u8; 3] = (&remaining_bytes[inclusive_start_index .. exclusive_end_index]).try_into().unwrap();
		for escape_sequence in escape_sequences
		{
			if &escape_sequence.octal_sequence_left_padded_with_zero == potentially_escaped
			{
				unsafe
				{
					remaining_bytes.set_unchecked_mut_safe(index, escape_sequence.unescaped_byte);
					let from = remaining_bytes.as_ptr().add(exclusive_end_index);
					let to = remaining_bytes.as_mut_ptr().add(inclusive_start_index);
					from.copy_to(to, remaining_bytes.len() - exclusive_end_index);
					*unescaped_length = *unescaped_length - Self::RemainingEscapeSequenceLength;
				}
				break
			}
		}
	}
}

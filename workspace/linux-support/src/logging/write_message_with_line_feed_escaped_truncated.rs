// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
unsafe fn write_message_with_line_feed_escaped_truncated(mut write_to: *mut u8, message: &str, end_pointer: *mut u8) -> (*mut u8, bool)
{
	const Slash: u8 = b'\\';
	const LineFeed: u8 = b'\n';
	
	const EscapeSequenceLength: usize = 2;
	
	#[inline(always)]
	fn choose_escape_sequence(character_to_escape_index: usize, remaining_bytes: &[u8]) -> [u8; EscapeSequenceLength]
	{
		const EscapedSlash: [u8; EscapeSequenceLength] = [Slash, Slash];
		const EscapedLineFeed: [u8; EscapeSequenceLength] = [Slash, LineFeed];
		
		match unsafe { * remaining_bytes.get_unchecked(character_to_escape_index) }
		{
			Slash => EscapedSlash,
			
			LineFeed => EscapedLineFeed,
			
			_ => unreachable!(),
		}
	}
	
	#[inline(always)]
	unsafe fn write_escape_sequence(write_to: *mut u8, remaining_bytes: &[u8], end_pointer: *mut u8, character_to_escape_index: usize) -> (*mut u8, bool)
	{
		let do_not_write_the_escape_sequence_if_it_would_be_truncated_as_otherwise_we_could_not_distinguish_a_genuine_slash_from_the_start_of_an_escape_sequence = write_to.add(EscapeSequenceLength) > end_pointer;
		if do_not_write_the_escape_sequence_if_it_would_be_truncated_as_otherwise_we_could_not_distinguish_a_genuine_slash_from_the_start_of_an_escape_sequence
		{
			return (write_to, true)
		}
		(write_slice_unchecked(write_to, &choose_escape_sequence(character_to_escape_index, remaining_bytes), end_pointer), false)
	}
	
	let mut remaining_bytes = message.as_bytes();
	while !remaining_bytes.is_empty()
	{
		match memchr2(LineFeed, Slash, remaining_bytes)
		{
			None => return write_slice_truncated(write_to, remaining_bytes, end_pointer),
			
			Some(character_to_escape_index) =>
			{
				let (new_write_to, truncated) = write_slice_truncated(write_to, &remaining_bytes[0 .. character_to_escape_index], end_pointer);
				if truncated
				{
					return (new_write_to, true)
				}
				write_to = new_write_to;
				
				let (new_write_to, truncated) = write_escape_sequence(write_to, remaining_bytes, end_pointer, character_to_escape_index);
				if truncated
				{
					return (new_write_to, true)
				}
				write_to = new_write_to;
				
				remaining_bytes = &remaining_bytes[character_to_escape_index + 1 .. ];
			}
		}
	}
	(write_to, false)
}

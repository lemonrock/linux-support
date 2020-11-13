// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parse a set of ASCII strings each terminated by NUL, or, an empty (zero byte) value.
pub fn parse_ascii_nul_string_values<D: Default, F: Fn(&mut D, &[u8]) -> Result<(), &'static str>>(bytes: &[u8], add_ascii_string: &F) -> Result<D, &'static str>
{
	let length = bytes.len();

	let mut collection = D::default();
	if unlikely!(length == 0)
	{
		return Ok(collection)
	}

	const AsciiNul: u8 = b'\0';

	let final_byte_index = length - 1;
	let final_byte = bytes.get_unchecked_value_safe(final_byte_index);
	if unlikely!(final_byte != AsciiNul)
	{
		return Err("bytes must end with an Ascii NUL");
	}

	for ascii_string in (&bytes[0 .. final_byte_index]).split_bytes(AsciiNul)
	{
		add_ascii_string(&mut collection, ascii_string)?
	}
	Ok(collection)
}

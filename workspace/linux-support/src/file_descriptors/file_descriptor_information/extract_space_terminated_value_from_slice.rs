// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn extract_space_terminated_value_from_slice<'a, Value, Error>(remaining_bytes: &'a [u8], prefix_including_colon_and_whitespace: &'static [u8], parser: impl FnOnce(&str) -> Result<Value, Error>) -> io::Result<(Value, &'a [u8])>
{
	if unlikely!(remaining_bytes.len() < prefix_including_colon_and_whitespace.len())
	{
		return Err(invalid_data())
	}

	if unlikely!(&remaining_bytes[0 .. prefix_including_colon_and_whitespace.len()] != prefix_including_colon_and_whitespace)
	{
		return Err(invalid_data())
	}

	let remaining_bytes = &remaining_bytes[prefix_including_colon_and_whitespace.len() .. ];
	let raw_value = match remaining_bytes.memchr(b' ')
	{
		None => remaining_bytes,

		Some(index) => &remaining_bytes[0 .. index],
	};

	let raw_value_str = from_utf8(raw_value).map_err(|_utf8_error| invalid_data())?;
	let value = parser(raw_value_str).map_err(|_utf8_error| invalid_data())?;

	Ok((value, &remaining_bytes[raw_value.len() + 1 .. ]))
}

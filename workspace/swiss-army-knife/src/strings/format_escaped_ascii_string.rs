// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Writes an escaped ASCII string.
#[inline(always)]
pub fn format_escaped_ascii_string(bytes: &[u8], f: &mut Formatter) -> fmt::Result
{
	use fmt::Write;
	for byte in bytes.iter().flat_map(|&unescaped| escape_default(unescaped))
	{
		f.write_char(byte as char)?;
	}
	Ok(())
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A string that contains only ASCII alphanumerics, underscores, and periods.
///
/// Metric names are case sensitive.
///
/// * Must start with a letter;
/// * Must not exceed 200 bytes; fewer than 100 is preferred for metric names and 100 bytes is the maximum for service checks.
/// * Can not be empty.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Name(ArrayString<[u8; Self::Length]>);

impl Name
{
	const Length: usize = 100;
	
	/// New instance.
	pub fn new(name: &str) -> Result<Self, String>
	{
		let bytes = name.as_bytes();
		
		let length = bytes.len();
		
		if unlikely!(length == 0)
		{
			return Err("Empty".to_string())
		}
		
		if unlikely!(length > Self::Length)
		{
			return Err(format!("Length `{}` exceeds maximum of {} in `{}`", length, Self::Length, name))
		}
		
		match bytes.get_unchecked_value_safe(0)
		{
			b'A' ..= b'Z' => (),
			b'a' ..= b'z' => (),
			first_byte @ _ => return Err(format!("First byte can not be '0x{:02X}' in `{}`", first_byte, name))
		}
		
		let subsequent_bytes = &bytes[1 .. ];
		for subsequent_byte in subsequent_bytes
		{
			match subsequent_byte
			{
				b'0' ..= b'9' => (),
				b'A' ..= b'Z' => (),
				b'a' ..= b'z' => (),
				b'_' => (),
				b'.' => (),
				subsequent_byte @ _ => return Err(format!("Subsequent byte can not be '0x{:02X}' in `{}`", subsequent_byte, name))
			}
		}
		
		Ok(Self(ArrayString::from(name).expect("Length check occurs above")))
	}
	
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter) -> Result<(), ()>
	{
		dog_stats_d_writer.write_array_string(&self.0)
	}
}

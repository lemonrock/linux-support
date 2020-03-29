// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure that can be stored in a bit set.
pub trait BitSetAware: Sized + Into<u16> + TryFrom<u16, Error=BitSetAwareTryFromU16Error> + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + From<u8> + Into<u32> + Into<u64> + Into<usize> + Into<i32> + Into<i64> + Into<isize>
{
	#[doc(hidden)]
	const LinuxMaximum: u16;

	/// Minimum.
	const InclusiveMinimum: Self;

	/// Maixmum.
	const InclusiveMaximum: Self;

	#[doc(hidden)]
	const Prefix: &'static [u8] = b"";

	/// Converts item into set of item.
	#[inline(always)]
	fn into_bit_set(self) -> BitSet<Self>
	{
		let mut new = BitSet::empty();
		new.add(self);
		new
	}

	#[doc(hidden)]
	fn hydrate(value: u16) -> Self;

	#[doc(hidden)]
	#[inline(always)]
	fn parse_file_name(file_name: &OsStr) -> io::Result<Option<Self>>
	{
		use self::ErrorKind::Other;

		let file_name = file_name.as_bytes();
		if !file_name.starts_with(Self::Prefix)
		{
			return Ok(None)
		}

		let mut parsed_number: u16 = 0;
		for &byte in &file_name[Self::Prefix.len() .. ]
		{
			parsed_number = parsed_number * 10 + match byte
			{
				b'0' ..= b'9' => (byte - b'0') as u16,
				_ => return Err(io::Error::new(Other, format!("invalid byte 0x{:02X}", byte))),
			}
		}
		Ok(Some(Self::try_from(parsed_number).map_err(|error| io::Error::new(Other, error))?))
	}
}

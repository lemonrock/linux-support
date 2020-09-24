// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure that can be stored in a bit set.
pub trait BitSetAware: Sized + Into<u16> + TryFrom<u16, Error=BitSetAwareTryFromU16Error> + ParseNumber + FromBytes<Error=ParseNumberError> + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Into<u32> + Into<u64> + Into<usize> + Into<i32> + Into<i64> + Into<isize>
{
	/// Exclusive maximum (count) that Linux enforces or is compiled for.
	///
	/// Defined at `u32` to allow for `(u16::MAX + 1)`.
	/// (ie the maximum for `LinuxMaximum` is `65,536`.
	const LinuxMaximum: u32;

	/// Minimum.
	const InclusiveMinimum: Self;

	/// Maixmum.
	const InclusiveMaximum: Self;

	#[doc(hidden)]
	const Prefix: &'static [u8] = b"";

	/// Mostly exists to support `Signal`.
	#[doc(hidden)]
	const OneBasedCorrection: u16 = 0;

	/// Converts item into set of item.
	#[inline(always)]
	fn into_bit_set(self) -> BitSet<Self>
	{
		let mut new = BitSet::empty();
		new.add(self);
		new
	}

	#[doc(hidden)]
	#[inline(always)]
	fn dehydrate(self) -> u16
	{
		let value: u16 = self.into();
		value - Self::OneBasedCorrection
	}

	#[doc(hidden)]
	#[inline(always)]
	fn hydrate(value: u16) -> Self
	{
		Self::from_validated_u16(value + Self::OneBasedCorrection)
	}

	// `value` will have had any OneBasedCorrection already applied.
	#[doc(hidden)]
	fn from_validated_u16(value: u16) -> Self;

	#[doc(hidden)]
	#[inline(always)]
	fn parse_file_name(file_name: &OsStr) -> io::Result<Option<Self>>
	{
		let file_name = file_name.as_bytes();
		if !file_name.starts_with(Self::Prefix)
		{
			return Ok(None)
		}

		let bytes = &file_name[Self::Prefix.len() .. ];

		let parsed_number = u16::parse_decimal_number(bytes).map_err(io_error_other)?;

		Ok(Some(Self::try_from(parsed_number).map_err(io_error_other)?))
	}
}

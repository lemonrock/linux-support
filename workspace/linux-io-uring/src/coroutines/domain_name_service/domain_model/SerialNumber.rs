// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A serial number that can wrap-around and is often based on a date-time.
#[derive(Default, Debug, Copy, Clone, Hash)]
pub struct SerialNumber(BigEndianU32);

impl Into<u32> for SerialNumber
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.into_u32()
	}
}

impl<'a> Into<u32> for &'a SerialNumber
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.into_u32()
	}
}

impl PartialEq for SerialNumber
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.into_u32() == other.into_u32()
	}
}

impl PartialOrd for SerialNumber
{
	/// Defined using the logic in RFC 1982, Section 3.3.
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		use self::Ordering::*;

		let i1 = self.into_u32();
		let i2 = other.into_u32();

		let is_equal = i1 == i2;
		if unlikely!(is_equal)
		{
			return Some(Equal)
		}

		if likely!(Self::is_less_than(i1, i2))
		{
			return Some(Less)
		}

		if likely!(Self::is_greater_than(i1, i2))
		{
			return Some(Greater)
		}

		None
	}
}

impl SerialNumber
{
	/// Into `u32`.
	#[inline(always)]
	pub const fn into_u32(&self) -> u32
	{
		u32::from_be_bytes(self.0)
	}

	/// Difference.
	///
	/// Returns `(left, right, difference)` unless the difference is too large to work out wrap-around.
	#[inline(always)]
	pub fn difference(&self, other: &Self) -> Option<(u32, u32, i32)>
	{
		let i1 = self.into_u32();
		let i2 = other.into_u32();

		let is_equal = i1 == i2;
		if unlikely!(is_equal)
		{
			return Some((i1, i2, 0))
		}

		if likely!(Self::is_less_than(i1, i2))
		{
			return Some((i1, i2, (i2 - i1) as i32))
		}

		if likely!(Self::is_greater_than(i1, i2))
		{
			return Some((i1, i2, (i1 - i2) as i32))
		}

		None
	}
	
	// `2^(SERIAL_BITS - 1)` where `SERIAL_BITS` is `32`.
	const Maximum: u32 = i32::MAX as u32;
	
	#[inline(always)]
	fn is_less_than(i1: u32, i2: u32) -> bool
	{
		((i1 < i2) && (i2 - i1) < Self::Maximum) || ((i1 > i2) && (i1 - i2) > Self::Maximum)
	}

	#[inline(always)]
	fn is_greater_than(i1: u32, i2: u32) -> bool
	{
		((i1 < i2) && (i2 - i1) > Self::Maximum) || ((i1 > i2) && (i1 - i2) < Self::Maximum)
	}
}

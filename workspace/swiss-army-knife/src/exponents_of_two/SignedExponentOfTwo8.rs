// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A signed exponent (power of) 2.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct SignedExponentOfTwo8(i8);

impl SignedExponentOfTwo8
{
	/// Zero.
	pub const Zero: Self = Self(0);
	
	/// Into an i8.
	#[inline(always)]
	pub fn into_i8(self) -> Option<i8>
	{
		if self.is_out_range(6)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as i8))
		}
	}
	
	/// Into an u8.
	#[inline(always)]
	pub fn into_u8(self) -> Option<u8>
	{
		if self.is_out_range(7)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as u8))
		}
	}
	
	/// Into an i16.
	#[inline(always)]
	pub fn into_i16(self) -> Option<i16>
	{
		if self.is_out_range(14)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as i16))
		}
	}
	
	/// Into an u16.
	#[inline(always)]
	pub fn into_u16(self) -> Option<u16>
	{
		if self.is_out_range(15)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as u16))
		}
	}
	
	/// Into an i32.
	#[inline(always)]
	pub fn into_i32(self) -> Option<i32>
	{
		if self.is_out_range(30)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as i32))
		}
	}
	
	/// Into an u32.
	#[inline(always)]
	pub fn into_u32(self) -> Option<u32>
	{
		if self.is_out_range(31)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as u32))
		}
	}
	
	/// Into an i64.
	#[inline(always)]
	pub fn into_i64(self) -> Option<i64>
	{
		if self.is_out_range(62)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as i64))
		}
	}
	
	/// Into an u64.
	#[inline(always)]
	pub fn into_u64(self) -> Option<u64>
	{
		if self.is_out_range(63)
		{
			None
		}
		else
		{
			Some(1 << (self.0 as u64))
		}
	}
	
	#[inline(always)]
	const fn is_out_range(self, number_of_bits: i8) -> bool
	{
		self.0 < 0 || self.0 > number_of_bits
	}
}

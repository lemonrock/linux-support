// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait UsizePointer
{
	fn unsafe_cast<'a, To>(self) -> &'a To;

	fn unsafe_cast_mut<'a, To>(self) -> &'a mut To;

	fn unsafe_cast_slice<'a, To>(self, length: usize) -> &'a [To];

	fn unsafe_cast_slice_mut<'a, To>(self, length: usize) -> &'a mut [To];

	fn dereference_u8(self) -> u8;

	fn set_u8_byte(self, value: u8);

	fn set_u16_bytes(self, value: [u8; 2]);

	fn set_u32_bytes(self, value: [u8; 4]);
}

impl UsizePointer for usize
{
	#[inline(always)]
	fn unsafe_cast<'a, To>(self) -> &'a To
	{
		unsafe { & * (self as *const To) }
	}

	#[inline(always)]
	fn unsafe_cast_mut<'a, To>(self) -> &'a mut To
	{
		unsafe { &mut * (self as *mut To) }
	}

	#[inline(always)]
	fn unsafe_cast_slice<'a, To>(self, length: usize) -> &'a [To]
	{
		unsafe { from_raw_parts(self.unsafe_cast::<To>(), length) }
	}

	#[inline(always)]
	fn unsafe_cast_slice_mut<'a, To>(self, length: usize) -> &'a mut [To]
	{
		unsafe { from_raw_parts_mut(self.unsafe_cast_mut::<To>(), length) }
	}

	#[inline(always)]
	fn dereference_u8(self) -> u8
	{
		unsafe { * (self as *const u8) }
	}

	#[inline(always)]
	fn set_u8_byte(self, value: u8)
	{
		unsafe { * (self as *mut u8) = value }
	}

	#[inline(always)]
	fn set_u16_bytes(self, value: [u8; 2])
	{
		unsafe { * (self as *mut [u8; 2]) = value }
	}
	
	fn set_u32_bytes(self, value: [u8; 4])
	{
		unsafe { * (self as *mut [u8; 4]) = value }
	}
}

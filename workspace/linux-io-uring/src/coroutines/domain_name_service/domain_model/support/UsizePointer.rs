// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait UsizePointer
{
	fn unsafe_cast<'a, To>(self) -> &'a To;

	fn unsafe_cast_mut<'a, To>(self) -> &'a mut To;

	fn unsafe_cast_slice<'a, To>(self, length: usize) -> &'a [To];

	fn unsafe_cast_slice_mut<'a, To>(self, length: usize) -> &'a mut [To];

	fn dereference_u8(self) -> u8;

	fn set_u8(self, value: u8);

	fn set_u16(self, network_endian_value: u16);

	fn set_u16_network_endian(self, native_endian_value: u16);

	fn set_u16_bytes(self, value: [u8; 2]);

	fn set_u16_network_endian_from_usize(self, native_endian_value: usize);

	fn set_u32_bytes(self, value: [u8; 4]);

	fn set_u64(self, network_endian_value: u64);
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
	fn set_u8(self, value: u8)
	{
		unsafe { * (self as *mut u8) = value }
	}

	#[inline(always)]
	fn set_u16(self, network_endian_value: u16)
	{
		unsafe { * (self as *mut u16) = network_endian_value }
	}

	#[inline(always)]
	fn set_u16_network_endian(self, native_endian_value: u16)
	{
		self.set_u16(native_endian_value.to_be())
	}

	#[inline(always)]
	fn set_u16_bytes(self, value: [u8; 2])
	{
		unsafe { * (self as *mut [u8; 2]) = value }
	}

	#[inline(always)]
	fn set_u16_network_endian_from_usize(self, native_endian_value: usize)
	{
		unsafe { * (self as *mut u16) = (native_endian_value as u16).to_be() }
	}

	fn set_u32_bytes(self, value: [u8; 4])
	{
		unsafe { * (self as *mut [u8; 4]) = value }
	}

	#[inline(always)]
	fn set_u64(self, network_endian_value: u64)
	{
		unsafe { * (self as *mut u64) = network_endian_value }
	}
}

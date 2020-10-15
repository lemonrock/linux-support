// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait U8Slice<'a>: Sized
{
	#[inline(always)]
	fn start_pointer(self) -> usize
	{
		self.slice_().as_ptr() as usize
	}

	#[inline(always)]
	fn end_pointer(self) -> usize
	{
		self.start_pointer() + self.len_()
	}

	/// RFC 4034, Appendix B.
	#[inline(always)]
	fn key_tag(self) -> KeyTag
	{
		#[inline(always)]
		fn accumulate(data: &[u8], length: usize) -> u32
		{
			let mut accumulator: u32 = 0;

			for index in 0 .. length
			{
				let value = data.u16_as_u32(index);
				accumulator += value;
			}

			accumulator
		}

		let length = self.len_();
		let slice = self.slice_();

		let accumulator = if length % 2 == 0
		{
			accumulate(slice, length)
		}
		else
		{
			let last = length - 1;
			accumulate(slice, last) + self.u8_as_u32(last) << 8
		};

		let accumulator = accumulator + ((accumulator >> 16) & 0xFFFF);
		KeyTag((accumulator & 0xFFFF) as u16)
	}

	#[inline(always)]
	fn cast<T>(self, offset: usize) -> &'a T
	{
		unsafe { & * (self.get_::<T>(offset)) }
	}

	#[inline(always)]
	fn u8(self, offset: usize) -> u8
	{
		self.value::<u8>(offset)
	}

	#[inline(always)]
	fn u8_as_u32(self, offset: usize) -> u32
	{
		self.u8(offset) as u32
	}

	#[inline(always)]
	fn u8_as_usize(self, offset: usize) -> usize
	{
		self.u8(offset) as usize
	}

	#[inline(always)]
	fn u16(self, offset: usize) -> u16
	{
		self.value::<[u8; size_of::<u16>()]>(offset).from_network_endian_to_native_endian()
	}

	#[inline(always)]
	fn u16_as_u32(self, offset: usize) -> u32
	{
		self.u16(offset) as u32
	}

	#[inline(always)]
	fn u16_as_usize(self, offset: usize) -> usize
	{
		self.u16(offset) as usize
	}

	#[inline(always)]
	fn u32(self, offset: usize) -> u32
	{
		self.value::<[u8; size_of::<u32>()]>(offset).from_network_endian_to_native_endian()
	}

	#[inline(always)]
	fn u64(self, offset: usize) -> u64
	{
		self.value::<[u8; size_of::<u64>()]>(offset).from_network_endian_to_native_endian()
	}

	#[inline(always)]
	fn u16_network_endian(self, offset: usize) -> u16
	{
		self.value::<u16>(offset)
	}

	#[inline(always)]
	fn value<T: Copy>(self, offset: usize) -> T
	{
		unsafe { * self.get_::<T>(offset) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn get_<T>(self, offset: usize) -> *const T
	{
		(unsafe { self.slice_().get_unchecked(offset) }).unsafe_cast::<T>()
	}

	#[doc(hidden)]
	#[inline(always)]
	fn len_(self) -> usize
	{
		self.slice_().len()
	}

	#[doc(hidden)]
	fn slice_(self) -> &'a [u8];
}

impl<'a> U8Slice<'a> for &'a [u8]
{
	#[inline(always)]
	fn slice_(self) -> &'a [u8]
	{
		self
	}
}

impl<'a> U8Slice<'a> for &'a mut [u8]
{
	#[inline(always)]
	fn slice_(self) -> &'a [u8]
	{
		self
	}
}

impl<'a> U8Slice<'a> for &'a [u8; 2]
{
	#[inline(always)]
	fn slice_(self) -> &'a [u8]
	{
		&self[..]
	}
}

impl<'a> U8Slice<'a> for &'a mut [u8; 2]
{
	#[inline(always)]
	fn slice_(self) -> &'a [u8]
	{
		&self[..]
	}
}

impl<'a> U8Slice<'a> for &'a [u8; 4]
{
	#[inline(always)]
	fn slice_(self) -> &'a [u8]
	{
		&self[..]
	}
}

impl<'a> U8Slice<'a> for &'a mut [u8; 4]
{
	#[inline(always)]
	fn slice_(self) -> &'a [u8]
	{
		&self[..]
	}
}
